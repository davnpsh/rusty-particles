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
    if is_key_pressed(KeyCode::L) {
    	state.mutable_particles = state.original_particles.clone();
     
     	state.give_feedback("loop!".to_string());
    }
}
