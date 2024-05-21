use crate::texture::Texture;
use crate::utils::{geometry::*, utils::*};
use glam::{Mat4, UVec3, Vec2, Vec3, Vec4Swizzles};

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
    pub fn process_vertex(v: &Vertex, viewport: Vec2) -> (Vertex, f32) {
        let rec = 1.0 / v.position.w;

        let uv = v.uv * rec;
        let color = v.color * rec;
        let ndc = v.position * rec;

        (
            Vertex {
                position: glam::vec4(
                    map_to_range(ndc.x, -1.0, 1.0, 0.0, viewport.x),
                    map_to_range(-ndc.y, -1.0, 1.0, 0.0, viewport.y),
                    ndc.z,
                    1.0,
                ),
                color: color,
                uv: uv,
            },
            rec,
        )
    }

    pub fn get_screen_vertices(&self, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> [(Vertex, f32); 3] {
        let viewport_size = glam::vec2(self.width as f32, self.height as f32);
        [
            Self::process_vertex(v0, viewport_size),
            Self::process_vertex(v1, viewport_size),
            Self::process_vertex(v2, viewport_size),
        ]
    }

    // bounding box for triangles in screen space
    pub fn triangle_screen_bounding_box(
        positions: &[Vec2; 3],
        viewport_size: Vec2,
    ) -> Option<BoundingBox2D> {
        let bb = get_triangle_bounding_box_2d(positions);

        if bb.left >= viewport_size.x
            || bb.right < 0.0
            || bb.bottom >= viewport_size.y
            || bb.top < 0.0
        {
            None
        } else {
            let left = bb.left.max(0.0);
            let right = bb.right.min(viewport_size.x - 1.0);
            let bottom = bb.bottom.max(0.0);
            let top = bb.top.min(viewport_size.y - 1.0);

            Some(BoundingBox2D {
                left,
                right,
                top,
                bottom,
            })
        }
    }

    pub fn view_frustum_culling(triangle: &Triangle) -> ClipResult {
        // X
        if triangle.v[0].position.x > triangle.v[0].position.w
            && triangle.v[1].position.x > triangle.v[1].position.w
            && triangle.v[2].position.x > triangle.v[2].position.w
        {
            return ClipResult::None;
        }
        if triangle.v[0].position.x < -triangle.v[0].position.w
            && triangle.v[1].position.x < -triangle.v[1].position.w
            && triangle.v[2].position.x < -triangle.v[2].position.w
        {
            return ClipResult::None;
        }
        // Y
        if triangle.v[0].position.y > triangle.v[0].position.w
            && triangle.v[1].position.y > triangle.v[1].position.w
            && triangle.v[2].position.y > triangle.v[2].position.w
        {
            return ClipResult::None;
        }
        if triangle.v[0].position.y < -triangle.v[0].position.w
            && triangle.v[1].position.y < -triangle.v[1].position.w
            && triangle.v[2].position.y < -triangle.v[2].position.w
        {
            return ClipResult::None;
        }
        // Z
        if triangle.v[0].position.z > triangle.v[0].position.w
            && triangle.v[1].position.z > triangle.v[1].position.w
            && triangle.v[2].position.z > triangle.v[2].position.w
        {
            return ClipResult::None;
        }
        if triangle.v[0].position.z < 0.0
            && triangle.v[1].position.z < 0.0
            && triangle.v[2].position.z < 0.0
        {
            return ClipResult::None;
        }

        ClipResult::One(*triangle)
    }

    // rasterize textured triangle
    pub fn raster_clipped_triangle(
        &mut self,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
        texture: Option<&Texture>,
    ) {
        let vertex = self.get_screen_vertices(v0, v1, v2);

        let triangle_area = edge_function(
            vertex[0].0.position.xy(),
            vertex[1].0.position.xy(),
            vertex[2].0.position.xy(),
        );

        let viewport_size = glam::vec2(self.width as f32, self.height as f32);
        if let Some(bb) = Self::triangle_screen_bounding_box(
            &[
                vertex[0].0.position.xy(),
                vertex[1].0.position.xy(),
                vertex[2].0.position.xy(),
            ],
            viewport_size,
        ) {
            for y in (bb.top as usize)..(bb.bottom as usize) {
                for x in (bb.left as usize)..(bb.right as usize) {
                    let coords = glam::vec2(x as f32, y as f32) + 0.5;
                    let pixel_id = from_coords_index(coords, viewport_size.x as usize);

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
                        if depth < self.z_buffer[pixel_id] {
                            self.z_buffer[pixel_id] = depth;

                            if let Some(tex) = texture {
                                let tex_coords = barycentric.x * vertex[0].0.uv
                                    + barycentric.y * vertex[1].0.uv
                                    + barycentric.z * vertex[2].0.uv;
                                let tex_coords = tex_coords * correction;
                                self.data[pixel_id] = tex.sample_at_uv(tex_coords.x, tex_coords.y);
                            } else {
                                let color = barycentric.x * vertex[0].0.color
                                    + barycentric.y * vertex[1].0.color
                                    + barycentric.z * vertex[2].0.color;
                                self.data[pixel_id] = from_rgb_u32(color * correction);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn raster_triangle(
        &mut self,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
        texture: Option<&Texture>,
        mvp: &Mat4,
    ) {
        let triangle = Triangle { v: [*v0, *v1, *v2] };
        let clip_tri = triangle.transform(mvp);

        match Self::view_frustum_culling(&clip_tri) {
            ClipResult::None => {
                println!("fully clipped!");
            }
            ClipResult::One(tri) => {
                self.raster_clipped_triangle(&tri.v[0], &tri.v[1], &tri.v[2], texture);
            }
        }
    }

    pub fn raster_mesh(&mut self, mesh: &Mesh, mvp: &Mat4, texture: Option<&Texture>) {
        for indecies in mesh.get_triangles() {
            let vertices = mesh.get_vertices_from_triangle(*indecies);

            self.raster_triangle(vertices[0], vertices[1], vertices[2], texture, mvp);
        }
    }
}
