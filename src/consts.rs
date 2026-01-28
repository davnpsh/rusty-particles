pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const CELL_SIZE: f32 = {
    let mut a = WINDOW_WIDTH;
    let mut b = WINDOW_HEIGHT;
    let mut temp: f32;

    while b != 0.0 {
        temp = b;
        b = a % b;
        a = temp;
    }

    a
};

// physics stuff
pub const G: f32 = 1.0;
pub const PARTICLE_MINIMUM_MASS: f32 = 10.0;
pub const DEFAULT_PARTICLES_QUANTITY: i32 = 5;
pub const RESTITUTION: f32 = 0.8;
pub const THROWING_POWER_FACTOR: f32 = 0.2;