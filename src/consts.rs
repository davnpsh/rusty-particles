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
pub const MINIMUM_MASS: f32 = 10.0;
// pub const DIMENSIONS: usize = 2;
#[allow(dead_code)]
pub const PARTICLES_QUANTITY: i32 = 100;
pub const RESTITUTION: f32 = 0.8;
