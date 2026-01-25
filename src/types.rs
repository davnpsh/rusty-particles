use crate::consts;

pub type Vector = [f32; consts::DIMENSIONS];

pub struct Particle {
    pub mass: f32,
    pub position: Vector,
    pub velocity: Vector,
}
