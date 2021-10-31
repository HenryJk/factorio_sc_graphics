use crate::anim::Anim;
use crate::sprite_config::SpriteFormat;
use image::{RgbaImage, GenericImageView, GenericImage, Pixel};
use std::f32::consts::PI;
use image::imageops::{overlay, crop_imm, resize, FilterType};
use std::error::Error;

#[derive(Copy, Clone)]
pub enum Preset {
    normal,
    mask,
    light,
}

#[derive(Clone)]
pub struct FactorioSprites {
    pub images: Vec<RgbaImage>,
    pub slice: i32,
    pub lines_per_file: i32,
    pub width: i32,
    pub height: i32,
}

pub fn makeSprites(
    anim: &Anim,
    format: &SpriteFormat,
    base_offset_x2: (i32, i32),
    preset: Preset,
) -> Result<Option<FactorioSprites>, Box<dyn Error>> {
    let mut frames = Vec::with_capacity((format.source_range_index.1 - format.source_range_index.0) as usize);
    let source_direction_count = (format.source_range_index.1 - format.source_range_index.0) / format.animation_length;
    for i in format.source_range_index.0..format.source_range_index.1 {
        if let Some(frame) = anim.getFrame(i as usize)? { frames.push(frame) };
    }
    let used_directions = if let Some(directions) = &format.used_directions {
        if *(&directions.len()) != *(&format.direction_count) as usize {
            panic!("format.direction_count and format.used_directions.len() does not match!")
        }
        directions.clone()
    } else {
        let step = 2 * (source_direction_count - 1) / &format.direction_count;
        (0..format.direction_count).map(|x| x * step).collect()
    };
    let mut frame_width = 0;
    let mut frame_height = 0;
    for i in 0..(format.source_range_index.1 - format.source_range_index.0) {
        let frame = &mut frames[i as usize];
        let mut theta = (i % source_direction_count) as f32;
        if source_direction_count > 1 {
            theta *= PI / (source_direction_count - 1) as f32;
        }
        let radial_offset_x2 = (
            (format.radial_offset_x2.0 * theta.sin()).round() as i32,
            (format.radial_offset_x2.1 * theta.cos()).round() as i32,
        );
        frame.center_x2.0 += base_offset_x2.0 + radial_offset_x2.0 + format.extra_offset_x2.0;
        frame.center_x2.1 += base_offset_x2.1 + radial_offset_x2.1 + format.extra_offset_x2.1;
        frame_width = frame_width
            .max(frame.center_x2.0)
            .max(2 * frame.width - frame.center_x2.0);
        frame_height = frame_height
            .max(frame.center_x2.1)
            .max(2 * frame.height - frame.center_x2.1);
    }
    let col_count = 1.max(4096 / frame_width);
    let row_count = 1.max(4096 / frame_height);
    let frame_per_img = if format.split_anim { format.animation_length } else { col_count * row_count };
    let image_count = (
        used_directions.len() as i32
            * format.animation_length
            + format.empty_pad
            + frame_per_img
            - 1
    ) / frame_per_img;

    let mut output: Vec<RgbaImage> = vec![
        RgbaImage::new(
            (col_count * frame_width) as u32,
            (row_count * frame_height) as u32,
        );
        image_count as usize
    ];

    for ori in 0..used_directions.len() as i32 {
        for anim_idx in 0..format.animation_length {
            let out_idx = ori * format.animation_length + anim_idx + format.empty_pad;
            let img_idx = out_idx / frame_per_img;
            let row_idx = (out_idx % frame_per_img as i32) / col_count;
            let col_idx = (out_idx % frame_per_img) % col_count as i32;
            let mut source_ori = used_directions[ori as usize];
            let mirrored = source_ori >= source_direction_count;
            if mirrored { source_ori = 2 * (source_direction_count - 1) - source_ori; }
            let source_idx = anim_idx * source_direction_count + source_ori;
            let frame = &frames[source_idx as usize];
            let start_x = if mirrored {
                col_idx * frame_width + (frame_width - 2 * frame.width + frame.center_x2.0) / 2
            } else {
                col_idx * frame_width + (frame_width - frame.center_x2.0) / 2
            } as u32;
            let start_y = (row_idx * frame_height + (frame_height - frame.center_x2.1) / 2) as u32;

            let diffuse = match &frame.diffuse {
                Some(img) => img,
                None => return Ok(None),
            };
            for x in 0..frame.width as u32 {
                for y in 0..frame.height as u32 {
                    unsafe {
                        let source_x = if mirrored { frame.width as u32 - x - 1 } else { x };
                        match preset {
                            Preset::normal => {
                                let p = diffuse.unsafe_get_pixel(source_x, y);
                                output[img_idx as usize]
                                    .unsafe_put_pixel(start_x + x, start_y + y, p);
                            }
                            Preset::mask => {
                                let teamcolor = match &frame.teamcolor {
                                    Some(img) => img,
                                    None => return Ok(None),
                                };
                                let masked = teamcolor
                                    .unsafe_get_pixel(source_x, y)
                                    .channels()[0] > 0;
                                if !masked { continue; }
                                let p = diffuse.unsafe_get_pixel(source_x, y);
                                output[img_idx as usize]
                                    .unsafe_put_pixel(start_x + x, start_y + y, p);
                            }
                            Preset::light => {
                                let emissive = match &frame.emissive {
                                    Some(img) => img,
                                    None => return Ok(None),
                                };
                                let mut p = emissive
                                    .unsafe_get_pixel(source_x, y)
                                    .to_rgba();
                                let channels = p.channels_mut();
                                channels[3] = channels[0]
                                    .max(channels[1])
                                    .max(channels[2]);
                                output[img_idx as usize]
                                    .unsafe_put_pixel(start_x + x, start_y + y, p);
                            }
                        }
                    }
                }
            }
            if cfg!(debug_assertions) {
                unsafe {
                    let green_pixel = image::Rgba([0, 255, 0, 255]);
                    let x_center1 = (frame_width - 1) / 2;
                    let x_center2 = frame_width / 2;
                    let y_center1 = (frame_height - 1) / 2;
                    let y_center2 = frame_height / 2;
                    let corner_x = (frame_width * col_idx) as u32;
                    let corner_y = (frame_height * row_idx) as u32;

                    for y in 0..frame_height {
                        let draw;
                        if y <= y_center1 {
                            draw = ((y_center1 - y) % 8) >= 4;
                        } else {
                            draw = ((y - y_center2) % 8) >= 4;
                        }
                        if draw {
                            output[img_idx as usize]
                                .unsafe_put_pixel(
                                    corner_x + x_center1 as u32,
                                    corner_y + y as u32,
                                    green_pixel,
                                );
                            output[img_idx as usize]
                                .unsafe_put_pixel(
                                    corner_x + x_center2 as u32,
                                    corner_y + y as u32,
                                    green_pixel,
                                );
                        }
                    }
                    for x in 0..frame_width {
                        let draw;
                        if x <= x_center1 {
                            draw = ((x_center1 - x) % 8) >= 4;
                        } else {
                            draw = ((x - x_center2) % 8) >= 4;
                        }
                        if draw {
                            output[img_idx as usize]
                                .unsafe_put_pixel(
                                    corner_x + x as u32,
                                    corner_y + y_center1 as u32,
                                    green_pixel,
                                );
                            output[img_idx as usize]
                                .unsafe_put_pixel(
                                    corner_x + x as u32,
                                    corner_y + y_center2 as u32,
                                    green_pixel,
                                );
                        }
                    }
                }
            }
        }
    }
    Ok(Some(FactorioSprites {
        images: output,
        slice: col_count as i32,
        lines_per_file: row_count as i32,
        width: frame_width as i32,
        height: frame_height as i32,
    }))
}

