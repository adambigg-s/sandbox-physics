


use crate::vector;



use macroquad::prelude::*;



use vector::Vector;



pub fn utility_test() -> String {
    String::from(":)")
}

pub fn dir_rand() -> isize {
    if rand::gen_range(0, 2) % 2 == 0 { -1 } else { 1 }
}

pub fn mag_rand(max: usize) -> isize {
    rand::gen_range(0, max+1) as isize
}

pub fn chance_100(chance: isize) -> bool {
    mag_rand(100) < chance
}

pub fn chance_1000(chance: isize) -> bool {
    mag_rand(1000) < chance
}

pub fn interpolate_f32(a: Vector<f32>, b: Vector<f32>) -> Vec<Vector<f32>> {
    let dx: f32 = b.x - a.x;
    let dy: f32 = b.y - a.y;
    let distance: f32 = (dx * dx + dy * dy).sqrt();
    let steps: usize = distance.round() as usize;
    let mut points: Vec<Vector<f32>> = Vec::new();

    if steps > 0 { } else {
        points.push(b);
        return points; 
    }
    for i in 0..(steps+1) {
        let t: f32 = (i as f32) / distance;
        points.push(
            Vector {
                x: a.x + t * dx,
                y: a.y + t * dy,
            }
        );
    }
    
    points
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_lower_test() {
        let mut test: isize = isize::MAX;
        for _ in 0..1000 {
            test = test.min(mag_rand(10));
        }
        assert_eq!(0, test);
    }

    #[test]
    fn rng_upper_test() {
        let mut test: isize = 0;
        for _ in 0..1000 {
            test = test.max(mag_rand(10));
        }
        assert_eq!(10, test);
    }

    #[test]
    fn rand_dir() {
        let mut neg: usize = 0;
        let mut pos: usize = 0;
        for _ in 0..10000 {
            let test: isize = dir_rand();
            if test > 0 { pos += 1 }
            if test < 0 { neg += 1 }
        }
        assert!(neg > 4800 && pos > 4800);
    }
}



// pub fn update_water(particle: Particle, god: &mut God) {
    //     let dx: isize = god.dir_rand();
    //     let spread_factor: usize = 20;
    //     if god.look(0, 1).is_none() {
    //         god.swap(particle, 0, 1);
    //         return;
    //     }
    //     for del in 1..god.mag_rand(spread_factor) {
    //         let delta: isize = del * dx;
    //         if god.look(delta, del).is_none() {
    //             god.swap(particle, delta, del);
    //             god.x += delta as usize;
    //             god.y += del as usize;
    //             break;
    //         }
    //         if god.look(delta, 0).is_none() {
    //             god.swap(particle, delta, 0);
    //             god.x += delta as usize;
    //             break;
    //         }
    //         break;
    //     }
    // }

    // pub fn update_water(particle: Particle, god: &mut God) {
    //     let dx: isize = god.dir_rand();
    //     let spread_factor: usize = 5;
    //     if god.look(0, 1).is_none() {
    //         god.swap(particle, 0, 1);
    //         return;
    //     }
    //     if god.look(dx, 1).is_none() {
    //         god.swap(particle, dx, 1);
    //         god.x += dx as usize;
    //         god.y += 1;
    //     }
    //     if god.look(dx, 0).is_none() {
    //         for _ in 1..god.mag_rand(spread_factor) {
    //             if god.look(dx, 0).is_none() {
    //                 god.swap(particle, dx, 0);
    //                 god.x += dx as usize;
    //                 continue;
    //             }
    //             break;
    //         }
    //     }
    // }
