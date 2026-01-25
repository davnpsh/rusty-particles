use crate::consts;
use crate::types;

pub fn update_particle_position(p: &mut types::Particle) {
    p.position.x += p.velocity.x;
    p.position.y += p.velocity.y;
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

    // compute all forces
    for i in 0..n {
        for j in (i + 1)..n {
            let force = calculate_g_force(&particles[i], &particles[j]);

            if !particles[i].fixed_on_screen {
                accelerations[i].x += force.x / particles[i].mass;
                accelerations[i].y += force.y / particles[i].mass;
            }

            // Newton's 3rd law
            accelerations[j].x -= force.x / particles[j].mass;
            accelerations[j].y -= force.y / particles[j].mass;
        }
    }

    // update positions
    for i in 0..particles.len() {
        if particles[i].fixed_on_screen {
            continue;
        }

        particles[i].velocity.x += accelerations[i].x;
        particles[i].velocity.y += accelerations[i].y;

        update_particle_position(&mut particles[i]);
    }
}
