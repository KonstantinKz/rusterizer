mod camera;
mod raster;
mod tests;
mod texture;
mod transform;
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

    let mut rot: f32 = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        screen.clear();

        //_test_indices(&mut screen);
        //_test_coords(&mut screen);
        //_test_edge_function(&mut screen);
        //_test_triangle1(&mut screen);
        //_test_triangle2(&mut screen);
        //_test_barycentric(&mut screen);
        //_test_textured_triangle(&mut screen);
        //_test_textured_quad(&mut screen);
        //_test_camera(&mut screen, &mut rot);
        //_test_raster_mesh(&mut screen);
        _test_textured_cube(&mut screen, &mut rot);

        window
            .update_with_buffer(&screen.data, WIDTH, HEIGHT)
            .unwrap();
    }
}
