use crate::consts;
use crate::types;

use ::rand::Rng;

pub fn random_particles(n: i32, particles: &mut Vec<types::Particle>) {
    let mut rng = ::rand::rng();

    particles.reserve(n as usize);

    for _ in 0..n {
        // WARNING: Specific to 2D space!!!!
        let position: types::Vector = [
            rng.random_range(
                (consts::WINDOW_WIDTH / 5.0)..(consts::WINDOW_WIDTH - consts::WINDOW_WIDTH / 5.0),
            ),
            rng.random_range(
                (consts::WINDOW_HEIGHT / 5.0)
                    ..(consts::WINDOW_HEIGHT - consts::WINDOW_HEIGHT / 5.0),
            ),
        ];

        // WARNING: Specific to 2D space!!!!
        let velocity: types::Vector = [0.0, 0.0];

        let particle = types::Particle {
            mass: rng.random_range(5.0..60.0),
            position: position,
            velocity: velocity,
        };

        particles.push(particle);
    }
}
