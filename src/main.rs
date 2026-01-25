use macroquad::{prelude::*, window};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const CELL_SIZE: f32 = calculate_grid_cell_size();

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
    loop {
        clear_background(BLACK);
        draw_grid();

        next_frame().await;
    }
}
