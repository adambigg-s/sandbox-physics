


use macroquad::prelude::*;



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

pub fn disretize_mouse(grid: f32) -> (usize, usize) {
    let (x, y): (f32, f32) = mouse_position();
    ((x / grid) as usize, (y / grid) as usize)
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