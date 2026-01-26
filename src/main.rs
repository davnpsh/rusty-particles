mod consts;
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

    setup::orbital_system(&mut state, consts::PARTICLES_QUANTITY);

    loop {
        scene::draw_grid();

        physics::apply(&mut state);
        scene::draw_particles(&mut state);

        scene::display_status_bar();

        next_frame().await;
    }
}
