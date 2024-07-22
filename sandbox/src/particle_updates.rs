


use crate::god;
use crate::utils;
use crate::particles;



use god::God;
use utils::{mag_rand, chance_100, chance_1000, dir_rand};
use particles::{Particle, ParticleType};



impl ParticleType {
    pub fn update_sand(particle: Particle, god: &mut God) {
        if particle.awake {
            let dx: isize = dir_rand();
            if god.peek(0, 1).is_none() || god.peek(0, 1).is_gas() {
                for _ in 0..(particle.velocity.y as usize) {
                    if god.peek(0, 1).is_none() || god.peek(0, 1).is_gas() {
                        god.swap(particle, 0, 1);
                        god.nudge_neighbors();
                    }
                }
                god.accel();
            } else if god.peek(0, 1).is_liquid() {
                god.swap(particle, 0, 1);
                god.nudge_neighbors();
            } else if god.peek(dx, 1).is_none() || god.peek(dx, 1).is_liquid() {
                god.swap(particle, dx, 1);
                god.nudge_neighbors();
            } else if god.peek(-dx, 1).is_none() || god.peek(-dx, 1).is_liquid() {
                god.swap(particle, -dx, 1);
                god.nudge_neighbors();
            } else { 
                god.sleep();
                god.slow();
            }
        }
        else if god.peek(0, 1).is_none() {
            god.awaken();
            god.nudge_neighbors();
        }
    }

