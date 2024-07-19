


pub fn utility_test() -> String {
    String::from(":)")
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