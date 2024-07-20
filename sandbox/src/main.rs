


mod config;
mod universe;
mod particles;
mod god;
mod test;
mod utils;
mod vector;



use std::{thread, time::Duration};
use macroquad::prelude::*;



use config::{window_configuration, State, CELL_SIZE};
use universe::Universe;
use particles::*;
use utils::{disretize_mouse,  utility_test};



#[macroquad::main(window_configuration)]
async fn main() {
    println!("Hello, falling sand!");
    let mut universe: Universe = Universe::new();
    let mut size: isize = 10;
    let mut state: State = State::Simulation;

    let util_test: String = utility_test();
    println!("{}", util_test);

    let _ = universe.curr[0][0].velocity.x;

    loop {
        clear_background(Color::from_rgba(150, 180, 190, 230));

        universe.draw();
        if state == State::Debug {
            universe.draw_debug();
        }
        universe.update();

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Sand, x, y, size, true)
        } else if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Water, x, y, size, true)
        } else if is_key_down(KeyCode::A) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Stone, x, y, size, true )
        } else if is_key_down(KeyCode::C) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::None, x, y, size, false)
        } else if is_key_down(KeyCode::H) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Coal, x, y, size, true )
        } else if is_key_down(KeyCode::F) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Fire, x, y, size, true);
        } else if is_key_down(KeyCode::O) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Oil, x, y, size, true);
        } else if is_key_down(KeyCode::W) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Wood, x, y, size, true);
        } else if is_key_down(KeyCode::G) {
            let (x, y) = disretize_mouse(CELL_SIZE);
            universe.add_cluster(ParticleType::Gravel, x, y, size, true);
        }

        if is_key_pressed(KeyCode::R) {
            universe.clear();
        }

        if is_key_pressed(KeyCode::P) {
            if state == State::Simulation {
                state = State::Debug;
            } else { state = State::Simulation ; } 
        }

        if is_key_pressed(KeyCode::Key1) {
            size = 1.max(size - 3);
        } else if is_key_pressed(KeyCode::Key2) {
            size += 3;
        }
        
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 22.5, BLACK);
        draw_text(&format!("SIZE: {}", size), 10.0, 40.0, 22.5, BLACK);
        next_frame().await;
        thread::sleep(Duration::from_millis(1));
    }
}