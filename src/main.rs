mod raster;
mod tests;
mod texture;

use minifb::{Key, Window, WindowOptions};
use raster::{structures::*, utils::*};
use std::path::Path;
use tests::tests::*;
use texture::Texture;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

enum Program {
    IndexTest = 0,
    CoordsTest = 1,
    EdgeFuncTest = 2,
    TriangleTest1 = 3,
    TriangleTest2 = 4,
    TestBarycentric = 5,
    TestTexturedTri = 6,
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let current_program = Program::TestTexturedTri;

    // Nice triangle
    let v0 = Vertex2d {
        position: glam::vec2(100.0, 100.0),
        color: glam::vec3(0.0, 1.0, 0.0),
    };

    let v1 = Vertex2d {
        position: glam::vec2(250.0, 400.0),
        color: glam::vec3(1.0, 0.0, 0.0),
    };

    let v2 = Vertex2d {
        position: glam::vec2(400.0, 100.0),
        color: glam::vec3(0.0, 0.0, 1.0),
    };

    let v4 = Vertex3d {
        position: glam::vec3(100.0, 100.0, 1.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };

    let v5 = Vertex3d {
        position: glam::vec3(250.0, 400.0, 1.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(1.0, 0.0),
    };

    let v6 = Vertex3d {
        position: glam::vec3(400.0, 100.0, 1.0),
        color: glam::vec3(0.0, 0.0, 1.0),
        uv: glam::vec2(0.5, 1.0),
    };

    // Area for barycentric test
    let area = edge_function(v0.position, v1.position, v2.position);

    // Texture
    let texture = Texture::load(Path::new("assets/bojan.jpg"));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = from_index_coords(i, WIDTH) + 0.5;

            let color = match current_program {
                Program::IndexTest => _test_index(i, WIDTH, HEIGHT),
                Program::CoordsTest => _test_coords(coords, WIDTH, HEIGHT),
                Program::EdgeFuncTest => _test_edge_function(coords, v0.position, v1.position),
                Program::TriangleTest1 => {
                    _test_triangle1(coords, v0.position, v1.position, v2.position)
                }
                Program::TriangleTest2 => {
                    _test_triangle2(coords, v0.position, v1.position, v2.position)
                }
                Program::TestBarycentric => _test_barycentric(coords, &v0, &v1, &v2, area),
                Program::TestTexturedTri => {
                    _test_textured_triangle(coords, &v4, &v5, &v6, area, &texture)
                }
            };

            *pixel = color;
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