    pub fn update_water(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(0, 1).is_none() || god.peek(0, 1).is_gas() {
            for _ in 0..(particle.velocity.y as usize) {
                if god.peek(0, 1).is_none() || god.peek(0, 1).is_gas() {
                    god.swap(particle, 0, 1);
                    god.nudge_neighbors();
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
        if particle.awake {
            let dx: isize = dir_rand();
            if god.peek(0, 1).is_none() {
                for _ in 0..(particle.velocity.y as usize) {
                    if god.peek(0, 1).is_none() {
                        god.swap(particle, 0, 1);
                        god.nudge_neighbors();
                    }
                }
                god.accel();
            } else if god.peek(0, 1).is_liquid() {
                god.swap(particle, 0, 1);
                god.nudge_neighbors();
            } else if god.peek(dx, 1).is_none() || god.peek(dx, 1).is_liquid() || god.peek(dx, 1).is_gas() {
                god.swap(particle, dx, 1);
                god.nudge_neighbors();
            } else { 
                god.sleep(); 
                god.slow();
            }
        }
        else if god.peek(0, 1).is_none() {
            god.awaken();
            god.nudge_neighbors();
        }
    }

    pub fn update_fire(particle: Particle, god: &mut God) {
        if chance_100(1) {
            god.remove();
        }
        if god.peek(0, 1).enflames() {
            god.place(particle, 0, 1);
            if god.peek(0, -1).is_none() {
                god.place(Particle::make_smoke(), 0, -1);
            } else if god.peek(1, -1).is_none() {
                god.place(Particle::make_smoke(), 1, -1);
            } else if god.peek(-1, -1).is_none() {
                god.place(Particle::make_smoke(), -1, -1);
            }
        } else if god.peek(1, 0).enflames() {
            god.place(particle, 1, 0);
            if god.peek(0, -1).is_none() {
                god.place(Particle::make_smoke(), 0, -1);
            } else if god.peek(1, -1).is_none() {
                god.place(Particle::make_smoke(), 1, -1);
            } else if god.peek(-1, -1).is_none() {
                god.place(Particle::make_smoke(), -1, -1);
            }
        } else if god.peek(-1, 0).enflames() {
            god.place(particle, -1, 0);
            if god.peek(0, -1).is_none() {
                god.place(Particle::make_smoke(), 0, -1);
            } else if god.peek(1, -1).is_none() {
                god.place(Particle::make_smoke(), 1, -1);
            } else if god.peek(-1, -1).is_none() {
                god.place(Particle::make_smoke(), -1, -1);
            }
        } else if god.peek(0, -1).enflames() {
            god.place(particle, 0, -1);
            if god.peek(0, -1).is_none() {
                god.place(Particle::make_smoke(), 0, -1);
            } else if god.peek(1, -1).is_none() {
                god.place(Particle::make_smoke(), 1, -1);
            } else if god.peek(-1, -1).is_none() {
                god.place(Particle::make_smoke(), -1, -1);
            }
        }
    }

    pub fn update_oil(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(0, 1).is_none() {
            for _ in 0..(particle.velocity.y as usize) {
                if god.peek(0, 1).is_none() {
                    god.swap(particle, 0, 1);
                    god.nudge_neighbors();
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
        if particle.awake {
            let dx: isize = if chance_100(50) { dir_rand() } else { 0 };
            if god.peek(0, 1).is_none() {
                for _ in 0..(particle.velocity.y as usize) {
                    if god.peek(0, 1).is_none() {
                        god.swap(particle, 0, 1);
                        god.nudge_neighbors();
                    }
                }
                god.accel();
            } else if god.peek(0, 1).is_liquid() {
                god.swap(particle, 0, 1);
                god.nudge_neighbors();
            } else if god.peek(dx, 1).is_none() || god.peek(dx, 1).is_liquid() || god.peek(dx, 1).is_gas() && chance_100(20) {
                god.swap(particle, dx, 1);
                god.nudge_neighbors();
            } else if god.peek(dx, 0).is_none() && chance_100(60) {
                god.swap(particle, dx, 0);
                god.nudge_neighbors();
            } else { 
                god.sleep(); 
                god.slow();
            }
        }
        else if god.peek(0, 1).is_none() {
            god.awaken();
            god.nudge_neighbors();
        }
    }
    
    pub fn update_gasoline(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(0, 1).is_none() {
            for _ in 0..(particle.velocity.y as usize) {
                if god.peek(0, 1).is_none() {
                    god.swap(particle, 0, 1);
                    god.nudge_neighbors();
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

    pub fn update_slimemold(particle: Particle, god: &mut God) {
        if !god.peek(0, 1).is_none() && god.peek(0, 1) != ParticleType::Bound && chance_100(1) {
            god.place(particle, 0, 1);
        } else if !god.peek(0, -1).is_none() && god.peek(0, -1) != ParticleType::Bound && chance_100(1) {
            god.place(particle, 0, -1);
        } else if !god.peek(1, 0).is_none() && god.peek(1, 0) != ParticleType::Bound && chance_100(1) {
            god.place(particle, 1, 0);
        } else if !god.peek(-1, 0).is_none() && god.peek(-1, 0) != ParticleType::Bound && chance_100(1) {
            god.place(particle, -1, 0);
        }
    }

    pub fn update_smoke(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if chance_1000(1) {
            god.remove();
        } else if god.peek(dx, -1).is_none() && chance_100(35) {
            god.swap(particle, dx, -1);
        }
    }

    pub fn update_acid(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(0, -1).is_none() && chance_1000(20) {
            god.remove();
            return;
        }
        if god.peek(0, 1).is_none() {
            for _ in 0..(particle.velocity.y as usize) {
                if god.peek(0, 1).is_none() {
                    god.swap(particle, 0, 1);
                    god.nudge_neighbors();
                }
            }
            god.accel();
            return;
        }    
        else if god.peek(0, 1).corrodes() {
            god.remove();
            god.place(particle, 0, 1);
        } else if god.peek(1, 1).corrodes() {
            god.remove();
            god.place(particle, 1, 1);
        } else if god.peek(-1, 1).corrodes() {
            god.remove();
            god.place(particle, -1, 1);
        } else if god.peek(0, -1).corrodes() {
            god.remove();
            god.place(particle, 0, -1);
        }
        for _ in 1..mag_rand(particle.particle_type.spread_factor()) {
            if god.peek(dx, 1).is_none() {
                god.swap(particle, dx, 1);
            } else if god.peek(dx, 0).is_none() {
                god.swap(particle, dx, 0);
            } else {
                return;
            }
        }
    }

    pub fn update_lava(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if (god.peek(0, 1).is_none() 
        || god.peek(0, 1).is_liquid() 
        || god.peek(0, 1).is_gas()) 
        && chance_100(90) 
        {
            god.swap(particle, 0, 1);
        } else if god.peek(dx, 0).is_none() && chance_100(20) {
            god.swap(particle, dx, 0);
        }
        if god.peek(0, 1).enflames() && chance_100(30) {
            god.place(Particle::make_fire(), 0, 1);
        } else if god.peek(0, -1).enflames() && chance_100(30) {
            god.place(Particle::make_fire(), 0, -1);
        } else if god.peek(dx, 0).enflames() && chance_100(10) {
            god.place(Particle::make_fire(), dx, 0);
        }
        if god.peek(0, 1).evaporates() {
            god.place(Particle::make_steam(), 0, 1);
        } else if god.peek(0, -1).evaporates() {
            god.place(Particle::make_steam(), 0, -1);
        } else if god.peek(dx, 0).evaporates() {
            god.place(Particle::make_steam(), dx, 0);
        }
    }
    
    pub fn update_steam(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if chance_1000(1) {
            god.place(Particle::make_water(), 0, 0);
        } else if god.peek(dx, -1).is_none() && chance_100(50) {
            god.swap(particle, dx, -1);
        }
    }

    pub fn update_cloner(particle: Particle, god: &mut God) {
        let dx: isize = dir_rand();
        if god.peek(dx, 1).is_none() {
            god.swap(particle, dx, 1);
        }
        if chance_100(70) { return; }
        if !god.peek(0, 1).is_none() 
            && god.peek(0, 1) != ParticleType::Bound 
            && god.peek(0, 1) != ParticleType::Cloner 
        {
            let mut particle: Particle = particle;
            particle.particle_type = god.peek(0, 1);
            particle.color = god.peek(0, 1).get_color();
            god.place(particle, 0, 0);
        } else if !god.peek(0, -1).is_none() 
            && god.peek(0, -1) != ParticleType::Bound 
            && god.peek(0, -1) != ParticleType::Cloner 
        {
            let mut particle: Particle = particle;
            particle.particle_type = god.peek(0, -1);
            particle.color = god.peek(0, -1).get_color();
            god.place(particle, 0, 0);
        }
    }    
}
