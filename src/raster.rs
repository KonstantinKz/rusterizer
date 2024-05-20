use crate::texture::Texture;
use crate::utils::{geometry::*, utils::*};
use glam::{Vec2, Vec3, Vec3Swizzles};

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub z_buffer: Vec<f32>,
}

impl Screen {
    // creates necessary buffers
    pub fn create(screen_width: usize, screen_height: usize) -> Self {
        Self {
            width: screen_width,
            height: screen_height,
            data: vec![0; screen_width * screen_height],
            z_buffer: vec![f32::INFINITY; screen_width * screen_height],
        }
    }

    // clears the screen
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    // outputs indicies as a color
    pub fn output_index(&mut self) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let index_as_color = i as f32 / (self.width * self.height) as f32;
            let index_as_color = (index_as_color * 255.0) as u8;
            *pixel = from_u8_rgb(index_as_color, index_as_color, index_as_color)
        }
    }

    // outputs coords as a color
    pub fn output_coords(&mut self) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            *pixel = from_u8_rgb(
                (coords.x * 255.0 / self.width as f32) as u8,
                (coords.y * 255.0 / self.height as f32) as u8,
                0,
            )
        }
    }

    // tests edge function with green and red colors
    pub fn output_edge_function(&mut self, v0: Vec2, v1: Vec2) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let mut color: u32 = 0;
            let edge_func = edge_function(coords, v0, v1);

            if edge_func > 0.0 {
                color = from_u8_rgb(255, 0, 0);
            } else if edge_func < 0.0 {
                color = from_u8_rgb(0, 255, 0);
            }

            *pixel = color;
        }
    }

    // tests triangle with green and red colors
    pub fn output_triangle1(&mut self, v0: Vec2, v1: Vec2, v2: Vec2) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let ef0 = edge_function(coords, v1, v2);
            let ef1 = edge_function(coords, v2, v0);
            let ef2 = edge_function(coords, v0, v1);

            if ef0 > 0.0 && ef1 > 0.0 && ef2 > 0.0 || ef0 < 0.0 && ef1 < 0.0 && ef2 < 0.0 {
                *pixel = from_u8_rgb(0, 255, 0);
            } else {
                *pixel = from_u8_rgb(255, 0, 0);
            }
        }
    }

    // tests triangle with edge function outputs
    pub fn output_triangle2(&mut self, v0: Vec2, v1: Vec2, v2: Vec2) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let ef0 = edge_function(coords, v0, v1);
            let ef1 = edge_function(coords, v1, v2);
            let ef2 = edge_function(coords, v2, v0);

            *pixel = from_u8_rgb(
                (ef0 * 255.0) as u8,
                (ef1 * 255.0) as u8,
                (ef2 * 255.0) as u8,
            );
        }
    }

    // outputs barycentric coordinates
    pub fn output_barycentric(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex) {
        let triangle_area = edge_function(v0.position.xy(), v1.position.xy(), v2.position.xy());

        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let coords = glam::vec2(coords.x as f32, coords.y as f32) + 0.5;

            let barycentric = barycentric_cordinates(
                coords,
                v0.position.xy(),
                v1.position.xy(),
                v2.position.xy(),
                triangle_area,
            );

            let mut color: Vec3 = glam::vec3(0.0, 0.0, 0.0);
            if barycentric.x > 0.0 && barycentric.y > 0.0 && barycentric.z > 0.0 {
                color =
                    barycentric.x * v0.color + barycentric.y * v1.color + barycentric.z * v2.color;
            }
            *pixel = from_rgb_u32(color);
        }
    }

    // rasterize triangle
    pub fn raster_triangle(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex, texture: &Texture) {
        let triangle_area = edge_function(v0.position.xy(), v1.position.xy(), v2.position.xy());

        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let coords = glam::vec2(coords.x as f32, coords.y as f32) + 0.5;

            let barycentric = barycentric_cordinates(
                coords,
                v0.position.xy(),
                v1.position.xy(),
                v2.position.xy(),
                triangle_area,
            );

            if barycentric.x > 0.0 && barycentric.y > 0.0 && barycentric.z > 0.0 {
                let depth = barycentric.x * v0.position.z
                    + barycentric.y * v1.position.z
                    + barycentric.z * v2.position.z;
                if depth < self.z_buffer[i] {
                    self.z_buffer[i] = depth;

                    let tex_coords =
                        barycentric.x * v0.uv + barycentric.y * v1.uv + barycentric.z * v2.uv;
                    let color = texture.sample_at_uv(tex_coords.x, tex_coords.y);
                    *pixel = color;
                }
            }
        }
    }
}
