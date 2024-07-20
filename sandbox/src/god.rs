


use crate::universe;
use crate::particles;
use crate::vector;



use universe::Universe;
use particles::{Particle, ParticleType};
use vector::Vector;



pub struct God<'a> {
    x: usize,
    y: usize,
    universe: &'a mut Universe,
}

impl<'a> God<'a> {
    pub fn new(x: usize, y: usize, universe: &'a mut Universe) -> Self {
        God { x, y, universe, }
    }

    pub fn peek(&self, dx: isize, dy: isize) -> ParticleType {
        let (nx, ny): (usize, usize) = self.idx(dx, dy);
        if self.universe.in_bounds(nx, ny) {
            self.universe.curr[ny][nx].particle_type
        } else { ParticleType::Bound }
    }

    pub fn place(&mut self, particle: Particle, dx: isize, dy: isize) {
        let (nx, ny): (usize, usize) = self.idx(dx, dy);
        self.universe.curr[ny][nx] = particle;
    }

    pub fn swap(&mut self, particle: Particle, dest_x: isize, dest_y: isize) {
        let (nx, ny): (usize, usize) = self.idx(dest_x, dest_y);
        self.universe.curr[self.y][self.x] = self.universe.curr[ny][nx];
        self.universe.curr[ny][nx] = particle;
        self.update(nx, ny);
    }

    pub fn remove(&mut self) {
        self.universe.add_particle(ParticleType::None, self.x, self.y);
    }

    pub fn sleep(&mut self) {
        self.universe.curr[self.y][self.x].awake = false;
    }

    pub fn awaken(&mut self) {
        self.universe.curr[self.y][self.x].awake = true;
    }

    pub fn nudge(&mut self, dx: isize, dy: isize) {
        let (nx, ny): (usize, usize) = self.idx(dx, dy);
        if self.universe.in_bounds(nx, ny) {
            self.universe.curr[ny][nx].awake = true;
        }
    }

    pub fn accel(&mut self) {
        self.universe.curr[self.y][self.x].velocity.y = self.universe.max_vel.min(self.universe.curr[self.y][self.x].velocity.y + self.universe.gravity);
    }

    pub fn slow(&mut self) {
        self.universe.curr[self.y][self.x].velocity = Vector { x: 0.0, y: self.universe.default_fall };
    }

    fn idx(&self, dx: isize, dy: isize) -> (usize, usize) {
        ((self.x as isize + dx) as usize, (self.y as isize + dy) as usize)
    }

    fn update(&mut self, nx: usize, ny: usize) {
        self.x = nx;
        self.y = ny;
    }
}