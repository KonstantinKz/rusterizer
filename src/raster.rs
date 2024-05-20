use crate::camera::*;
use crate::texture::Texture;
use crate::transform::Transform;
use crate::utils::{geometry::*, utils::*};
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4};

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
        self.z_buffer.fill(f32::INFINITY);
    }

    // outputs indicies as a color
    pub fn _output_index(&mut self) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let index_as_color = i as f32 / (self.width * self.height) as f32;
            let index_as_color = (index_as_color * 255.0) as u8;
            *pixel = from_u8_rgb(index_as_color, index_as_color, index_as_color)
        }
    }

    // outputs coords as a color
    pub fn _output_coords(&mut self) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            *pixel = from_u8_rgb(
                (coords.0 as f32 * 255.0 / self.width as f32) as u8,
                (coords.1 as f32 * 255.0 / self.height as f32) as u8,
                0,
            )
        }
    }

    // tests edge function with green and red colors
    pub fn _output_edge_function(&mut self, v0: Vec2, v1: Vec2) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let mut color: u32 = 0;
            let edge_func = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v0, v1);

            if edge_func > 0.0 {
                color = from_u8_rgb(255, 0, 0);
            } else if edge_func < 0.0 {
                color = from_u8_rgb(0, 255, 0);
            }

            *pixel = color;
        }
    }

    // tests triangle with green and red colors
    pub fn _output_triangle1(&mut self, v0: Vec2, v1: Vec2, v2: Vec2) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let ef0 = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v1, v2);
            let ef1 = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v2, v0);
            let ef2 = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v0, v1);

            if ef0 > 0.0 && ef1 > 0.0 && ef2 > 0.0 || ef0 < 0.0 && ef1 < 0.0 && ef2 < 0.0 {
                *pixel = from_u8_rgb(0, 255, 0);
            } else {
                *pixel = from_u8_rgb(255, 0, 0);
            }
        }
    }

    // tests triangle with edge function outputs
    pub fn _output_triangle2(&mut self, v0: Vec2, v1: Vec2, v2: Vec2) {
        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let ef0 = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v0, v1);
            let ef1 = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v1, v2);
            let ef2 = edge_function(glam::vec2(coords.0 as f32, coords.1 as f32), v2, v0);

            *pixel = from_u8_rgb(
                (ef0 * 255.0) as u8,
                (ef1 * 255.0) as u8,
                (ef2 * 255.0) as u8,
            );
        }
    }

    // outputs barycentric coordinates
    pub fn _output_barycentric(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex) {
        let triangle_area = edge_function(v0.position.xy(), v1.position.xy(), v2.position.xy());

        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;

            let barycentric = barycentric_cordinates(
                coords,
                v0.position.xy(),
                v1.position.xy(),
                v2.position.xy(),
                triangle_area,
            );

            let mut color: Vec3 = glam::vec3(0.0, 0.0, 0.0);
            if barycentric.x >= 0.0 && barycentric.y >= 0.0 && barycentric.z >= 0.0 {
                color =
                    barycentric.x * v0.color + barycentric.y * v1.color + barycentric.z * v2.color;
            }
            *pixel = from_rgb_u32(color);
        }
    }

    // returns new vertex and reciprocal
    pub fn process_vertex(mvp: Mat4, v: &Vertex, viewport: Vec2) -> (Vertex, f32) {
        let clip = mvp * Vec4::from((v.position, 1.0));
        let rec = 1.0 / clip.w;

        let uv = v.uv * rec;
        let color = v.color * rec;
        let ndc = clip * rec;

        (
            Vertex {
                position: glam::vec3(
                    map_to_range(ndc.x, -1.0, 1.0, 0.0, viewport.x),
                    map_to_range(-ndc.y, -1.0, 1.0, 0.0, viewport.y),
                    ndc.z,
                ),
                color: color,
                uv: uv,
            },
            rec,
        )
    }

    pub fn get_screen_vertices(
        &self,
        cam: &Camera,
        transform: &Mat4,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
    ) -> [(Vertex, f32); 3] {
        let projection = cam.projection();
        let view = cam.view();
        let model = transform;

        let viewport_size = glam::vec2(self.width as f32, self.height as f32);
        let mvp = projection * view * *model;

        [
            Self::process_vertex(mvp, v0, viewport_size),
            Self::process_vertex(mvp, v1, viewport_size),
            Self::process_vertex(mvp, v2, viewport_size),
        ]
    }

    // rasterize textured triangle
    pub fn raster_triangle(
        &mut self,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
        texture: Option<&Texture>,
        camera: Option<&Camera>,
        transform: Option<&Mat4>,
    ) {
        let mut vertex = [(v0.clone(), 1.0), (v1.clone(), 1.0), (v2.clone(), 1.0)];
        if let Some(cam) = camera {
            if let Some(trsfr) = transform {
                vertex = self.get_screen_vertices(cam, trsfr, v0, v1, v2);
            } else {
                vertex =
                    self.get_screen_vertices(cam, &Transform::IDENTITY.get_local(), v0, v1, v2);
            }
        }

        let triangle_area = edge_function(
            vertex[0].0.position.xy(),
            vertex[1].0.position.xy(),
            vertex[2].0.position.xy(),
        );

        for (i, pixel) in self.data.iter_mut().enumerate() {
            let coords = from_index_coords(i, self.width);
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;

            let barycentric = barycentric_cordinates(
                coords,
                vertex[0].0.position.xy(),
                vertex[1].0.position.xy(),
                vertex[2].0.position.xy(),
                triangle_area,
            );

            let correction = barycentric.x * vertex[0].1
                + barycentric.y * vertex[1].1
                + barycentric.z * vertex[2].1;

            let correction = 1.0 / correction;

            if barycentric.x >= 0.0 && barycentric.y >= 0.0 && barycentric.z >= 0.0 {
                let depth = barycentric.x * vertex[0].0.position.z
                    + barycentric.y * vertex[1].0.position.z
                    + barycentric.z * vertex[2].0.position.z;
                if depth < self.z_buffer[i] {
                    self.z_buffer[i] = depth;

                    if let Some(tex) = texture {
                        let tex_coords = barycentric.x * vertex[0].0.uv
                            + barycentric.y * vertex[1].0.uv
                            + barycentric.z * vertex[2].0.uv;
                        let tex_coords = tex_coords * correction;
                        *pixel = tex.sample_at_uv(tex_coords.x, tex_coords.y);
                    } else {
                        let color = barycentric.x * vertex[0].0.color
                            + barycentric.y * vertex[1].0.color
                            + barycentric.z * vertex[2].0.color;
                        *pixel = from_rgb_u32(color * correction);
                    }
                }
            }
        }
    }

    pub fn raster_mesh(
        &mut self,
        mesh: &Mesh,
        transform: Option<&Mat4>,
        texture: Option<&Texture>,
        camera: Option<&Camera>,
    ) {
        for triangle in mesh.get_triangles() {
            let vertices = mesh.get_vertices_from_triangle(*triangle);

            self.raster_triangle(
                vertices[0],
                vertices[1],
                vertices[2],
                texture,
                camera,
                transform,
            );
        }
    }
}
