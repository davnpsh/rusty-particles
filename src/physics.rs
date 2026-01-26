use crate::consts;
use crate::state::GlobalState;
use crate::types;

use rayon::prelude::*;

fn handle_collision_between_particles(particles: &mut Vec<types::Particle>) {
    let len = particles.len();

    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = particles.split_at_mut(j);
            let a = &mut left[i];
            let b = &mut right[0];

            let dx = b.position.x - a.position.x;
            let dy = b.position.y - a.position.y;

            let dist2 = dx * dx + dy * dy;
            let min_dist = a.radius + b.radius;

            // no collision
            if dist2 >= min_dist * min_dist {
                continue;
            }

            let distance = dist2.sqrt().max(0.0001);

            let nx = dx / distance;
            let ny = dy / distance;

            // relative velocity
            let rvx = b.velocity.x - a.velocity.x;
            let rvy = b.velocity.y - a.velocity.y;

            let vel_along_normal = rvx * nx + rvy * ny;

            // Skip if separating
            if vel_along_normal > 0.0 {
                continue;
            }

            let inv_mass_a = if a.fixed_on_screen { 0.0 } else { 1.0 / a.mass };
            let inv_mass_b = if b.fixed_on_screen { 0.0 } else { 1.0 / b.mass };

            let impulse =
                -(1.0 + consts::RESTITUTION) * vel_along_normal / (inv_mass_a + inv_mass_b);

            let ix = impulse * nx;
            let iy = impulse * ny;

            // Apply impulse
            a.velocity.x -= ix * inv_mass_a * 1.2;
            a.velocity.y -= iy * inv_mass_a * 1.2;

            b.velocity.x += ix * inv_mass_b * 1.2;
            b.velocity.y += iy * inv_mass_b * 1.2;

            // Positional correction
            let overlap = min_dist - distance;
            let correction = overlap * 0.5;

            a.position.x -= nx * correction;
            a.position.y -= ny * correction;

            b.position.x += nx * correction;
            b.position.y += ny * correction;
        }
    }
}

fn handle_particle_bounds_collision(p: &mut types::Particle) {
    // LEFT
    if p.position.x - p.radius < 0.0 {
        p.position.x = p.radius;
        p.velocity.x = -p.velocity.x * consts::RESTITUTION;
    }

    // RIGHT
    if p.position.x + p.radius > consts::WINDOW_WIDTH {
        p.position.x = consts::WINDOW_WIDTH - p.radius;
        p.velocity.x = -p.velocity.x * consts::RESTITUTION;
    }

    // TOP
    if p.position.y - p.radius < 0.0 {
        p.position.y = p.radius;
        p.velocity.y = -p.velocity.y * consts::RESTITUTION;
    }

    // BOTTOM
    if p.position.y + p.radius > consts::WINDOW_HEIGHT {
        p.position.y = consts::WINDOW_HEIGHT - p.radius;
        p.velocity.y = -p.velocity.y * consts::RESTITUTION;
    }
}

fn update_particle_position(p: &mut types::Particle) {
    p.position.x += p.velocity.x;
    p.position.y += p.velocity.y;

    handle_particle_bounds_collision(p);
}

fn calculate_distance_vector(a: &types::Particle, b: &types::Particle) -> types::Vector {
    types::Vector {
        x: b.position.x - a.position.x,
        y: b.position.y - a.position.y,
    }
}

fn calculate_g_force(a: &types::Particle, b: &types::Particle) -> types::Vector {
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

pub fn apply(state: &mut GlobalState) {
    if state.paused {
        return;
    }

    let particles = &mut state.particles;

    let n = particles.len();
    let mut accelerations = vec![types::Vector { x: 0.0, y: 0.0 }; n];

    // multi-threaded
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

    // multi-threaded
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

    if state.particle_collisions_enabled {
        handle_collision_between_particles(particles);
    }
}
