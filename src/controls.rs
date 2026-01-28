use crate::setup;
use crate::state::GlobalState;
use crate::utils;
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

            draw_circle(x, y, particle.radius, WHITE);
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        if state.dragging_particle_index != -1 {
            let particle = &mut particles[state.dragging_particle_index as usize];

            let (x, y) = mouse_position();
            particle.position.x = x;
            particle.position.y = y;

            particle.dragging = false;

            state.dragging_particle_index = -1;
        }
    }
}
