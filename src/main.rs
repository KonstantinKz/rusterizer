mod raster;
use glam::Vec2;
use minifb::{Key, Window, WindowOptions};
use raster::{
    _test_coords, _test_edge_function, _test_index, _test_triangle1, _test_triangle2,
    from_index_coords,
};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

enum Program {
    IndexTest = 0,
    CoordsTest = 1,
    EdgeFuncTest = 2,
    TriangleTest1 = 3,
    TriangleTest2 = 4,
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let current_program = Program::TriangleTest1;

    // Nice triangle
    let v0 = Vec2::new(250.0, 30.0);
    let v1 = Vec2::new(150.0, 90.0);
    let v2 = Vec2::new(50.0, 30.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = from_index_coords(i, WIDTH);

            let color = match current_program {
                Program::IndexTest => _test_index(i, WIDTH, HEIGHT),
                Program::CoordsTest => _test_coords(coords, WIDTH, HEIGHT),
                Program::EdgeFuncTest => _test_edge_function(coords, v0, v1),
                Program::TriangleTest1 => _test_triangle1(coords, v0, v1, v2),
                Program::TriangleTest2 => _test_triangle2(coords, v0, v1, v2),
            };

            *pixel = color;
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
