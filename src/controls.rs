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
}
