mod consts;
mod controls;
mod physics;
mod scene;
mod setup;
mod state;
mod types;

use crate::state::GlobalState;
use macroquad::{prelude::*, window};

fn window_configuration() -> window::Conf {
    window::Conf {
        window_title: String::from("Rusty Particles"),
        window_width: consts::WINDOW_WIDTH as i32,
        window_height: consts::WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut state = GlobalState::default();

    // load preset 1 by default
    setup::orbital_system(&mut state);

    loop {
        controls::handle_input(&mut state);

        scene::draw_grid();

        physics::apply(&mut state);
        scene::draw_particles(&state);

        scene::display_status_bar(&state);
        scene::display_feedback_message(&mut state);
        scene::show_mouse_coordinates();
        scene::show_particle_information(&state);

        next_frame().await;
    }
}
