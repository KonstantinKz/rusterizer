pub mod geometry {
    use glam::{Mat4, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};

    // required for the extend_from_slice in the add_section_from_vertices
    #[derive(Debug, Copy, Clone)]
    pub struct Vertex {
        pub position: Vec4,
        pub color: Vec3,
        pub uv: Vec2,
    }

    #[derive(Copy, Clone)]
    pub struct Triangle {
        pub v: [Vertex; 3],
    }

    impl Triangle {
        pub fn transform(&self, matrix: &Mat4) -> Self {
            let mut result = *self;
            result.v[0].position = *matrix * self.v[0].position.xyz().extend(1.0);
            result.v[1].position = *matrix * self.v[1].position.xyz().extend(1.0);
            result.v[2].position = *matrix * self.v[2].position.xyz().extend(1.0);
            result
        }
    }

    pub enum ClipResult {
        None,
        One(Triangle),
    }

    pub struct Mesh {
        pub triangles: Vec<UVec3>,
        pub vertices: Vec<Vertex>,
    }

    impl Mesh {
        pub fn create() -> Self {
            Self {
                triangles: Vec::new(),
                vertices: Vec::new(),
            }
        }

        pub fn get_triangles(&self) -> &Vec<UVec3> {
            &self.triangles
        }

        pub fn _get_vertices(&self) -> &Vec<Vertex> {
            &self.vertices
        }

        pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [&Vertex; 3] {
            [
                &self.vertices[triangle.x as usize],
                &self.vertices[triangle.y as usize],
                &self.vertices[triangle.z as usize],
            ]
        }

        pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
            let offset = self.vertices.len() as u32;
            let triangles: Vec<UVec3> = triangles.iter().map(|index| *index + offset).collect();
            self.triangles.extend_from_slice(&triangles);
            self.vertices.extend_from_slice(vertices);
        }

        pub fn from_vertices(triangles: &[UVec3], vertices: &[Vertex]) -> Self {
            let mut mesh = Mesh::create();
            mesh.add_section_from_vertices(triangles, vertices);
            mesh
        }
    }

    impl Default for Mesh {
        fn default() -> Self {
            Self::create()
        }
    }

    pub struct BoundingBox2D {
        pub left: f32,
        pub right: f32,
        pub top: f32,
        pub bottom: f32,
    }

    pub fn get_triangle_bounding_box_2d(positions: &[Vec2; 3]) -> BoundingBox2D {
        let left = positions[0].x.min(positions[1].x).min(positions[2].x);
        let right = positions[0].x.max(positions[1].x).max(positions[2].x);
        let top = positions[0].y.min(positions[1].y).min(positions[2].y);
        let bottom = positions[0].y.max(positions[1].y).max(positions[2].y);

        BoundingBox2D {
            left,
            right,
            top,
            bottom,
        }
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
    pub fn from_index_coords(index: usize, width: usize) -> (usize, usize) {
        (index % width, index / width)
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
        let ef0 = edge_function(pos, v1, v2);
        let ef1 = edge_function(pos, v2, v0);
        let ef2 = edge_function(pos, v0, v1);

        let res = 1.0 / area;
        Vec3::new(ef0 * res, ef1 * res, ef2 * res)
    }

    pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Add<Output = T>
            + Copy,
    {
        b1 + (v - a1) * (b2 - b1) / (a2 - a1)
    }
}
