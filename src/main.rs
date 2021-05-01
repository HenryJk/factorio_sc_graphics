#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// mod reference;
mod casc;
mod anim;
mod sprite_maker;
mod sprite_config;
mod lua_builder;
mod factorio_anim_writer;

use std::error::Error;
use std::ffi::CString;
use sprite_config::{SpriteFormat, DEFAULT_SPRITE_FORMAT};
use crate::sprite_maker::Preset;
use lua_builder::LuaExp;

fn main() -> Result<(), Box<dyn Error>> {
    //let storage_path = CString::new("/home/henryj/Games/battlenet/drive_c/Program Files (x86)/StarCraft/")?;
    let mut storage = casc::CascStorage::open("/home/henryj/Games/battlenet/drive_c/Program Files (x86)/StarCraft/")?;
    let mut fp = storage.openFile("anim/main_151.anim")?;
    let anim = anim::Anim::fromFile(fp)?;
    let config = SpriteFormat {
        name: String::from("attack"),
        direction_count: 32,
        animation_length: 5,
        start_index: 0,
        end_index: 5 * 17,
        extra_offset_x2: (12, 0),
        ..DEFAULT_SPRITE_FORMAT
    };
    let result = match sprite_maker::makeSprites(
        &anim,
        &config,
        (0, 0),
        Preset::normal
    )? {
        Some(res) => res,
        None => return Ok(()),
    };

    let result = sprite_maker::makeSpritesSd(&result, &config);
    println!("{}", result.images.len());
    for i in 0..3 {
        &result.images[i as usize].save(format!("{}.png", i));
    }
    storage.close()?;

    // let mut res = LuaExp::Array(Vec::new());
    // match &mut res {
    //     LuaExp::Array(arr) => {
    //         arr.push(LuaExp::Nil);
    //     },
    //     _ => {}
    // }
    // println!("{}", res.build(0));

    Ok(())
}
