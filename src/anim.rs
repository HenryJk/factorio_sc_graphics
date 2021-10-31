use std::io::{Read, Seek, SeekFrom};
use std::error::Error;
use byteorder::{ReadBytesExt, LE};
use image::{DynamicImage, SubImage};
use image::codecs::dds::DdsDecoder;
use image::imageops::crop_imm;
use std::collections::HashMap;

pub struct Anim {
    layers: HashMap<String, DynamicImage>,
    frames: Vec<FrameInfo>,
    width: u16,
    height: u16,
}

pub struct LayerInfo {
    name: String,
    offset: u32,
    size: u32,
    width: u16,
    height: u16,
}

#[derive(Debug)]
struct FrameInfo {
    pub tex_x: u16,
    pub tex_y: u16,
    pub x_off: i16,
    pub y_off: i16,
    pub width: u16,
    pub height: u16,
    pub unknown: u32,
}

pub struct Frame<'a> {
    pub center_x2: (i32, i32),
    pub width: i32,
    pub height: i32,
    pub diffuse: Option<SubImage<&'a DynamicImage>>,
    pub bright: Option<SubImage<&'a DynamicImage>>,
    pub teamcolor: Option<SubImage<&'a DynamicImage>>,
    pub emissive: Option<SubImage<&'a DynamicImage>>,
    pub normal: Option<SubImage<&'a DynamicImage>>,
    pub specular: Option<SubImage<&'a DynamicImage>>,
    pub ao_depth: Option<SubImage<&'a DynamicImage>>,
}

const ANIM_MAGIC: u32 = 0x4d494e41;

impl Anim {
    pub fn fromFile<R: Read + Seek + Copy>(mut fp: R) -> Result<Anim, Box<dyn Error>> {
        let anim = fp.read_u32::<LE>()?;
        assert_eq!(anim, ANIM_MAGIC);
        let _scale = fp.read_u8()?;
        let _ty = fp.read_u8()?;
        let _unknown = fp.read_u16::<LE>()?;
        let layer_count = fp.read_u16::<LE>()?;
        let _entry_count = fp.read_u16::<LE>()?;
        let mut layer_names = Vec::with_capacity(layer_count as usize);
        for _ in 0..layer_count {
            let mut buf = [0u8; 0x20];
            fp.read_exact(&mut buf)?;
            let end = buf.iter().position(|x| *x == 0).unwrap_or(buf.len());
            layer_names.push(String::from_utf8_lossy(&buf[..end]).into_owned());
        }

        fp.seek(SeekFrom::Start(0x14c))?;
        let frame_count = fp.read_u16::<LE>()?;
        let _ref_id = fp.read_u16::<LE>()?;
        let width = fp.read_u16::<LE>()?;
        let height = fp.read_u16::<LE>()?;
        let _frame_info_offset = fp.read_u32::<LE>()?;
        let mut layer_infos = Vec::with_capacity(layer_count as usize);

        for i in 0..layer_count {
            layer_infos.push(LayerInfo {
                name: layer_names[i as usize].clone(),
                offset: fp.read_u32::<LE>()?,
                size: fp.read_u32::<LE>()?,
                width: fp.read_u16::<LE>()?,
                height: fp.read_u16::<LE>()?,
            });
        }

        let mut layers = HashMap::with_capacity(layer_count as usize);
        for layer_info in &layer_infos {
            if layer_info.size == 0 { continue; }
            let stream = fp.by_ref().take(layer_info.size.into());
            let dds = DdsDecoder::new(stream)?;
            let img = DynamicImage::from_decoder(dds)?;
            // println!("{:?}", img.color());
            layers.insert(layer_info.name.clone(), img);
        }

        let mut frames = Vec::with_capacity(frame_count as usize);
        for _ in 0..frame_count {
            frames.push(FrameInfo {
                tex_x: fp.read_u16::<LE>()?,
                tex_y: fp.read_u16::<LE>()?,
                x_off: fp.read_i16::<LE>()?,
                y_off: fp.read_i16::<LE>()?,
                width: fp.read_u16::<LE>()?,
                height: fp.read_u16::<LE>()?,
                unknown: fp.read_u32::<LE>()?,
            });
        }
        Ok(Anim { layers, frames, width, height })
    }

    pub fn getFrame(&self, idx: usize) -> Result<Option<Frame>, Box<dyn Error>> {
        let frame_info = match self.frames.get(idx) {
            Some(info) => info,
            None => return Ok(None),
        };
        let center_x2 = (
            self.width as i32 - 2 * frame_info.x_off as i32,
            self.height as i32 - 2 * frame_info.y_off as i32,
        );
        let mut frame = Frame {
            center_x2,
            width: frame_info.width as i32,
            height: frame_info.height as i32,
            diffuse: None,
            bright: None,
            teamcolor: None,
            emissive: None,
            normal: None,
            specular: None,
            ao_depth: None,
        };
        for (layer_name, img) in self.layers.iter() {
            let sub_img = crop_imm(
                img,
                frame_info.tex_x as u32,
                frame_info.tex_y as u32,
                frame_info.width as u32,
                frame_info.height as u32,
            );
            match &layer_name[..] {
                "diffuse" => frame.diffuse = Some(sub_img),
                "bright" => frame.bright = Some(sub_img),
                "teamcolor" => frame.teamcolor = Some(sub_img),
                "emissive" => frame.emissive = Some(sub_img),
                "normal" => frame.normal = Some(sub_img),
                "specular" => frame.specular = Some(sub_img),
                "ao_depth" => frame.ao_depth = Some(sub_img),
                _ => {},
            }
        }
        Ok(Some(frame))
    }
}