pub fn makeSpritesSd(
    hd_sprites: &FactorioSprites,
    format: &SpriteFormat,
) -> FactorioSprites {
    let sd_width = hd_sprites.width / 2;
    let sd_height = hd_sprites.height / 2;
    let col_count = 1.max(2048 / sd_width);
    let row_count = 1.max(2048 / sd_height);
    let frame_per_img = if format.split_anim { format.animation_length } else { col_count * row_count };
    let image_count = (
        format.direction_count
            * format.animation_length
            + format.empty_pad
            + frame_per_img
            - 1
    ) / frame_per_img;
    let mut output: Vec<RgbaImage> = vec![
        RgbaImage::new(
            (sd_width * col_count) as u32,
            (sd_height * row_count) as u32,
        );
        image_count as usize
    ];
    for idx in 0..(format.direction_count * format.animation_length + format.empty_pad) {
        let source_img_idx = idx / (hd_sprites.lines_per_file * hd_sprites.slice);
        let source_row_idx = (idx / hd_sprites.slice) % hd_sprites.lines_per_file;
        let source_col_idx = idx % hd_sprites.slice;

        let out_img_idx = idx / (col_count * row_count);
        let out_row_idx = (idx / col_count) % row_count;
        let out_col_idx = idx % col_count;

        let source = crop_imm(
            &hd_sprites.images[source_img_idx as usize],
            (source_col_idx * hd_sprites.width) as u32,
            (source_row_idx * hd_sprites.height) as u32,
            hd_sprites.width as u32,
            hd_sprites.height as u32,
        );
        overlay(
            &mut output[out_img_idx as usize],
            &resize(
                &source,
                sd_width as u32,
                sd_height as u32,
                FilterType::Lanczos3,
            ),
            (out_col_idx * sd_width) as u32,
            (out_row_idx * sd_height) as u32,
        );
    }
    FactorioSprites {
        images: output,
        slice: col_count as i32,
        lines_per_file: row_count as i32,
        width: sd_width,
        height: sd_height,
    }
}
