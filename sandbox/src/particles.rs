


use crate::config;
use crate::god;
use crate::utils;
use crate::vector;



use macroquad::prelude::*;
use config::FALLING_START;
use god::God;
use utils::{dir_rand, mag_rand, chance_100};
use vector::Vector;



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParticleType {
    None,
    Bound,
    Sand,
    Water,
    Stone,
    Coal,
    Fire,
    Oil,
    Wood, 
    Gravel,
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color: Color,
    pub awake: bool,
    pub velocity: Vector<f32>,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Particle {
            color: particle_type.get_color(),
            particle_type: particle_type,
            awake: true,
            velocity: Vector { x: 0.0, y: FALLING_START },
        }
    }

    pub fn update(self, god: &mut God) {
        match self.particle_type {
            ParticleType::Sand   => ParticleType::update_sand(self, god),
            ParticleType::Water  => ParticleType::update_water(self, god),
            ParticleType::Coal   => ParticleType::update_coal(self, god),
            ParticleType::Fire   => ParticleType::update_fire(self, god),
            ParticleType::Oil    => ParticleType::update_oil(self, god),
            ParticleType::Gravel => ParticleType::update_gravel(self, god), 
            _ => {}
        }
    }

    pub fn is_awake(&self) -> bool {
        self.awake == true
    }
}

impl ParticleType {
    pub fn get_color(&self) -> Color {
        let colors: Vec<Color> = self.base_colors();
        let idx: usize = mag_rand(colors.len()-1) as usize;
        return colors[idx];
    }

