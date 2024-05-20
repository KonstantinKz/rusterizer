pub mod geometry {
    use glam::{UVec3, Vec2, Vec3};

    // required for the extend_from_slice in the add_section_from_vertices
    #[derive(Clone)]
    pub struct Vertex {
        pub position: Vec3,
        pub color: Vec3,
        pub uv: Vec2,
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
