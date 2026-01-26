use crate::types;

pub struct GlobalState {
    pub particles: Vec<types::Particle>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            particles: Vec::new(),
        }
    }
}
