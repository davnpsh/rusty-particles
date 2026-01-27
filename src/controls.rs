use crate::setup;
use crate::state::GlobalState;
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
}
