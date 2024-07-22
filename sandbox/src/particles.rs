


use crate::config;
use crate::god;
use crate::utils;
use crate::vector;



use macroquad::prelude::*;



use config::FALLING_START;
use god::God;
use utils::{mag_rand, chance_100, chance_1000};
use vector::Vector;



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParticleType {
    None      = 0,
    Bound     = 1,
    Sand      = 2,
    Water     = 3,
    Stone     = 4,
    Coal      = 5,
    Fire      = 6,
    Oil       = 7,
    Wood      = 8, 
    Gravel    = 9,
    Gasoline  = 10,
    SlimeMold = 11,
    Smoke     = 12, 
    Acid      = 13, 
    Lava      = 14,
    Steam     = 15,
    Wick      = 16,
    Glass     = 17,
    Cloner    = 18,
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color:         Color,
    pub awake:         bool,
    pub velocity:      Vector<f32>,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Particle {
            color:    particle_type.get_color(),
            awake:    true,
            velocity: Vector { x: 0.0, y: FALLING_START },
            particle_type,
        }
    }

    pub fn update(self, god: &mut God) {
        match self.particle_type {
            ParticleType::Sand      => ParticleType::update_sand(self, god),
            ParticleType::Water     => ParticleType::update_water(self, god),
            ParticleType::Coal      => ParticleType::update_coal(self, god),
            ParticleType::Fire      => ParticleType::update_fire(self, god),
            ParticleType::Oil       => ParticleType::update_oil(self, god),
            ParticleType::Gravel    => ParticleType::update_gravel(self, god), 
            ParticleType::Gasoline  => ParticleType::update_gasoline(self, god),
            ParticleType::SlimeMold => ParticleType::update_slimemold(self, god),
            ParticleType::Smoke     => ParticleType::update_smoke(self, god),
            ParticleType::Acid      => ParticleType::update_acid(self, god),
            ParticleType::Lava      => ParticleType::update_lava(self, god),
            ParticleType::Steam     => ParticleType::update_steam(self, god),
            ParticleType::Cloner    => ParticleType::update_cloner(self, god),
            _                       => {},
        }
    }

    pub fn make_smoke() -> Self {
        Self::new(ParticleType::Smoke)
    }

    pub fn make_fire() -> Self {
        Self::new(ParticleType::Fire)
    }

    pub fn make_steam() -> Self {
        Self::new(ParticleType::Steam)
    }

    pub fn make_water() -> Self {
        Self::new(ParticleType::Water)
    }
}

impl ParticleType {
    pub fn get_types() -> usize {
        18+1
    }

    pub fn get_from_int(int: usize) -> Self {
        match int {
            0  => Self::None,
            1  => Self::Bound,
            2  => Self::Sand,
            3  => Self::Water,
            4  => Self::Stone,
            5  => Self::Coal,
            6  => Self::Fire,
            7  => Self::Oil,
            8  => Self::Wood,
            9  => Self::Gravel,
            10 => Self::Gasoline,
            11 => Self::SlimeMold,
            12 => Self::Smoke,
            13 => Self::Acid,
            14 => Self::Lava,
            15 => Self::Steam,
            16 => Self::Wick,
            17 => Self::Glass,
            18 => Self::Cloner,
            _  => Self::None,
        }
    }
    
    pub fn get_color(&self) -> Color {
        let colors: Vec<Color> = self.base_colors();
        let idx: usize = mag_rand(colors.len()-1) as usize;
        colors[idx]
    }

    pub fn spawns_fail(&self) -> bool {
        let affinity: isize = match self {
            Self::Sand      => 99,
            Self::Water     => 95,
            Self::Coal      => 90,
            Self::Fire      => 95,
            Self::Oil       => 50,
            Self::Gravel    => 99,
            Self::SlimeMold => 99,
            Self::Smoke     => 90,
            Self::Acid      => 99,
            Self::Lava      => 95,
            Self::Steam     => 90,
            Self::Cloner    => 95,
            _               => return false,
        };
        chance_100(affinity)
    }

    pub fn is_none(&self) -> bool {
        self == &ParticleType::None
    }

    pub fn is_liquid(&self) -> bool {
        match self {
            Self::None     => false,
            Self::Water    => true,
            Self::Oil      => true,
            Self::Gasoline => true,
            Self::Acid     => true,
            _              => false,
        }
    }
    
    pub fn is_gas(&self) -> bool {
        match self {
            Self::None  => false,
            Self::Fire  => true,
            Self::Smoke => true,
            Self::Steam => true,
            _           => false, 
        }
    }

