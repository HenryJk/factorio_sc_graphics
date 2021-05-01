use std::path::Path;


use crate::casc::{CascStorage, CascFile};
use crate::sprite_config::{SpriteGroup, SpriteFormat};
use crate::lua_builder::LuaExp;
use std::error::Error;
use crate::anim::Anim;
use std::fs::create_dir_all;
use crate::sprite_maker::makeSprites;

pub fn writeAnimations(
    storage: &mut CascStorage,
    metadata: &Vec<SpriteGroup>
) -> Result<(), Box<dyn Error>> {
    let mut base = LuaExp::Table(Vec::new());
    let table = match &mut base {
        LuaExp::Table(internal) => internal,
        _ => {},
    };

    for sprite_group in metadata {
        let anim = Anim::fromFile(
            storage.openFile(&sprite_group.source)?
        )?;
        let hd_output_path = Path::new(".")
            .join("hd")
            .join(&sprite_group.category);
        create_dir_all(&hd_output_path)?;
        let sd_output_path = Path::new(".")
            .join("sd")
            .join(&sprite_group.category);
        create_dir_all(&sd_output_path)?;
        for format in &sprite_group.sprites {
            let hd_sprites = makeSprites(&anim, format, sprite_group.base_offset_x2, )?;
        }

    }
    Ok(())
}