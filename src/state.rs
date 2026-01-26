use std::time::Instant;

use crate::types;

pub struct GlobalState {
    pub particles: Vec<types::Particle>,
    pub particle_collisions_enabled: bool,
    pub feedback_message: String,
    pub feedback_timestamp: Instant,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            particles: Vec::new(),
            particle_collisions_enabled: true,
            feedback_message: String::new(),
            feedback_timestamp: Instant::now(),
        }
    }
}

impl GlobalState {
    pub fn give_feedback(&mut self, msg: String) {
        self.feedback_message = msg;
        self.feedback_timestamp = Instant::now();
    }
}
