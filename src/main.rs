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
mod lua;
mod factorio_anim_writer;

use std::error::Error;
use std::env::args;

fn main() -> Result<(), Box<dyn Error>> {
    let mut storage = casc::CascStorage::open("/home/henryj/Games/battlenet/drive_c/Program Files (x86)/StarCraft/")?;
    let argv: Vec<String> = args().collect();
    let working_dir = String::from(".");
    let output_dir = if argv.len() > 1 {&argv[1]} else {&working_dir};
    factorio_anim_writer::writeAnimations(&mut storage,&sprite_config::getConfig(), output_dir)?;
    storage.close()?;

    Ok(())
}
