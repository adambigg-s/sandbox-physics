


use crate::universe;
use crate::particles;



use macroquad::rand;
use universe::Universe;
use particles::{Particle, ParticleType};



pub struct God<'a> {
    pub x: usize,
    pub y: usize,
    pub universe: &'a mut Universe,
}

impl<'a> God<'a> {
    pub fn look(&self, dx: isize, dy: isize) -> ParticleType {
        let (nx, ny): (usize, usize) = self.idx(dx, dy);
        if self.universe.in_bounds(nx, ny) {
            return self.universe.curr[ny][nx].particle_type;
        }
        else {
            return ParticleType::Bound;
        }
    }

    pub fn swap(&mut self, particle: Particle, dest_x: isize, dest_y: isize) {
        let (nx, ny): (usize, usize) = self.idx(dest_x, dest_y);
        self.universe.curr[self.y][self.x] = self.universe.curr[ny][nx];
        self.universe.curr[ny][nx] = particle;
    }

    pub fn dir_rand(&self) -> isize {
        if rand::gen_range(0, 2) % 2 == 0 { -1 } else { 1 }
    }

    pub fn mag_rand(&self, max: usize) -> isize {
        rand::gen_range(1, max+1) as isize
    }

    fn idx(&self, dx: isize, dy: isize) -> (usize, usize) {
        ((self.x as isize + dx) as usize, (self.y as isize + dy) as usize)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_test() {
        let mut universe: Universe = Universe::new();
        let god = God {
            x: 0,
            y: 0,
            universe: &mut universe,
        };

        let mut test: isize = 0;
        for _ in 0..1000 {
            test = test.max(god.mag_rand(3));
        }

        assert_eq!(test, 3);
    }
}