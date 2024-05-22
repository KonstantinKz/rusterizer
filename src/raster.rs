use crate::texture::Texture;
use crate::utils::{geometry::*, utils::*};
use glam::{Mat4, Vec2, Vec3, Vec4Swizzles};

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub z_buffer: Vec<f32>,
}

pub enum ClipResult {
    None,
    One(Triangle),
    Two((Triangle, Triangle)),
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

            let barycentric = barycentric_coordinates(
                coords,
                v0.position.xy(),
                v1.position.xy(),
                v2.position.xy(),
                triangle_area,
            );

            let mut color: Vec3 = glam::vec3(0.0, 0.0, 0.0);
            if let Some(bary) = barycentric {
                color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
            }
            *pixel = from_rgb_u32(color);
        }
    }

    // Culling

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

    pub fn view_frustum_culling(triangle: &Triangle) -> bool {
        // X
        if triangle.v0.position.x > triangle.v0.position.w
            && triangle.v1.position.x > triangle.v1.position.w
            && triangle.v2.position.x > triangle.v2.position.w
        {
            return true;
        }
        if triangle.v0.position.x < -triangle.v0.position.w
            && triangle.v1.position.x < -triangle.v1.position.w
            && triangle.v2.position.x < -triangle.v2.position.w
        {
            return true;
        }
        //Y
        if triangle.v0.position.y > triangle.v0.position.w
            && triangle.v1.position.y > triangle.v1.position.w
            && triangle.v2.position.y > triangle.v2.position.w
        {
            return true;
        }
        if triangle.v0.position.y < -triangle.v0.position.w
            && triangle.v1.position.y < -triangle.v1.position.w
            && triangle.v2.position.y < -triangle.v2.position.w
        {
            return true;
        }
        //Z
        if triangle.v0.position.z > triangle.v0.position.w
            && triangle.v1.position.z > triangle.v1.position.w
            && triangle.v2.position.z > triangle.v2.position.w
        {
            return true;
        }
        if triangle.v0.position.z < 0.0
            && triangle.v1.position.z < 0.0
            && triangle.v2.position.z < 0.0
        {
            return true;
        }

        false
    }

    pub fn clip_triangle_two(triangle: &Triangle) -> (Triangle, Triangle) {
        // calculate alpha values for getting adjusted vertices
        let alpha_a = (-triangle.v0.position.z) / (triangle.v1.position.z - triangle.v0.position.z);
        let alpha_b = (-triangle.v0.position.z) / (triangle.v2.position.z - triangle.v0.position.z);

        // interpolate to get v0a and v0b
        let v0_a = lerp(triangle.v0, triangle.v1, alpha_a);
        let v0_b = lerp(triangle.v0, triangle.v2, alpha_b);

        // draw triangles
        let mut result_a = *triangle;
        let mut result_b = *triangle;

        result_a.v0 = v0_a;

        result_b.v0 = v0_a;
        result_b.v1 = v0_b;

        let green = Vec3::new(0.0, 1.0, 0.0);
        let blue = Vec3::new(0.0, 0.0, 1.0);

        result_a.v0.color = green;
        result_a.v1.color = green;
        result_a.v2.color = green;
        result_b.v0.color = blue;
        result_b.v1.color = blue;
        result_b.v2.color = blue;

        (result_a, result_b)
    }

    pub fn clip_triangle_one(triangle: &Triangle) -> Triangle {
        // calculate alpha values for getting adjusted vertices
        let alpha_a = (-triangle.v0.position.z) / (triangle.v2.position.z - triangle.v0.position.z);
        let alpha_b = (-triangle.v1.position.z) / (triangle.v2.position.z - triangle.v1.position.z);

        // interpolate to get v0a and v0b
        let mut v0 = lerp(triangle.v0, triangle.v2, alpha_a);
        let mut v1 = lerp(triangle.v1, triangle.v2, alpha_b);

        let mut v2 = triangle.v2;

        let red = Vec3::new(1.0, 0.0, 0.0);

        v0.color = red;
        v1.color = red;
        v2.color = red;

        Triangle { v0, v1, v2 }
    }

    pub fn cull_triangle_backface(triangle: &Triangle) -> bool {
        let normal = (triangle.v1.position.xyz() - triangle.v0.position.xyz())
            .cross(triangle.v2.position.xyz() - triangle.v0.position.xyz());
        // any is vertex valid
        let view_dir = -Vec3::Z;
        // also we don't care about normalizing
        // if negative facing the camera
        normal.dot(view_dir) >= 0.0
    }

    pub fn clip_cull_triangle(triangle: &Triangle) -> ClipResult {
        if Self::cull_triangle_backface(triangle) {
            return ClipResult::None;
        }
        if Self::view_frustum_culling(triangle) {
            ClipResult::None
        } else {
            // clipping routines
            if triangle.v0.position.z < 0.0 {
                if triangle.v1.position.z < 0.0 {
                    ClipResult::One(Self::clip_triangle_one(triangle))
                } else if triangle.v2.position.z < 0.0 {
                    ClipResult::One(Self::clip_triangle_one(
                        &triangle.reorder(VerticesOrder::ACB),
                    ))
                } else {
                    ClipResult::Two(Self::clip_triangle_two(
                        &triangle.reorder(VerticesOrder::ACB),
                    ))
                }
            } else if triangle.v1.position.z < 0.0 {
                if triangle.v2.position.z < 0.0 {
                    ClipResult::One(Self::clip_triangle_one(
                        &triangle.reorder(VerticesOrder::BCA),
                    ))
                } else {
                    ClipResult::Two(Self::clip_triangle_two(
                        &triangle.reorder(VerticesOrder::BAC),
                    ))
                }
            } else if triangle.v2.position.z < 0.0 {
                ClipResult::Two(Self::clip_triangle_two(
                    &triangle.reorder(VerticesOrder::CBA),
                ))
            } else {
                // no near clipping necessary
                //return original
                ClipResult::One(*triangle)
            }
        }
    }

    // rasterize textured triangle
    pub fn raster_clipped_triangle(&mut self, clip_triangle: &Triangle, texture: Option<&Texture>) {
        let viewport_size = glam::vec2(self.width as f32, self.height as f32);

        let rec0 = 1.0 / clip_triangle.v0.position.w;
        let rec1 = 1.0 / clip_triangle.v1.position.w;
        let rec2 = 1.0 / clip_triangle.v2.position.w;

        // This would be the output of the vertex shader (clip space)
        // then we perform perspective division to transform in ndc
        // now x,y,z componend of ndc are between -1 and 1
        let ndc0 = clip_triangle.v0.position * rec0;
        let ndc1 = clip_triangle.v1.position * rec1;
        let ndc2 = clip_triangle.v2.position * rec2;

        // perspective division on all attributes
        let v0 = clip_triangle.v0 * rec0;
        let v1 = clip_triangle.v1 * rec1;
        let v2 = clip_triangle.v2 * rec2;

        // screeen coordinates remapped to window
        let sc0 = glam::vec2(
            map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc1 = glam::vec2(
            map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc2 = glam::vec2(
            map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
        );

        if let Some(bb) = Self::triangle_screen_bounding_box(&[sc0, sc1, sc2], viewport_size) {
            for y in (bb.top as usize)..=bb.bottom as usize {
                for x in (bb.left as usize)..=bb.right as usize {
                    let coords = glam::vec2(x as f32, y as f32) + 0.5;
                    let pixel_id = from_coords_index(coords, viewport_size.x as usize);
                    let area = edge_function(sc0, sc1, sc2);

                    if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
                        let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                        let correction = 1.0 / correction;
                        let depth = bary.x * ndc0.z + bary.y * ndc1.z + bary.z * ndc2.z;
                        if depth < self.z_buffer[pixel_id] {
                            self.z_buffer[pixel_id] = depth;

                            let normal =
                                bary.x * v0.normal + bary.y * v1.normal + bary.z * v2.normal;
                            let normal = normal * correction;
                            let n_dot_l = normal.dot(Vec3::ONE.normalize());

                            let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                            let mut output = color * correction;
                            if let Some(tex) = texture {
                                let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                                let tex_coords = tex_coords * correction;
                                output = tex.sample_at_uv_rgb(tex_coords.x, tex_coords.y);
                            }

                            let ambient = glam::vec3(0.2, 0.2, 0.2);
                            output = output * n_dot_l + ambient;
                            self.data[pixel_id] = from_rgb_u32(output);
                        }
                    }
                }
            }
        }
    }

    pub fn raster_triangle(
        &mut self,
        vertices: &[&Vertex; 3],
        texture: Option<&Texture>,
        mvp: &Mat4,
        model: &Mat4,
    ) {
        let cof_model = cofactor(model);
        let triangle = Triangle {
            v0: *vertices[0],
            v1: *vertices[1],
            v2: *vertices[2],
        };
        let mut clip_tri = triangle.transform(mvp);
        clip_tri.v0.normal = (cof_model * clip_tri.v0.normal.extend(0.0)).xyz();
        clip_tri.v1.normal = (cof_model * clip_tri.v1.normal.extend(0.0)).xyz();
        clip_tri.v2.normal = (cof_model * clip_tri.v2.normal.extend(0.0)).xyz();

        match Self::clip_cull_triangle(&clip_tri) {
            ClipResult::None => {}
            ClipResult::One(tri) => {
                self.raster_clipped_triangle(&tri, texture);
            }
            ClipResult::Two(tri) => {
                self.raster_clipped_triangle(&tri.0, texture);
                self.raster_clipped_triangle(&tri.1, texture);
            }
        }
    }

    pub fn raster_mesh(
        &mut self,
        mesh: &Mesh,
        mvp: &Mat4,
        model: &Mat4,
        texture: Option<&Texture>,
    ) {
        for indecies in mesh.get_triangles() {
            let vertices = mesh.get_vertices_from_triangle(*indecies);

            self.raster_triangle(&vertices, texture, mvp, model);
        }
    }
}
