use crate::consts;
use crate::types;

use rayon::prelude::*;

pub fn update_particle_position(p: &mut types::Particle) {
    p.position.x += p.velocity.x;
    p.position.y += p.velocity.y;

    // detect collisions left/right walls
    if p.position.x - p.radius < 0.0 {
        p.position.x = p.radius;
        p.velocity.x = -p.velocity.x * consts::RESTITUTION;
    } else if p.position.x + p.radius > consts::WINDOW_WIDTH {
        p.position.x = consts::WINDOW_WIDTH - p.radius;
        p.velocity.x = -p.velocity.x * consts::RESTITUTION;
    }

    // detect collisions top/bottom walls
    if p.position.y - p.radius < 0.0 {
        p.position.y = p.radius;
        p.velocity.y = -p.velocity.y * consts::RESTITUTION;
    } else if p.position.y + p.radius > consts::WINDOW_HEIGHT {
        p.position.y = consts::WINDOW_HEIGHT - p.radius;
        p.velocity.y = -p.velocity.y * consts::RESTITUTION;
    }
}

pub fn calculate_distance_vector(a: &types::Particle, b: &types::Particle) -> types::Vector {
    types::Vector {
        x: b.position.x - a.position.x,
        y: b.position.y - a.position.y,
    }
}

pub fn calculate_g_force(a: &types::Particle, b: &types::Particle) -> types::Vector {
    let dead_zone = consts::MINIMUM_MASS;

    let distance_vector = calculate_distance_vector(a, b);
    let distance = (distance_vector.x.powi(2) + distance_vector.y.powi(2)).sqrt();

    // to avoid particles shooting each other out of the screen
    if distance < dead_zone {
        return types::Vector { x: 0.0, y: 0.0 };
    }

    let force_magnitude = consts::G * a.mass * b.mass / distance.powi(2);

    return types::Vector {
        x: force_magnitude * (distance_vector.x / distance),
        y: force_magnitude * (distance_vector.y / distance),
    };
}

pub fn apply(particles: &mut Vec<types::Particle>) {
    let n = particles.len();
    let mut accelerations = vec![types::Vector { x: 0.0, y: 0.0 }; n];

    // multi-thread
    accelerations
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, acc)| {
            let pi = &particles[i];

            if pi.fixed_on_screen {
                return;
            }

            let mut ax = 0.0;
            let mut ay = 0.0;

            let inv_mass = 1.0 / pi.mass;

            for (j, pj) in particles.iter().enumerate() {
                if i == j {
                    continue;
                }

                let force = calculate_g_force(pi, pj);

                ax += force.x * inv_mass;
                ay += force.y * inv_mass;
            }

            acc.x = ax;
            acc.y = ay;
        });

    // multi-thread
    particles
        .par_iter_mut()
        .zip(accelerations.par_iter())
        .for_each(|(p, acc)| {
            if p.fixed_on_screen {
                return;
            }

            p.velocity.x += acc.x;
            p.velocity.y += acc.y;

            update_particle_position(p);
        });
}
