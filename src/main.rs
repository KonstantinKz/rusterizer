mod raster;
mod tests;
mod texture;
mod utils;

use minifb::{Key, Window, WindowOptions};
use raster::Screen;
use tests::tests::*;
use utils::utils::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
    let mut screen = Screen::create(WIDTH, HEIGHT);

    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    _test_textured_quad(&mut screen);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&screen.data, WIDTH, HEIGHT)
            .unwrap();
    }
}
