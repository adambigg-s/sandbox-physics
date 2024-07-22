


use macroquad::prelude::*;



pub const UNIVERSE_HEIGHT: usize = 610;
pub const UNIVERSE_WIDTH: usize = 825;
pub const CELL_SIZE: f32 = 2.0; 

// pub const UNIVERSE_HEIGHT: usize = 300;
// pub const UNIVERSE_WIDTH: usize = 350;
// pub const CELL_SIZE: f32 = 3.0; 



pub const GRAVITY: f32 = 0.05;
pub const SPEED_MAX: f32 = 4.0;
pub const FALLING_START: f32 = 2.0;



pub fn window_configuration() -> Conf {
    let height: i32 = (UNIVERSE_HEIGHT as f32 * CELL_SIZE) as i32;
    let width: i32 = (UNIVERSE_WIDTH as f32 * CELL_SIZE) as i32;
    Conf {
        window_height: height,
        window_width: width,
        window_resizable: false,
        window_title: String::from("sandbox"),
        ..Default::default()
    }
}

#[derive(PartialEq)]
pub enum State {
    Simulation,
    Debug, 
}
