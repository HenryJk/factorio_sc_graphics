use std::path::Path;


use crate::casc::CascStorage;
use crate::sprite_config::{SpriteGroup, SpriteFormat};
use std::error::Error;
use crate::anim::Anim;
use std::fs::{create_dir_all, File, read_dir};
use crate::sprite_maker::{makeSprites, Preset, makeSpritesSd, FactorioSprites};
use crate::lua;
use crate::lua::LuaSyntax;
use std::io::Write;


const modname: &str = "sc-redux";

enum Resolution {
    HD,
    SD,
}

fn writeAnimation(
    group: &String,
    format: &SpriteFormat,
    sprites: &FactorioSprites,
    preset: &Preset,
    resolution: &Resolution,
    output_dir: &String,
) -> Result<lua::Exp, Box<dyn Error>> {
    let sprite_dir = Path::new(&format!("{}/graphics", output_dir))
        .join(match resolution {
            Resolution::HD => "hd",
            Resolution::SD => "sd"
        })
        .join(group);
    create_dir_all(&sprite_dir)?;
    let filename_field = if sprites.images.len() > 1 {
        (
            String::from("filenames"),
            lua::Exp::Array {
                member_list: (0..sprites.images.len())
                    .map(|i| {
                        let filename = match preset {
                            Preset::normal => format!("{}-{:02}.png", &format.name, i + 1),
                            Preset::mask => format!("{}-mask-{:02}.png", &format.name, i + 1),
                            Preset::light => format!("{}-light-{:02}.png", &format.name, i + 1),
                        };
                        sprites.images[i].save(sprite_dir.join(&filename)).unwrap();
                        lua::Exp::String(
                            match resolution {
                                Resolution::HD => format!("__{}__/graphics/hd/{}/{}", modname, group, &filename),
                                Resolution::SD => format!("__{}__/graphics/sd/{}/{}", modname, group, &filename),
                            }
                        )
                    })
                    .collect()
            }
        )
    } else {
        (
            String::from("filename"),
            {
                let filename = match preset {
                    Preset::normal => format!("{}.png", &format.name),
                    Preset::mask => format!("{}-mask.png", &format.name),
                    Preset::light => format!("{}-light.png", &format.name),
                };
                sprites.images[0].save(sprite_dir.join(&filename)).unwrap();
                lua::Exp::String(
                    match resolution {
                        Resolution::HD => format!("__{}__/graphics/hd/{}/{}", modname, group, &filename),
                        Resolution::SD => format!("__{}__/graphics/sd/{}/{}", modname, group, &filename),
                    }
                )
            }
        )
    };
    let mut result = lua::Exp::Table {
        field_list: vec![
            filename_field,
            (
                String::from("slice"),
                lua::Exp::Number(sprites.slice as f32)
            ),
            (
                String::from("line_length"),
                lua::Exp::Number(sprites.slice as f32)
            ),
            (
                String::from("lines_per_file"),
                lua::Exp::Number(sprites.lines_per_file as f32)
            ),
            (
                String::from("width"),
                lua::Exp::Number(sprites.width as f32)
            ),
            (
                String::from("height"),
                lua::Exp::Number(sprites.height as f32)
            ),
            (
                String::from("frame_count"),
                lua::Exp::Number(format.animation_length as f32)
            ),
            (
                String::from("direction_count"),
                lua::Exp::Number(format.direction_count as f32)
            ),
            (
                String::from("shift"),
                lua::Exp::Array {
                    member_list: vec![
                        lua::Exp::Number(format.final_offset.0),
                        lua::Exp::Number(format.final_offset.1),
                    ]
                }
            ),
            (
                String::from("animation_speed"),
                lua::Exp::Number(0.4)
            ),
            (
                String::from("draw_as_shadow"),
                lua::Exp::Bool(format.draw_as_shadow),
            ),
            (
                String::from("run_mode"),
                lua::Exp::String(format.run_mode.clone()),
            ),
            (
                String::from("scale"),
                if format.scalable {
                    match resolution {
                        Resolution::HD => lua::Exp::Var(String::from("0.5 * scale")),
                        Resolution::SD => lua::Exp::Var(String::from("scale")),
                    }
                } else {
                    match resolution {
                        Resolution::HD => lua::Exp::Var(String::from("0.5")),
                        Resolution::SD => lua::Exp::Var(String::from("1")),
                    }
                }
            ),
        ]
    };
    if let lua::Exp::Table { field_list } = &mut result {
        if let Some(frame_sequence) = &format.frame_sequence {
            field_list.push(
                (
                    String::from("frame_sequence"),
                    lua::Exp::Array {
                        member_list: frame_sequence
                            .into_iter()
                            .map(|x| lua::Exp::Number(*x as f32))
                            .collect()
                    }
                )
            )
        }
    }
    if let lua::Exp::Table { field_list } = &mut result {
        match preset {
            Preset::normal => {
                field_list.push(
                    (
                        String::from("draw_as_glow"),
                        lua::Exp::Bool(format.draw_as_glow),
                    )
                );
            }
            Preset::mask => {
                field_list.push(
                    (
                        String::from("flags"),
                        lua::Exp::Array {
                            member_list: vec![lua::Exp::String(String::from("mask"))]
                        },
                    )
                );
                field_list.push(
                    (
                        String::from("tint"),
                        lua::Exp::Var(String::from("tint")),
                    )
                );
            }
            Preset::light => {
                field_list.push(
                    (
                        String::from("draw_as_glow"),
                        lua::Exp::Bool(true),
                    )
                );
                field_list.push(
                    (
                        String::from("flags"),
                        lua::Exp::Array {
                            member_list: vec![lua::Exp::String(String::from("light"))]
                        },
                    )
                );
            }
        }
    }
    Ok(result)
}

