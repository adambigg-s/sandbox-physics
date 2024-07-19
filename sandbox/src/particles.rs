


use crate::god;



use macroquad::prelude::*;
use god::God;



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParticleType {
    None,
    Bound,
    Sand,
    Water,
    Stone,
}

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color: Color,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Particle {
            color: particle_type.get_color(),
            particle_type: particle_type,
        }
    }

    pub fn update(self, god: &mut God) {
        match self.particle_type {
            ParticleType::Sand => ParticleType::update_sand(self, god),
            ParticleType::Water => ParticleType::update_water(self, god),
            _ => {}
        }
    }
}

impl ParticleType {
    pub fn get_color(&self) -> Color {
        let colors: Vec<Color> = self.base_colors();
        let idx: usize = (macroquad::rand::gen_range(0.0, 1.0) * colors.len() as f32).floor() as usize;
        return colors[idx];
    }

    pub fn update_sand(particle: Particle, god: &mut God) {
        let dx: isize = god.dir_rand();
        if god.look(0, 1).is_none() || god.look(0, 1).is_water() {
            god.swap(particle, 0, 1);
        } else if god.look(dx, 1).is_none() || god.look(dx, 1).is_water() {
            god.swap(particle, dx, 1);
        }
    }

    pub fn update_water(particle: Particle, god: &mut God) {
        let spread_factor: usize = 12;
        let dx: isize = god.dir_rand();
        if god.look(0, 1).is_none() {
            god.swap(particle, 0, 1);
            return;
        }
        for _ in 1..god.mag_rand(spread_factor) {
            if god.look(dx, 1).is_none() {
                god.swap(particle, dx, 1);
                god.x += dx as usize;
                god.y += 1;
            } else if god.look(dx, 0).is_none() {
                god.swap(particle, dx, 0);
                god.x += dx as usize;
            } else {
                break;
            }
        }
    }

    fn is_none(&self) -> bool {
        self == &ParticleType::None
    }

    fn is_water(&self) -> bool {
        self == &ParticleType::Water
    }

    fn base_colors(&self) -> Vec<Color> {
        match self {
            ParticleType::Sand => vec![
                Color::from_hex(0xFFE580),
                Color::from_hex(0xFFD580),
                Color::from_hex(0xFFCC66),
                Color::from_hex(0xFFB84D),
                Color::from_hex(0xFFA133),
            ],
            ParticleType::Water => vec![
                Color::from_hex(0x80CCFF),
                Color::from_hex(0x66B2FF),
                Color::from_hex(0x4D99FF),
                Color::from_hex(0x3385FF),
                Color::from_hex(0x1A66FF),
            ],
            ParticleType::Stone => vec![
                Color::from_hex(0xCCCCCC),
                Color::from_hex(0xB3B3B3),
                Color::from_hex(0x999999),
                Color::from_hex(0x808080),
                Color::from_hex(0x666666),
            ],
            _ => vec![BLACK],
        }
    }
}

