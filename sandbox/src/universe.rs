


use crate::config;
use crate::particles;
use crate::god;



use macroquad::prelude::*;



use config::{UNIVERSE_HEIGHT, UNIVERSE_WIDTH, CELL_SIZE, GRAVITY, SPEED_MAX, FALLING_START};
use particles::{Particle, ParticleType};
use god::God;



pub struct Universe {
    height: usize,
    width:  usize,
    size:   f32,
    pub gravity:      f32,
    pub max_vel:      f32,
    pub default_fall: f32, 
    pub curr:         Vec<Vec<Particle>>,
}

impl Universe {
    pub fn new() -> Universe {
        let blank: Vec<Vec<Particle>> = vec![vec![Particle::new(ParticleType::None); UNIVERSE_WIDTH]; UNIVERSE_HEIGHT];
        Universe {
            height:       UNIVERSE_HEIGHT,
            width:        UNIVERSE_WIDTH,
            size:         CELL_SIZE,
            gravity:      GRAVITY,
            max_vel:      SPEED_MAX,
            default_fall: FALLING_START,
            curr:         blank,
        }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn clear(&mut self) {
        self.curr = vec![vec![Particle::new(ParticleType::None); self.width]; self.height];
    }
    
    pub fn add_particle(&mut self, particle_type: ParticleType, x: usize, y: usize) {
        if particle_type == ParticleType::None {
            self.curr[y][x].particle_type = ParticleType::None;
        }
        else if self.curr[y][x].particle_type == ParticleType::None {
            self.curr[y][x] = Particle::new(particle_type);
        }
    }

    pub fn add_cluster(&mut self, particle_type: ParticleType, x: usize, y: usize, size: isize, rounded: bool) {
        if size == 1 {
            self.add_particle(particle_type, x, y);
            return;
        }
        for i in (-size)..(size+1) {
            for j in (-size)..(size+1) {
                if rounded && (i * i + j * j) > size * size { continue; }
                if particle_type.spawns_fail() { continue; }
                let (dx, dy): (usize, usize) = ((x as isize + j) as usize, (y as isize + i) as usize);
                if self.in_bounds(dx, dy) {
                    self.add_particle(particle_type, dx, dy);
                }
            }
        }
    }

    pub fn draw(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let particle: &Particle = &self.curr[i][j];
                match particle.particle_type {
                    ParticleType::None => {}
                    ParticleType::Bound => {}
                    _ => {
                        let y: f32 = i as f32 * self.size;
                        let x: f32 = j as f32 * self.size;
                        let color: Color = particle.color;
                        draw_rectangle(x, y, self.size, self.size, color);
                    }
                }
            }
        }
    }

    pub fn draw_debug(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let particle: &Particle = &self.curr[i][j];
                match particle.particle_type {
                    ParticleType::None => {}
                    ParticleType::Bound => {}
                    _ => {
                        let y: f32 = i as f32 * self.size;
                        let x: f32 = j as f32 * self.size;
                        let color: Color = if particle.awake {
                            GREEN
                        } else {
                            RED
                        };
                        draw_rectangle(x, y, self.size, self.size, color);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        for i in (0..self.height).rev() {
            if i % 2 == 0 {
                for j in 0..self.width {
                    self.update_handler(j, i);
                }
            } else {
                for j in (0..self.width).rev() {
                    self.update_handler(j, i);
                }
            }
        }
    }

    fn update_handler(&mut self, x: usize, y: usize) {
        let particle: Particle = self.curr[y][x];
        let mut god: God = God::new(x, y, self);
        particle.update(&mut god);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_check() {
        let mut universe: Universe = Universe::new();
        let god: God = God::new(0, 0, &mut universe);

        let test_1: ParticleType = god.peek(-1, 0);
        assert_eq!(test_1, ParticleType::Bound);

        let test_2: ParticleType = god.peek(0, -1);
        assert_eq!(test_2, ParticleType::Bound);
    }

    #[test]
    fn bounds_checker_check() {
        let universe: Universe = Universe::new();

        assert!(universe.in_bounds(10, 10));
        assert!(!universe.in_bounds(UNIVERSE_WIDTH, 10));
        assert!(universe.in_bounds(UNIVERSE_HEIGHT-1, 10));
    }
}
