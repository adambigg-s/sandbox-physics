


use macroquad::prelude::*;



pub const UNIVERSE_HEIGHT: usize = 610;
pub const UNIVERSE_WIDTH: usize = 825;
pub const CELL_SIZE: f32 = 2.0; 
// pub const UNIVERSE_HEIGHT: usize = 35;
// pub const UNIVERSE_WIDTH: usize = 50;
// pub const CELL_SIZE: f32 = 30.0; 




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