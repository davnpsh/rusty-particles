#[derive(Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Particle {
    pub mass: f32,
    pub radius: f32,
    pub position: Vector,
    pub velocity: Vector,
    pub fixed_on_screen: bool,
}
