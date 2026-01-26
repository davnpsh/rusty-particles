use crate::types;

pub struct GlobalState {
    pub particles: Vec<types::Particle>,
    pub particle_collisions_enabled: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            particles: Vec::new(),
            particle_collisions_enabled: true,
        }
    }
}
