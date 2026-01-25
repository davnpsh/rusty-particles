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
	let mut accelerations = vec![types::Vector { x: 0.0, y: 0.0 }; particles.len()];

    // compute all forces
    for i in 0..particles.len() {
        let mut total_force = types::Vector { x: 0.0, y: 0.0 };

        for j in 0..particles.len() {
            if i != j {
                let force = calculate_g_force(&particles[i], &particles[j]);

                total_force.x += force.x;
                total_force.y += force.y;
            }
        }

        accelerations[i].x = total_force.x / particles[i].mass;
        accelerations[i].y = total_force.y / particles[i].mass;
    }

    // update positions
    for i in 0..particles.len() {
        particles[i].velocity.x += accelerations[i].x;
        particles[i].velocity.y += accelerations[i].y;

        update_particle_position(&mut particles[i]);
    }
}
