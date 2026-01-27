use std::time::Duration;

use crate::consts;
use crate::state::GlobalState;
// use crate::types;

use macroquad::prelude::*;

// WARNING: Specific to 2D space!!!!
pub fn draw_particles(state: &GlobalState) {
    let particles = &state.mutable_particles;

    for i in 0..particles.len() {
        draw_circle(
            particles[i].position.x,
            particles[i].position.y,
            particles[i].radius,
            WHITE,
        );
    }
}

pub fn display_status_bar(state: &GlobalState) {
    let fps = get_fps();
    let particles_count = state.mutable_particles.len();

    let status_text = format!("fps: {:>3}, particles: {}", fps, particles_count);
    let text_measures = measure_text(&status_text, None, 30, 1.0);

    draw_rectangle(
        20.0,
        0.0,
        text_measures.width,
        text_measures.height + 5.0,
        BLACK,
    );

    draw_text(&status_text, 20.0, 20.0, 30.0, WHITE);
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

pub fn display_feedback_message(state: &mut GlobalState) {
    let timeout = Duration::from_secs(3);

    if !state.feedback_message.is_empty() && state.feedback_timestamp.elapsed() > timeout {
        state.feedback_message.clear();
    }

    if !state.feedback_message.is_empty() {
        let text_measures = measure_text(&state.feedback_message, None, 30, 1.0);

        draw_rectangle(
            20.0,
            consts::WINDOW_HEIGHT - 40.0,
            text_measures.width,
            text_measures.height + 5.0,
            BLACK,
        );

        draw_text(
            &state.feedback_message,
            20.0,
            consts::WINDOW_HEIGHT - 20.0,
            30.0,
            WHITE,
        );
    }
}

pub fn show_mouse_coordinates() {
    let lens_center = mouse_position();

    let status_text = format!("{:?}", lens_center);

    let text_measures = measure_text(&status_text, None, 30, 1.0);

    draw_rectangle(
        consts::WINDOW_WIDTH - text_measures.width - 20.0,
        consts::WINDOW_HEIGHT - 40.0,
        text_measures.width,
        text_measures.height + 5.0,
        BLACK,
    );

    draw_text(
        &status_text,
        consts::WINDOW_WIDTH - text_measures.width - 20.0,
        consts::WINDOW_HEIGHT - 20.0,
        30.0,
        WHITE,
    );
}

pub fn show_particle_information(state: &GlobalState) {
    let particles = &state.mutable_particles;
    let n = particles.len();

    let (x, y) = mouse_position();

    for i in 0..n {
        let (h, k) = (particles[i].position.x, particles[i].position.y);
        let r = particles[i].radius;

        let dx = x - h;
        let dy = y - k;

        // hovering a particle with the mouse
        if dx * dx + dy * dy <= r * r {
            // speed
            let vx = particles[i].velocity.x;
            let vy = particles[i].velocity.y;

            let total_speed = (vx * vx + vy * vy).sqrt();

            let status_text = format!(
                "mass: {}, speed: {:>3.3} px/s",
                particles[i].mass, total_speed
            );

            let text_measures = measure_text(&status_text, None, 30, 1.0);

            draw_rectangle(
                consts::WINDOW_WIDTH - text_measures.width - 20.0,
                0.0,
                text_measures.width,
                text_measures.height + 5.0,
                BLACK,
            );

            draw_text(
                &status_text,
                consts::WINDOW_WIDTH - text_measures.width - 20.0,
                20.0,
                30.0,
                WHITE,
            );
        }
    }
}
