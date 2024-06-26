mod camera;
mod raster;
mod tests;
mod texture;
mod transform;
mod utils;

use crate::camera::Camera;
use crate::texture::Texture;
use crate::transform::Transform;
use minifb::{Key, Window, WindowOptions};
use raster::Screen;
use std::path::Path;
use tests::tests::*;
use utils::utils::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

pub fn process_camera_inputs(window: &Window, camera: &mut Camera) {
    let mut axis = glam::vec2(0.0, 0.0);

    if window.is_key_down(Key::A) {
        axis.x -= 1.0;
    }
    if window.is_key_down(Key::D) {
        axis.x += 1.0;
    }
    if window.is_key_down(Key::W) {
        axis.y += 1.0;
    }
    if window.is_key_down(Key::S) {
        axis.y -= 1.0;
    }

    camera.transform.translation += camera.transform.right() * camera.speed * axis.x
        + camera.transform.forward() * camera.speed * axis.y;
}

fn main() {
    let mut screen = Screen::create(WIDTH, HEIGHT);

    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut rot: f32 = 0.0;
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = Camera {
        far_plane: 100.0,
        near_plane: 0.1,
        aspect_ratio: aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        speed: 0.5,
        ..Default::default()
    };

    let _texture = Texture::load(Path::new("assets/gltf/Default_albedo.jpg"));
    let _mesh_teapot = load_gltf(Path::new("assets/gltf/teapot.gltf"));
    let _mesh_helmet = load_gltf(Path::new("assets/gltf/DamagedHelmet.gltf"));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        screen.clear();
        process_camera_inputs(&window, &mut camera);

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
        //_test_textured_cube(&mut screen, &mut rot);
        //_test_camera_inputs(&mut screen, &mut rot, &camera);
        //_test_gltf(&mut screen, &mut rot, &camera, &_mesh_teapot);
        _test_gltf_textured(&mut screen, &mut rot, &camera, &_mesh_helmet, &_texture);

        window
            .update_with_buffer(&screen.data, WIDTH, HEIGHT)
            .unwrap();
    }
}
