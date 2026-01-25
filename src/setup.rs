use crate::consts;
use crate::types;

use rand::prelude::*;
use std::f32::consts::PI;

#[allow(dead_code)]
pub fn random_particles(n: i32, particles: &mut Vec<types::Particle>) {
    let mut rng = rand::rng();

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
            mass: rng.random_range(consts::MINIMUM_MASS..60.0),
            position: position,
            velocity: velocity,
        };

        particles.push(particle);
    }
}

#[allow(dead_code)]
pub fn orbital_system(n: i32, particles: &mut Vec<types::Particle>) {
    let mut rng = rand::rng();
    let r1 = 100.0; // inner radius
    let r2 = consts::WINDOW_HEIGHT / 2.0; // outer radius

    // center particle
    particles.push(types::Particle {
        mass: r1,
        position: [consts::WINDOW_WIDTH / 2.0, r2],
        velocity: [0.0, 0.0],
    });

    // orbiting particles
    for _ in 0..n {
        let theta = rng.random_range(0.0..2.0 * PI);
        let u: f32 = rng.random();

        let r = (r1.powi(2) + (r2.powi(2) - r1.powi(2)) * u).sqrt();

        let x = r * theta.cos() + consts::WINDOW_WIDTH / 2.0;
        let y = r * theta.sin() + consts::WINDOW_HEIGHT / 2.0;

        particles.push(types::Particle {
            mass: rng.random_range(consts::MINIMUM_MASS..(r1 / 2.0)),
            position: [x, y],
            velocity: [0.0, 0.0],
        });

        println!("Spawned at {:?}", [x, y]);
    }
}