    pub fn update_sand(particle: Particle, god: &mut God) {
        if particle.is_awake() {
            let dx: isize = dir_rand();
            if god.peek(0, 1).is_none() {
                for _ in 0..(particle.velocity.y as usize) {
                    if god.peek(0, 1).is_none() {
                        god.swap(particle, 0, 1);
                        god.nudge(-1, 0);
                        god.nudge(1, 0);
                    }
                }
                god.accel();
            } 
            else if god.peek(0, 1).is_liquid() {
                god.swap(particle, 0, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
            else if god.peek(dx, 1).is_none() || god.peek(dx, 1).is_liquid() {
                god.swap(particle, dx, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
            else if god.peek(-dx, 1).is_none() || god.peek(-dx, 1).is_liquid() {
                god.swap(particle, -dx, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            } 
            else { 
                god.sleep();
                god.slow();
            }
        }
        else {
            if god.peek(0, 1).is_none() {
                god.awaken();
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
        }
    }

    pub fn update_water(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(0, 1).is_none() {
            for _ in 0..(particle.velocity.y as usize) {
                if god.peek(0, 1).is_none() {
                    god.swap(particle, 0, 1);
                    god.nudge(-1, 0);
                    god.nudge(1, 0);
                }
            }
            god.accel();
            return; 
        }
        for _ in 1..mag_rand(particle.particle_type.spread_factor()) {
            if god.peek(dx, 1).is_none() {
                god.swap(particle, dx, 1);
            } else if god.peek(dx, 0).is_none() {
                god.swap(particle, dx, 0);
            } else { 
                god.slow();
                return; 
            }
        }
    }

    pub fn update_coal(particle: Particle, god: &mut God) {
        if particle.is_awake() {
            let dx: isize = dir_rand();
            if god.peek(0, 1).is_none() {
                for _ in 0..(particle.velocity.y as usize) {
                    if god.peek(0, 1).is_none() {
                        god.swap(particle, 0, 1);
                        god.nudge(-1, 0);
                        god.nudge(1, 0);
                    }
                }
                god.accel();
            }
            else if god.peek(0, 1).is_liquid() {
                god.swap(particle, 0, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
            else if god.peek(dx, 1).is_none() || god.peek(dx, 1).is_liquid() || god.peek(dx, 1).is_gas() {
                god.swap(particle, dx, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
            else { 
                god.sleep(); 
                god.slow();
            }
        }
        else {
            if god.peek(0, 1).is_none() {
                god.awaken();
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
        }
    }

    pub fn update_fire(particle: Particle, god: &mut God) {
        if mag_rand(40) == 1 {
            god.remove();
        }
        if god.peek(0, 1).enflames() {
            god.place(particle, 0, 1);
        }  else if god.peek(1, 0).enflames() {
            god.place(particle, 1, 0);
        } else if god.peek(-1, 0).enflames() {
            god.place(particle, -1, 0);
        } else if god.peek(0, -1).enflames() {
            god.place(particle, 0, -1);
        }
    }

    pub fn update_oil(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(0, 1).is_none() || god.peek(0, 1) == ParticleType::Water {
            for _ in 0..(particle.velocity.y as usize) {
                if god.peek(0, 1).is_none() || god.peek(0, 1) == ParticleType::Water {
                    god.swap(particle, 0, 1);
                    god.nudge(-1, 0);
                    god.nudge(1, 0);
                }
            }
            god.accel();
            return; 
        }
        for _ in 1..mag_rand(particle.particle_type.spread_factor()) {
            if god.peek(dx, 1).is_none() {
                god.swap(particle, dx, 1);
            } else if god.peek(dx, 0).is_none() {
                god.swap(particle, dx, 0);
            } else { 
                god.slow();
                return; 
            }
        }
    }

    pub fn update_gravel(particle: Particle, god: &mut God) {
        if particle.is_awake() {
            let dx: isize = if chance_100(50) { dir_rand() } else { 0 };
            if god.peek(0, 1).is_none() {
                for _ in 0..(particle.velocity.y as usize) {
                    if god.peek(0, 1).is_none() {
                        god.swap(particle, 0, 1);
                        god.nudge(-1, 0);
                        god.nudge(1, 0);
                    }
                }
                god.accel();
            }
            else if god.peek(0, 1).is_liquid() && chance_100(20) {
                god.swap(particle, 0, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
            else if god.peek(dx, 1).is_none() || god.peek(dx, 1).is_liquid() || god.peek(dx, 1).is_gas() {
                god.swap(particle, dx, 1);
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
            else { 
                god.sleep(); 
                god.slow();
            }
        }
        else {
            if god.peek(0, 1).is_none() {
                god.awaken();
                god.nudge(-1, 0);
                god.nudge(1, 0);
            }
        }
    }

    fn is_none(&self) -> bool {
        self == &ParticleType::None
    }

    fn is_liquid(&self) -> bool {
        match self {
            ParticleType::Water => true,
            ParticleType::Oil => true,
            _ => false,
        }
    }
    
    fn is_gas(&self) -> bool {
        match self {
            ParticleType::Fire => true,
            _ => false, 
        }
    }

    fn spread_factor(&self) -> usize {
        match self {
            ParticleType::Water => 30,
            ParticleType::Oil => 3,
            _ => 0,
        }
    }

    fn enflames(&self) -> bool {
        let flamability: isize = match self {
            ParticleType::Coal => 5,
            ParticleType::Oil => 30,
            ParticleType::Wood => 2, 
            _ => 0,
        };
        return chance_100(flamability);
    }

    fn base_colors(&self) -> Vec<Color> {
        match self {
            ParticleType::Sand => vec![
                Color::from_hex(0xe1bf92),
                Color::from_hex(0xe7c496),
                Color::from_hex(0xeccca2),
                Color::from_hex(0xf2d2a9),
                Color::from_hex(0xf6d7b0),
            ],
            ParticleType::Water => vec![
                Color::from_hex(0x5cb5e1),
                Color::from_hex(0x3ea4f0),
                Color::from_hex(0xd97e7),
            ],
            ParticleType::Stone => vec![
                Color::from_hex(0xCCCCCC),
                Color::from_hex(0xB3B3B3),
                Color::from_hex(0x999999),
                Color::from_hex(0x808080),
                Color::from_hex(0x666666),
            ],
            ParticleType::Coal => vec![
                Color::from_hex(0x000000),
                Color::from_hex(0x2d2d2d),
                Color::from_hex(0x121520),
            ],
            ParticleType::Fire => vec![
                Color::from_hex(0xFF9180),
                Color::from_hex(0xFFA78C),
                Color::from_hex(0xFFcB87),
                Color::from_hex(0xC70039),
                Color::from_hex(0xFF5733),
            ],
            ParticleType::Oil => vec![
                Color::from_hex(0xccb79c),
                Color::from_hex(0xcab69e),
                Color::from_hex(0xcebaa0),
            ],
            ParticleType::Wood => vec![
                Color::from_hex(0x553311),
                Color::from_hex(0x664433),
                Color::from_hex(0x996633),
            ],
            ParticleType::Gravel => vec![
                Color::from_hex(0xA8BBA8),
                Color::from_hex(0xB0C4B1),
                Color::from_hex(0xC0D2C0),
                Color::from_hex(0xD1E0D1),
            ],
            _ => vec![BLACK],
        }
    }
}

