


mod config;
mod universe;
mod particles;
mod god;
mod test;
mod utils;



use std::{thread, time::Duration};
use macroquad::prelude::*;



use config::{window_configuration, CELL_SIZE};
use universe::Universe;
use particles::*;
use utils::utility_test;



#[macroquad::main(window_configuration)]
async fn main() {
    println!("Hello, falling sand!");
    let mut universe: Universe = Universe::new();
    let mut size: isize = 10;

    let util_test: String = utility_test();
    println!("{}", util_test);

    loop {
        clear_background(Color::from_rgba(122, 120, 120, 1));

        universe.draw();
        universe.update();

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y): (f32, f32) = mouse_position();
            let (x, y): (usize, usize) = ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize);
            universe.add_cluster(ParticleType::Sand, x, y, size);
        } else if is_mouse_button_down(MouseButton::Right) {
            let (x, y): (f32, f32) = mouse_position();
            let (x, y): (usize, usize) = ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize);
            universe.add_cluster(ParticleType::Water, x, y, size)
        } else if is_key_down(KeyCode::A) {
            let (x, y): (f32, f32) = mouse_position();
            let (x, y): (usize, usize) = ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize);
            universe.add_cluster(ParticleType::Stone, x, y, size)
        } else if is_key_down(KeyCode::C) {
            let (x, y): (f32, f32) = mouse_position();
            let (x, y): (usize, usize) = ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize);
            universe.add_cluster(ParticleType::None, x, y, size)
        }

        if is_key_pressed(KeyCode::R) {
            universe.clear();
        }

        if is_key_pressed(KeyCode::Key1) {
            size -= 1;
        } else if is_key_pressed(KeyCode::Key2) {
            size += 1;
        }
        
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 22.5, Color::from_hex(0xf8a5b3));
        draw_text(&format!("SIZE: {}", size), 10.0, 40.0, 22.5, Color::from_hex(0xf8a5b3));
        next_frame().await;
        thread::sleep(Duration::from_millis(1));
    }
}