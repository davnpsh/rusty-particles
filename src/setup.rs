use crate::consts;
use crate::types;

use rand::prelude::*;
use std::f32::consts::PI;

#[allow(dead_code)]
pub fn random_particles(n: i32, particles: &mut Vec<types::Particle>) {
    let mut rng = rand::rng();

    particles.reserve(n as usize);

    for _ in 0..n {
        let radius = rng.random_range(consts::MINIMUM_MASS..60.0);
        let position = types::Vector {
            x: rng.random_range(
                (consts::WINDOW_WIDTH / 5.0)..(consts::WINDOW_WIDTH - consts::WINDOW_WIDTH / 5.0),
            ),
            y: rng.random_range(
                (consts::WINDOW_HEIGHT / 5.0)
                    ..(consts::WINDOW_HEIGHT - consts::WINDOW_HEIGHT / 5.0),
            ),
        };

        let velocity = types::Vector { x: 0.0, y: 0.0 };

        let particle = types::Particle {
            mass: radius,
            radius: radius,
            position: position,
            velocity: velocity,
        };

        particles.push(particle);
    }
}

#[allow(dead_code)]
pub fn orbital_system(n: i32, particles: &mut Vec<types::Particle>) {
    let mut rng = rand::rng();
    let r1: f32 = 100.0; // inner radius
    let r2 = consts::WINDOW_HEIGHT / 2.0; // outer radius

    // offset
    let cx = consts::WINDOW_WIDTH / 2.0;
    let cy = r2;

    particles.reserve((n + 1) as usize);

    // center particle
    particles.push(types::Particle {
        mass: r1,
        radius: r1,
        position: types::Vector { x: cx, y: cy },
        velocity: types::Vector { x: 0.0, y: 0.0 },
    });

    println!("Spawned at {:?}", [cx, cy]);

    // orbiting particles
    for _ in 0..n {
        let theta = rng.random_range(0.0..2.0 * PI);
        let u: f32 = rng.random();

        let r = (r1.powi(2) + (r2.powi(2) - r1.powi(2)) * u).sqrt();

        // coordinates with offset
        let x = r * theta.cos() + cx;
        let y = r * theta.sin() + cy;

        // orbital speed
        // r1 is the central mass, basically
        let speed = (consts::G * r1 / r).sqrt();

        let dx = x - cx;
        let dy = y - cy;

        let vx = -dy / r * speed;
        let vy = dx / r * speed;

        // ACTUAL radius value
        let radius_property = rng.random_range(consts::MINIMUM_MASS..(r1 / 3.0));

        particles.push(types::Particle {
            mass: radius_property,
            radius: radius_property,
            position: types::Vector { x: x, y: y },
            velocity: types::Vector { x: vx, y: vy },
        });

        println!("Spawned at {:?}", [x, y]);
    }
}
