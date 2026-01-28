use std::time::Instant;

use crate::types;

pub struct GlobalState {
    pub original_particles: Vec<types::Particle>,
    pub mutable_particles: Vec<types::Particle>,
    pub particle_collisions_enabled: bool,
    pub paused: bool,
    pub feedback_message: String,
    pub feedback_timestamp: Instant,
    pub speed: i8,
    pub dragging_particle_index: i32,
    pub last_particle_dragging_position: types::Vector,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            original_particles: Vec::new(),
            mutable_particles: Vec::new(),
            particle_collisions_enabled: true,
            paused: false,
            feedback_message: String::new(),
            feedback_timestamp: Instant::now(),
            speed: 0,
            dragging_particle_index: -1,
            last_particle_dragging_position: types::Vector { x: 0.0, y: 0.0 },
        }
    }
}

impl GlobalState {
    pub fn give_feedback(&mut self, msg: String) {
        self.feedback_message = msg;
        self.feedback_timestamp = Instant::now();
    }
}
