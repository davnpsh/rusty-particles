mod consts;

use ::rand::Rng;
use macroquad::{prelude::*, window};

type Vector = [f32; consts::DIMENSIONS];

struct Particle {
    mass: f32,
    position: Vector,
    velocity: Vector,
}

fn update_particle_position(p: &mut Particle) {
    for i in 0..consts::DIMENSIONS {
        p.position[i] += p.velocity[i];
    }
}

fn calculate_distance_vector(a: &Particle, b: &Particle) -> Vector {
    let mut distance = [0.0; consts::DIMENSIONS];

    for i in 0..distance.len() {
        distance[i] = b.position[i] - a.position[i];
    }

    return distance;
}

fn calculate_g_force(a: &Particle, b: &Particle) -> Vector {
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

fn apply_physics(particles: &mut Vec<Particle>) {
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

fn generate_random_particles(n: i32, particles: &mut Vec<Particle>) {
    let mut rng = ::rand::rng();

    particles.reserve(n as usize);

    for _ in 0..n {
        // WARNING: Specific to 2D space!!!!
        let position: Vector = [
            rng.random_range(
                (consts::WINDOW_WIDTH / 5.0)..(consts::WINDOW_WIDTH - consts::WINDOW_WIDTH / 5.0),
            ),
            rng.random_range(
                (consts::WINDOW_HEIGHT / 5.0)
                    ..(consts::WINDOW_HEIGHT - consts::WINDOW_HEIGHT / 5.0),
            ),
        ];

        // WARNING: Specific to 2D space!!!!
        let velocity: Vector = [0.0, 0.0];

        let particle = Particle {
            mass: rng.random_range(5.0..60.0),
            position: position,
            velocity: velocity,
        };

        particles.push(particle);
    }
}

// WARNING: Specific to 2D space!!!!
fn draw_particles(particles: &Vec<Particle>) {
    for i in 0..particles.len() {
        draw_circle(
            particles[i].position[0],
            particles[i].position[1],
            particles[i].mass,
            WHITE,
        );
    }
}

fn draw_grid() {
    let (mut x1, mut y1): (f32, f32);
    let (mut x2, mut y2): (f32, f32);
    let mut i: f32;

    // vertical lines
    i = consts::CELL_SIZE;
    while i < consts::WINDOW_WIDTH {
        (x1, y1) = (i, 0.0);
        (x2, y2) = (i, consts::WINDOW_HEIGHT);

        draw_line(x1, y1, x2, y2, 1.0, GRAY);

        i += consts::CELL_SIZE;
    }

    // horizontal lines
    i = consts::CELL_SIZE;
    while i < consts::WINDOW_HEIGHT {
        (x1, y1) = (0.0, i);
        (x2, y2) = (consts::WINDOW_WIDTH, i);

        draw_line(x1, y1, x2, y2, 1.0, GRAY);

        i += consts::CELL_SIZE;
    }
}

fn window_configuration() -> window::Conf {
    window::Conf {
        window_title: String::from("Rusty Particles"),
        window_width: consts::WINDOW_WIDTH as i32,
        window_height: consts::WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();

    generate_random_particles(consts::PARTICLES_QUANTITY, &mut particles);

    loop {
        clear_background(BLACK);
        draw_grid();

        apply_physics(&mut particles);
        draw_particles(&particles);

        next_frame().await;
    }
}
