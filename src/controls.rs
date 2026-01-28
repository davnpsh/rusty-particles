use std::time::Instant;

use crate::state::GlobalState;
use crate::{consts, utils};
use crate::{setup, types};
use macroquad::prelude::*;

pub fn handle_input(state: &mut GlobalState) {
    // toggle collisions
    if is_key_pressed(KeyCode::C) {
        state.particle_collisions_enabled = !state.particle_collisions_enabled;

        state.give_feedback(format!(
            "collisions: {}",
            if state.particle_collisions_enabled {
                "on"
            } else {
                "off"
            }
        ));
    }

    // toggle pause
    if is_key_pressed(KeyCode::Space) {
        state.paused = !state.paused;

        state.give_feedback(
            if state.paused {
                "|| paused"
            } else {
                "|> resumed"
            }
            .to_string(),
        );
    }

    // loop
    if is_key_pressed(KeyCode::R) {
        state.mutable_particles = state.original_particles.clone();

        state.give_feedback("reset!".to_string());
    }

    // speed
    if is_key_pressed(KeyCode::Right) {
        if state.speed < 2 {
            state.speed += 1;
        }
    }

    if is_key_pressed(KeyCode::Left) {
        if state.speed > -2 {
            state.speed -= 1;
        }
    }

    if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) {
        let msg = match state.speed {
            2 => "speed >>",
            1 => "speed >",
            -1 => "< speed",
            -2 => "<< speed",
            _ => "normal speed",
        };

        state.give_feedback(msg.to_string());
    }

    // load presets
    if let Some(key) = get_last_key_pressed() {
        match key {
            // preset 1
            KeyCode::Key1 => {
                setup::orbital_system(state);
                state.give_feedback("loaded preset: [orbit]".to_string());
            }

            // preset 2
            KeyCode::Key2 => {
                setup::random_particles(state);
                state.give_feedback("loaded preset: [random]".to_string());
            }
            _ => {}
        }
    }

    handle_mouse_events(state);
}

fn handle_mouse_events(state: &mut GlobalState) {
    let particles = &mut state.mutable_particles;
    let n = particles.len();

    // altering particles
    if is_mouse_button_pressed(MouseButton::Left) {
        for i in 0..n {
            if utils::is_mouse_over_particle(&particles[i]) {
                let particle = &mut particles[i];

                if particle.fixed_on_screen {
                    return;
                }

                particle.dragging = true;

                state.dragging_particle_index = i as i32;
            }
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        if state.dragging_particle_index != -1 {
            let particle = &particles[state.dragging_particle_index as usize];

            let (x, y) = mouse_position();
            state.last_particle_dragging_position = types::Vector { x: x, y: y };

            draw_circle(x, y, particle.radius, BLUE);
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        if state.dragging_particle_index != -1 {
            let particle = &mut particles[state.dragging_particle_index as usize];

            // starting point
            let (h, k) = (
                state.last_particle_dragging_position.x,
                state.last_particle_dragging_position.y,
            );

            // finish point
            let (x, y) = mouse_position();
            particle.position.x = x;
            particle.position.y = y;

            let power_factor = consts::THROWING_POWER_FACTOR;

            let vx = (x - h) * power_factor;
            let vy = (y - k) * power_factor;

            particle.velocity.x = vx;
            particle.velocity.y = vy;

            particle.dragging = false;

            state.dragging_particle_index = -1;
        }
    }

    // creating particles
    if is_mouse_button_pressed(MouseButton::Right) {
        state.particle_creation_timestamp = Instant::now();
    }

    if is_mouse_button_down(MouseButton::Right) {
        let elapsed = state.particle_creation_timestamp.elapsed().as_millis();

        let mass = elapsed as f32 / 10.0;

        if mass > 0.0 {
            if mass <= 100.0 {
                state.particle_creation_mass = mass;
            } else {
                state.particle_creation_mass = 100.0;
            }
        }

        // draw
        let (x, y) = mouse_position();

        draw_circle(x, y, state.particle_creation_mass, GREEN);
    }

    if is_mouse_button_released(MouseButton::Right) {
        if state.particle_creation_mass > consts::PARTICLE_MINIMUM_MASS {
            let (x, y) = mouse_position();

            state.mutable_particles.push(types::Particle {
                mass: state.particle_creation_mass,
                radius: state.particle_creation_mass,
                position: types::Vector { x: x, y: y },
                velocity: types::Vector { x: 0.0, y: 0.0 },
                fixed_on_screen: false,
                dragging: false,
            });
        }

        state.particle_creation_mass = 0.0;
    }
}
