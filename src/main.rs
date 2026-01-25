mod consts;
mod physics;
mod scene;
mod setup;
mod types;

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
    let mut particles: Vec<types::Particle> = Vec::new();

    // setup::random_particles(consts::PARTICLES_QUANTITY, &mut particles);
    setup::orbital_system(consts::PARTICLES_QUANTITY, &mut particles);

    loop {
        scene::draw_grid();

        physics::apply(&mut particles);
        scene::draw_particles(&particles);
        
        scene::display_status_bar();

        next_frame().await;
    }
}
