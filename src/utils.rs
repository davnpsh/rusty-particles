use crate::types;
use macroquad::prelude::*;

pub fn is_mouse_over_particle(particle: &types::Particle) -> bool {
    let (x, y) = mouse_position();

    let (h, k) = (particle.position.x, particle.position.y);
    let r = particle.radius;

    let dx = x - h;
    let dy = y - k;

    if dx * dx + dy * dy <= r * r {
        return true;
    }

    return false;
}
