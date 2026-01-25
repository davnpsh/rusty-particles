#[derive(Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

pub struct Particle {
    pub mass: f32,
    pub radius: f32,
    pub position: Vector,
    pub velocity: Vector,
}
