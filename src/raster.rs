pub mod structures {
    use glam::{Vec2, Vec3};
    //#[derive(Debug, Copy, Clone)]
    pub struct Vertex2d {
        pub position: Vec2,
        pub color: Vec3,
    }

    pub struct Vertex3d {
        pub position: Vec3,
        pub color: Vec3,
        pub uv: Vec2,
    }
}

pub mod utils {
    use glam::{Vec2, Vec3};

    pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn from_rgb_u32(color: Vec3) -> u32 {
        from_u8_rgb(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        )
    }

    // conversion from indives to coordinate system
    pub fn from_index_coords(index: usize, width: usize) -> Vec2 {
        Vec2::new((index % width) as f32, (index / width) as f32)
    }

    // conversion from a coordinate system to indices
    pub fn from_coords_index(pos: Vec2, width: usize) -> usize {
        pos.x as usize + pos.y as usize * width
    }

    pub fn edge_function(pos: Vec2, v0: Vec2, v1: Vec2) -> f32 {
        let seg_a = v1 - v0;
        let seg_b = pos - v0;

        seg_a.x * seg_b.y - seg_a.y * seg_b.x
    }

    pub fn barycentric_cordinates(pos: Vec2, v0: Vec2, v1: Vec2, v2: Vec2, area: f32) -> Vec3 {
        let ef0 = edge_function(pos, v0, v1);
        let ef1 = edge_function(pos, v1, v2);
        let ef2 = edge_function(pos, v2, v0);

        let res = 1.0 / area;
        Vec3::new(ef0 * res, ef1 * res, ef2 * res)
    }
}
