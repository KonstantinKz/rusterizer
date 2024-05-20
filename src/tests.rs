pub mod tests {
    use crate::raster::structures::*;
    use crate::raster::utils::*;
    use crate::texture::*;
    use glam::{Vec2, Vec3, Vec3Swizzles};

    // outputs indicies as a color
    pub fn _test_index(index: usize, width: usize, height: usize) -> u32 {
        let index_as_color = index as f32 / (width * height) as f32;
        let index_as_color = (index_as_color * 255.0) as u8;
        from_u8_rgb(index_as_color, index_as_color, index_as_color)
    }

    // outputs coords as a color
    pub fn _test_coords(coords: Vec2, width: usize, height: usize) -> u32 {
        from_u8_rgb(
            (coords.x * 255.0 / width as f32) as u8,
            (coords.y * 255.0 / height as f32) as u8,
            0,
        )
    }

    // tests edge function with green and red colors
    pub fn _test_edge_function(pos: Vec2, v0: Vec2, v1: Vec2) -> u32 {
        let mut color: u32 = 0;
        let edge_func = edge_function(pos, v0, v1);

        if edge_func > 0.0 {
            color = from_u8_rgb(255, 0, 0);
        } else if edge_func < 0.0 {
            color = from_u8_rgb(0, 255, 0);
        }

        color
    }

    // tests triangle with green and red colors
    pub fn _test_triangle1(pos: Vec2, v0: Vec2, v1: Vec2, v2: Vec2) -> u32 {
        let mut color: u32 = from_u8_rgb(255, 0, 0);
        let ef0 = edge_function(pos, v1, v2);
        let ef1 = edge_function(pos, v2, v0);
        let ef2 = edge_function(pos, v0, v1);

        if ef0 > 0.0 && ef1 > 0.0 && ef2 > 0.0 || ef0 < 0.0 && ef1 < 0.0 && ef2 < 0.0 {
            color = from_u8_rgb(0, 255, 0);
        }

        color
    }

    // tests triangle with edge function outputs
    pub fn _test_triangle2(pos: Vec2, v0: Vec2, v1: Vec2, v2: Vec2) -> u32 {
        let ef0 = edge_function(pos, v0, v1);
        let ef1 = edge_function(pos, v1, v2);
        let ef2 = edge_function(pos, v2, v0);

        from_u8_rgb(
            (ef0 * 255.0) as u8,
            (ef1 * 255.0) as u8,
            (ef2 * 255.0) as u8,
        )
    }

    pub fn _test_barycentric_vec2(pos: Vec2, v0: Vec2, v1: Vec2, v2: Vec2, area: f32) -> u32 {
        let barycentric = barycentric_cordinates(pos, v0, v1, v2, area);
        from_rgb_u32(barycentric)
    }

    pub fn _test_barycentric(
        pos: Vec2,
        v0: &Vertex2d,
        v1: &Vertex2d,
        v2: &Vertex2d,
        area: f32,
    ) -> u32 {
        let barycentric = barycentric_cordinates(pos, v0.position, v1.position, v2.position, area);
        let mut color: Vec3 = glam::vec3(0.0, 0.0, 0.0);
        if barycentric.x > 0.0 && barycentric.y > 0.0 && barycentric.z > 0.0 {
            color = barycentric.x * v0.color + barycentric.y * v1.color + barycentric.z * v2.color;
        }

        from_rgb_u32(color)
    }

    pub fn _test_textured_triangle(
        pos: Vec2,
        v0: &Vertex3d,
        v1: &Vertex3d,
        v2: &Vertex3d,
        area: f32,
        tex: &Texture,
    ) -> u32 {
        let barycentric = barycentric_cordinates(
            pos,
            v0.position.xy(),
            v1.position.xy(),
            v2.position.xy(),
            area,
        );
        let mut color: u32 = 0;
        if barycentric.x > 0.0 && barycentric.y > 0.0 && barycentric.z > 0.0 {
            let tex_coords = barycentric.x * v0.uv + barycentric.y * v1.uv + barycentric.z * v2.uv;

            color = tex.sample_at_uv(tex_coords.x, tex_coords.y);
        }

        color
    }
}
