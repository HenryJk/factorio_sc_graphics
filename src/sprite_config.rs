pub struct SpriteFormat {
    pub name: String,
    pub extra_offset_x2: (i32, i32),
    pub radial_offset_x2: (f32, f32),
    pub final_offset: (f32, f32),
    pub direction_count: i32,
    pub animation_length: i32,
    pub empty_pad: i32,
    pub start_index: i32,
    pub end_index: i32,
    pub draw_as_glow: bool,
}

pub struct SpriteGroup {
    pub source: String,
    pub category: String,
    pub base_offset_x2: (i32, i32),
    pub sprites: Vec<SpriteFormat>,
}

pub const DEFAULT_SPRITE_FORMAT: SpriteFormat = SpriteFormat {
    name: String::new(),
    extra_offset_x2: (0, 0),
    radial_offset_x2: (0.0, 0.0),
    final_offset: (0.0, 0.0),
    direction_count: 0,
    animation_length: 0,
    empty_pad: 0,
    start_index: 0,
    end_index: 0,
    draw_as_glow: false,
};

pub fn getConfig() -> Vec<SpriteGroup> {
    vec![
        SpriteGroup {
            source: String::from("anim/main_151.anim"),
            category: String::from("zealot"),
            base_offset_x2: (12, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("attack"),
                    direction_count: 32,
                    animation_length: 5,
                    start_index: 0,
                    end_index: 85,
                    ..DEFAULT_SPRITE_FORMAT
                }
            ]

        }
    ]
}

// pub const CONFIG: [SpriteGroup; 1] = [
//     SpriteGroup {
//         source: "151",
//         name: "zealot",
//         sprite_formats: *[
//             SpriteFormat {
//                 name: "attack",
//                 direction_count: 32,
//                 animation_length: 5,
//                 start_index: 0,
//                 end_index: 5 * 17,
//                 ..DEFAULT_SPRITE_FORMAT
//             }
//         ],
//     }
// ];