#[derive(Clone)]
pub struct SpriteFormat {
    pub name: String,
    pub extra_offset_x2: (i32, i32),
    pub radial_offset_x2: (f32, f32),
    pub final_offset: (f32, f32),
    pub direction_count: i32,
    pub animation_length: i32,
    pub empty_pad: i32,
    pub source_range_index: (i32, i32),
    pub draw_as_shadow: bool,
    pub draw_as_glow: bool,
    pub scalable: bool,
    pub run_mode: String,
    pub frame_sequence: Option<Vec<u32>>,
}

pub struct SpriteGroup {
    pub source: String,
    pub category: String,
    pub base_offset_x2: (i32, i32),
    pub sprites: Vec<SpriteFormat>,
}

pub fn getConfig() -> Vec<SpriteGroup> {
    let DEFAULT_SPRITE_FORMAT = SpriteFormat {
        name: String::new(),
        extra_offset_x2: (0, 0),
        radial_offset_x2: (0.0, 0.0),
        final_offset: (0.0, 0.0),
        direction_count: 0,
        animation_length: 0,
        empty_pad: 0,
        source_range_index: (0, 0),
        draw_as_shadow: false,
        draw_as_glow: false,
        scalable: false,
        run_mode: String::from("forward"),
        frame_sequence: None,
    };
    vec![
        SpriteGroup {
            source: String::from("anim/main_112.anim"),
            category: String::from("carrier"),
            base_offset_x2: (-3, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("run"),
                    direction_count: 32,
                    animation_length: 1,
                    source_range_index: (0, 17),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_113.anim"),
            category: String::from("carrier"),
            base_offset_x2: (-3, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("run-shadow"),
                    direction_count: 32,
                    animation_length: 1,
                    source_range_index: (0, 17),
                    draw_as_shadow: true,
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_114.anim"),
            category: String::from("carrier"),
            base_offset_x2: (-3, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("exhaust"),
                    direction_count: 32,
                    animation_length: 1,
                    source_range_index: (0, 17),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_116.anim"),
            category: String::from("interceptor"),
            base_offset_x2: (-4, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("run"),
                    direction_count: 32,
                    animation_length: 1,
                    source_range_index: (0, 17),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("attack"),
                    direction_count: 32,
                    animation_length: 1,
                    source_range_index: (17, 34),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_122.anim"),
            category: String::from("dragoon"),
            base_offset_x2: (-4, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("idle"),
                    direction_count: 1,
                    animation_length: 8,
                    source_range_index: (0, 136),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("run"),
                    direction_count: 4,
                    animation_length: 8,
                    source_range_index: (136, 272),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("attack"),
                    direction_count: 1,
                    animation_length: 8,
                    source_range_index: (272, 408),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("die"),
                    direction_count: 1,
                    animation_length: 7,
                    source_range_index: (408, 415),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_123.anim"),
            category: String::from("dragoon"),
            base_offset_x2: (-8, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("idle-shadow"),
                    direction_count: 1,
                    animation_length: 8,
                    source_range_index: (0, 136),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("run-shadow"),
                    direction_count: 4,
                    animation_length: 8,
                    source_range_index: (136, 272),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("attack-shadow"),
                    direction_count: 1,
                    animation_length: 8,
                    source_range_index: (272, 408),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_124.anim"),
            category: String::from("dragoon"),
            base_offset_x2: (-8, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("corpse"),
                    direction_count: 1,
                    animation_length: 5,
                    source_range_index: (0, 5),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_151.anim"),
            category: String::from("zealot"),
            base_offset_x2: (14, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("attack"),
                    direction_count: 32,
                    animation_length: 5,
                    source_range_index: (0, 85),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("run"),
                    direction_count: 32,
                    animation_length: 8,
                    source_range_index: (85, 221),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("die"),
                    direction_count: 1,
                    animation_length: 7,
                    source_range_index: (221, 228),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_152.anim"),
            category: String::from("zealot"),
            base_offset_x2: (14, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("attack-shadow"),
                    direction_count: 32,
                    animation_length: 5,
                    source_range_index: (0, 85),
                    draw_as_shadow: true,
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("run-shadow"),
                    direction_count: 32,
                    animation_length: 8,
                    source_range_index: (85, 221),
                    draw_as_shadow: true,
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_424.anim"),
            category: String::from("effects"),
            base_offset_x2: (0, 69),
            sprites: vec![
                SpriteFormat {
                    name: String::from("shield"),
                    direction_count: 8,
                    animation_length: 4,
                    source_range_index: (0, 68),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_427.anim"),
            category: String::from("effects"),
            base_offset_x2: (0, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("explosion-medium"),
                    direction_count: 1,
                    animation_length: 14,
                    source_range_index: (0, 14),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_523.anim"),
            category: String::from("projectiles"),
            base_offset_x2: (0, 0),
            sprites: vec![
                SpriteFormat {
                    name: String::from("phase-disruptor"),
                    direction_count: 1,
                    animation_length: 5,
                    source_range_index: (0, 5),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_547.anim"),
            category: String::from("archon"),
            base_offset_x2: (-16, 44),
            sprites: vec![
                SpriteFormat {
                    name: String::from("psionic-shockwave"),
                    direction_count: 1,
                    animation_length: 6,
                    source_range_index: (0, 6),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
        SpriteGroup {
            source: String::from("anim/main_549.anim"),
            category: String::from("archon"),
            base_offset_x2: (-18, 42),
            sprites: vec![
                SpriteFormat {
                    name: String::from("lightning-long"),
                    direction_count: 32,
                    animation_length: 2,
                    source_range_index: (0, 34),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
                SpriteFormat {
                    name: String::from("lightning-short"),
                    direction_count: 32,
                    animation_length: 2,
                    source_range_index: (34, 68),
                    ..DEFAULT_SPRITE_FORMAT.clone()
                },
            ],
        },
    ]
}
