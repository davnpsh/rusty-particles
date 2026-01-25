use crate::consts;
use crate::types;

use macroquad::prelude::*;

// WARNING: Specific to 2D space!!!!
pub fn draw_particles(particles: &Vec<types::Particle>) {
    for i in 0..particles.len() {
        draw_circle(
            particles[i].position[0],
            particles[i].position[1],
            particles[i].mass,
            WHITE,
        );
    }
}

pub fn draw_grid() {
    let (mut x1, mut y1): (f32, f32);
    let (mut x2, mut y2): (f32, f32);
    let mut i: f32;
    
    clear_background(BLACK);

    // vertical lines
    i = consts::CELL_SIZE;
    while i < consts::WINDOW_WIDTH {
        (x1, y1) = (i, 0.0);
        (x2, y2) = (i, consts::WINDOW_HEIGHT);

        draw_line(x1, y1, x2, y2, 1.0, GRAY);

        i += consts::CELL_SIZE;
    }

    // horizontal lines
    i = consts::CELL_SIZE;
    while i < consts::WINDOW_HEIGHT {
        (x1, y1) = (0.0, i);
        (x2, y2) = (consts::WINDOW_WIDTH, i);

        draw_line(x1, y1, x2, y2, 1.0, GRAY);

        i += consts::CELL_SIZE;
    }
}
