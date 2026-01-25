use ::rand::Rng;
use macroquad::{prelude::*, window};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const CELL_SIZE: f32 = calculate_grid_cell_size();
const DIMENSIONS: usize = 2;
const PARTICLES_QUANTITY: i32 = 3;

// physics stuff
const G: f32 = 6.67430e-11;

type Vector = [f32; DIMENSIONS];

struct Particle {
    mass: f32,
    position: Vector,
    velocity: Vector,
}

fn update_particle_position(p: &mut Particle) {
    for i in 0..DIMENSIONS {
        p.position[i] += p.velocity[i];
    }
}

fn calculate_distance_vector(a: &Particle, b: &Particle) -> Vector {
    let mut distance = [0.0; DIMENSIONS];

    for i in 0..distance.len() {
        distance[i] = b.position[i] - a.position[i];
    }

    return distance;
}

fn calculate_g_force(a: &Particle, b: &Particle) -> Vector {
    let mut force = [0.0; DIMENSIONS];

    let distance_vector = calculate_distance_vector(a, b);
    let mut distance = 0.0;

    for i in 0..distance_vector.len() {
        distance += distance_vector[i].powi(2);
    }

    distance = distance.sqrt();

    if distance == 0.0 {
        return force;
    }

    let force_magnitude = G * a.mass * b.mass / distance.powi(2);

    for i in 0..DIMENSIONS {
        force[i] = force_magnitude * (distance_vector[i] / distance);
    }

    return force;
}

fn apply_physics(particles: &mut Vec<Particle>) {
    let mut accelerations = vec![[0.0; DIMENSIONS]; particles.len()];

    // compute all forces
    for i in 0..particles.len() {
        let mut total_force = [0.0; DIMENSIONS];

        for j in 0..particles.len() {
            if i != j {
                let force = calculate_g_force(&particles[i], &particles[j]);

                for k in 0..DIMENSIONS {
                    total_force[k] += force[k];
                }
            }
        }

        for k in 0..DIMENSIONS {
            accelerations[i][k] = total_force[k] / particles[i].mass;
        }
    }

    // update positions
    for i in 0..particles.len() {
        for k in 0..DIMENSIONS {
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
            rng.random_range(0.0..WINDOW_WIDTH),
            rng.random_range(0.0..WINDOW_HEIGHT),
        ];

        // WARNING: Specific to 2D space!!!!
        let velocity: Vector = [rng.random_range(0.0..2.0), rng.random_range(0.0..2.0)];

        let particle = Particle {
            mass: rng.random_range(1.0..10.0),
            position: position,
            velocity: velocity,
        };

        particles.push(particle);
    }
}

// WARNING: Specific to 2D space!!!!
fn draw_particles(particles: &Vec<Particle>) {
    for i in 0..particles.len() {
        draw_circle(particles[i].position[0], particles[i].position[1], particles[i].mass, WHITE);
    }
}

const fn calculate_grid_cell_size() -> f32 {
    let mut a = WINDOW_WIDTH;
    let mut b = WINDOW_HEIGHT;
    let mut temp: f32;

    while b != 0.0 {
        temp = b;
        b = a % b;
        a = temp;
    }

    return a;
}

fn draw_grid() {
    let (mut x1, mut y1): (f32, f32);
    let (mut x2, mut y2): (f32, f32);
    let mut i: f32;

    // vertical lines
    i = CELL_SIZE;
    while i < WINDOW_WIDTH {
        (x1, y1) = (i, 0.0);
        (x2, y2) = (i, WINDOW_HEIGHT);

        draw_line(x1, y1, x2, y2, 1.0, GRAY);

        i += CELL_SIZE;
    }

    // horizontal lines
    i = CELL_SIZE;
    while i < WINDOW_HEIGHT {
        (x1, y1) = (0.0, i);
        (x2, y2) = (WINDOW_WIDTH, i);

        draw_line(x1, y1, x2, y2, 1.0, GRAY);

        i += CELL_SIZE;
    }
}

fn window_configuration() -> window::Conf {
    window::Conf {
        window_title: String::from("Rusty Particles"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();

    generate_random_particles(PARTICLES_QUANTITY, &mut particles);

    loop {
        clear_background(BLACK);
        draw_grid();

        apply_physics(&mut particles);
        draw_particles(&particles);

        next_frame().await;
    }
}
