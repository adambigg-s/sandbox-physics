


mod config;
mod universe;
mod god;
mod test;
mod utils;
mod vector;
mod particle_updates;
mod particles;



use std::{thread, time::Duration};
use macroquad::prelude::*;



use config::{window_configuration, State, CELL_SIZE};
use universe::Universe;
use particles::*;
use utils::{utility_test, interpolate_f32};
use vector::Vector;



#[macroquad::main(window_configuration)]
async fn main() {
    println!("Hello, falling sand!");
    let mut universe:      Universe = Universe::new();
    let mut size:          isize = 10;
    let mut state:         State = State::Simulation;
    let mut particle_type: usize = 0;
    let mut prev_mouse:    Option<Vector<f32>> = None;
    
    let util_test: String = utility_test();
    println!("{}", util_test);

    loop {
        clear_background(Color::from_rgba(220, 235, 245, 230));

        universe.draw();
        if state == State::Simulation {
            universe.update();
        }

        if is_key_pressed(KeyCode::W) {
            particle_type = (particle_type + 1).rem_euclid(ParticleType::get_types());
        } else if is_key_pressed(KeyCode::Q) {
            particle_type = (particle_type as isize - 1).rem_euclid(ParticleType::get_types() as isize) as usize;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            if let Some(prev) = prev_mouse {
                let points: Vec<Vector<f32>> = interpolate_f32(
                    prev,
                    Vector { x, y } 
                );
                for point in points {
                    universe.add_cluster(
                        ParticleType::get_from_int(particle_type),
                        (point.x / CELL_SIZE) as usize,
                        (point.y / CELL_SIZE) as usize, 
                        size,
                        true
                    );
                }
            }
            prev_mouse = Some(
                Vector { x, y }
            );
        } else { prev_mouse = None; }

        if is_key_pressed(KeyCode::R) {
            universe.clear();
        }

        if is_key_pressed(KeyCode::P) {
            if state == State::Simulation {
                state = State::Debug;
            } else { state = State::Simulation; } 
        }
        if is_key_down(KeyCode::D) {
            universe.draw_debug();
        }

        if is_key_pressed(KeyCode::Key1) {
            size = 1.max(size - 3);
        } else if is_key_pressed(KeyCode::Key2) {
            size += 3;
        }
        
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 22.5, BLACK);
        draw_text(&format!("SIZE: {}", size), 10.0, 40.0, 22.5, BLACK);
        draw_text(&format!("Type: {:?}", ParticleType::get_from_int(particle_type)), 10.0, 60.0, 22.5, BLACK);
        next_frame().await;
        thread::sleep(Duration::from_millis(1));
    }
}
