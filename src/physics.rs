use crate::consts;
use crate::types;

pub fn update_particle_position(p: &mut types::Particle) {
    for i in 0..consts::DIMENSIONS {
        p.position[i] += p.velocity[i];
    }
}

pub fn calculate_distance_vector(a: &types::Particle, b: &types::Particle) -> types::Vector {
    let mut distance = [0.0; consts::DIMENSIONS];

    for i in 0..distance.len() {
        distance[i] = b.position[i] - a.position[i];
    }

    return distance;
}

pub fn calculate_g_force(a: &types::Particle, b: &types::Particle) -> types::Vector {
    let mut force = [0.0; consts::DIMENSIONS];
    let dead_zone = 5.0;

    let distance_vector = calculate_distance_vector(a, b);
    let mut distance = 0.0;

    for i in 0..distance_vector.len() {
        distance += distance_vector[i].powi(2);
    }

    distance = distance.sqrt();

    // to avoid particles shooting each other out of the screen
    if distance < dead_zone {
        return force;
    }

    let force_magnitude = consts::G * a.mass * b.mass / distance.powi(2);

    for i in 0..consts::DIMENSIONS {
        force[i] = force_magnitude * (distance_vector[i] / distance);
    }

    return force;
}

pub fn apply(particles: &mut Vec<types::Particle>) {
    let mut accelerations = vec![[0.0; consts::DIMENSIONS]; particles.len()];

    // compute all forces
    for i in 0..particles.len() {
        let mut total_force = [0.0; consts::DIMENSIONS];

        for j in 0..particles.len() {
            if i != j {
                let force = calculate_g_force(&particles[i], &particles[j]);

                for k in 0..consts::DIMENSIONS {
                    total_force[k] += force[k];
                }
            }
        }

        for k in 0..consts::DIMENSIONS {
            accelerations[i][k] = total_force[k] / particles[i].mass;
        }
    }

    // update positions
    for i in 0..particles.len() {
        for k in 0..consts::DIMENSIONS {
            particles[i].velocity[k] += accelerations[i][k];
        }

        update_particle_position(&mut particles[i]);
    }
}