    pub fn spread_factor(&self) -> usize {
        match self {
            Self::Water    => 100,
            Self::Oil      => 3,
            Self::Gasoline => 150,
            Self::Acid     => 30,
            _              => 0,
        }
    }

    pub fn enflames(&self) -> bool {
        let flamability: isize = match self {
            Self::Coal      => 5,
            Self::Oil       => 10,
            Self::Wood      => 2,
            Self::Gasoline  => 75,
            Self::SlimeMold => return true,
            Self::Acid      => 1,
            Self::Wick      => return true,
            _               => return false,
        };
        chance_100(flamability)
    }

    pub fn corrodes(&self) -> bool {
        let corrosion: isize = match self {
            Self::Sand   => 2,
            Self::Water  => 2,
            Self::Stone  => 1,
            Self::Coal   => 1,
            Self::Wood   => 5,
            Self::Gravel => 1,
            Self::Smoke  => return true,
            Self::Steam  => return true,
            Self::Wick   => 2,
            _            => return false,
        };
        chance_100(corrosion)
    }

    pub fn evaporates(&self) -> bool {
        let steam: isize = match self {
            Self::Water     => 30,
            Self::SlimeMold => 5,
            Self::Acid      => 100,
            _               => return false, 
        };
        chance_1000(steam)
    }
    
    fn base_colors(&self) -> Vec<Color> {
        match self {
            ParticleType::Sand => vec![
                Color::from_hex(0xf4d6a6),
                Color::from_hex(0xf7e0b3),
                Color::from_hex(0xfae9c0),
                Color::from_hex(0xfcf1cd),
                Color::from_hex(0xfffae1),
            ],
            ParticleType::Water => vec![
                Color::from_hex(0xa0d8f1),
                Color::from_hex(0x8cc6ec),
                Color::from_hex(0x73b5e8),
            ],
            ParticleType::Stone => vec![
                Color::from_hex(0xd3d3d3),
                Color::from_hex(0xbdbdbd),
                Color::from_hex(0xa8a8a8),
                Color::from_hex(0x939393),
                Color::from_hex(0x7f7f7f),
            ],
            ParticleType::Coal => vec![
                Color::from_hex(0x4d4d4d),
                Color::from_hex(0x3a3a3a),
                Color::from_hex(0x292929),
            ],
            ParticleType::Fire => vec![
                Color::from_hex(0xff9d80),
                Color::from_hex(0xffab87),
                Color::from_hex(0xffbe90),
                Color::from_hex(0xff8978),
                Color::from_hex(0xff634a),
            ],
            ParticleType::Oil => vec![
                Color::from_hex(0x5e4c3b),
                Color::from_hex(0x4a3a2d),
                Color::from_hex(0x392b20),
            ],
            ParticleType::Wood => vec![
                Color::from_hex(0x85644e),
                Color::from_hex(0x9b725d),
                Color::from_hex(0xb48f6f),
            ],
            ParticleType::Gravel => vec![
                Color::from_hex(0xa3a3a3),
                Color::from_hex(0xafb1b1),
            ],
            ParticleType::Gasoline => vec![
                Color::from_hex(0xf0dec2),
                Color::from_hex(0xeeddc4),
                Color::from_hex(0xf3e2c6),
            ],
            ParticleType::SlimeMold => vec![
                Color::from_hex(0xe3bedc),
                Color::from_hex(0xe2bedf),
                Color::from_hex(0xeac6e4),
                Color::from_hex(0xe5c0e0),
                Color::from_hex(0xeac6e4),
            ],
            ParticleType::Acid => vec![
                Color::from_hex(0xccff99),
                Color::from_hex(0xbbff66),
                Color::from_hex(0xc6ff70),
                Color::from_hex(0xd2ff80),
            ],
            ParticleType::Lava => vec![
                Color::from_hex(0xff6666),  
                Color::from_hex(0xff9966),  
                Color::from_hex(0xff4d4d),  
            ],
            ParticleType::Steam => vec![
                Color::from_hex(0xe0dfe6),
            ],
            ParticleType::Wick => vec![
                Color::from_hex(0x6b4d40),
                Color::from_hex(0x7f6051),
            ],
            ParticleType::Glass => vec![
                Color::from_rgba(195, 220, 230, 240),
                Color::from_rgba(200, 210, 230, 240),
                Color::from_rgba(205, 220, 225, 240),
            ],
            ParticleType::Cloner => vec![
                Color::from_hex(0xffd69b),  
                Color::from_hex(0xd9a576),  
            ],
            _ => vec![BLACK],
        }
    }
}