pub fn writeAnimations(
    storage: &mut CascStorage,
    metadata: &Vec<SpriteGroup>,
    output_dir: &String,
) -> Result<(), Box<dyn Error>> {
    let mut return_table = Vec::new();

    for sprite_group in metadata {
        println!("Processing: {} ({})", sprite_group.source, sprite_group.category);
        if cfg!(debug_assertions) {
            let folder_name = format!("{}/graphics/hd/{}", output_dir, sprite_group.category);
            if !Path::new(&folder_name).exists() {
                create_dir_all(&folder_name)?;
            }
            if sprite_group.sprites.iter()
                .all(|sprite| read_dir(&folder_name)
                    .unwrap()
                    .any(|file| file.as_ref()
                        .unwrap()
                        .path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .starts_with(&sprite.name)
                    )
                ) {
                continue;
            }
        }
        let anim = Anim::fromFile(
            storage.openFile(&sprite_group.source)?
        )?;

        let presets = vec![Preset::normal, Preset::mask, Preset::light];
        for format in &sprite_group.sprites {
            for preset in &presets {
                let tmp = makeSprites(&anim, format, sprite_group.base_offset_x2, preset.clone())?;
                let hd_sprites;
                match tmp {
                    Some(sprites) => hd_sprites = sprites,
                    None => { continue; }
                }
                let lua_sprites = if format.split_anim {
                    lua::Exp::Array {
                        member_list: (0..format.direction_count)
                            .map(|i| {
                                let split_format = SpriteFormat {
                                    direction_count: 1,
                                    name: format!("{}-{:02}", format.name, i + 1),
                                    ..format.clone()
                                };
                                let split_hd_sprites = FactorioSprites {
                                    images: vec![hd_sprites.images[i as usize].clone()],
                                    ..hd_sprites.clone()
                                };
                                let lua_hr_sprites = writeAnimation(
                                    &sprite_group.category,
                                    &split_format,
                                    &split_hd_sprites,
                                    preset,
                                    &Resolution::HD,
                                    &output_dir,
                                ).unwrap();
                                let sd_sprites = makeSpritesSd(&split_hd_sprites, &split_format);
                                let mut lua_sprites = writeAnimation(
                                    &sprite_group.category,
                                    &split_format,
                                    &sd_sprites,
                                    preset,
                                    &Resolution::SD,
                                    &output_dir,
                                ).unwrap();
                                if let lua::Exp::Table { field_list } = &mut lua_sprites {
                                    field_list.push(
                                        (
                                            String::from("hr_version"),
                                            lua_hr_sprites
                                        )
                                    );
                                }
                                lua_sprites
                            })
                            .collect()
                    }
                } else {
                    let lua_hr_sprites = writeAnimation(
                        &sprite_group.category,
                        &format,
                        &hd_sprites,
                        preset,
                        &Resolution::HD,
                        &output_dir,
                    )?;
                    let sd_sprites = makeSpritesSd(&hd_sprites, format);
                    let mut lua_sprites = writeAnimation(
                        &sprite_group.category,
                        &format,
                        &sd_sprites,
                        preset,
                        &Resolution::SD,
                        &output_dir,
                    )?;
                    if let lua::Exp::Table { field_list } = &mut lua_sprites {
                        field_list.push(
                            (
                                String::from("hr_version"),
                                lua_hr_sprites
                            )
                        );
                    }
                    lua_sprites
                };
                let anim_name = match preset {
                    Preset::normal => format!("{}_{}", &sprite_group.category, format.name),
                    Preset::mask => format!("{}_{}_mask", &sprite_group.category, format.name),
                    Preset::light => format!("{}_{}_light", &sprite_group.category, format.name),
                };
                let anim_name = anim_name.replace("-", "_");
                let mut params = Vec::new();
                if format.scalable { params.push(String::from("scale")); }
                if let Preset::mask = preset { params.push(String::from("tint")); }
                return_table.push(
                    (
                        anim_name,
                        lua::Exp::Function {
                            par_list: params,
                            body: lua::Block {
                                stats: Vec::new(),
                                last_stat: Some(lua::LastStat::Return { exp_list: vec![lua_sprites] }),
                            },
                        }
                    )
                )
            }
        }
    }
    let mut file = File::create(format!("{}/anim.lua", output_dir))?;
    file.write_all(
        lua::LastStat::Return {
            exp_list: vec![
                lua::Exp::Table {
                    field_list: return_table
                }
            ]
        }.prettyPrint().as_ref()
    )?;
    Ok(())
}
