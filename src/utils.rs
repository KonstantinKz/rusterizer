pub mod geometry {
    use glam::{Mat4, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

    // Vertex
    #[derive(Debug, Copy, Clone)]
    pub struct Vertex {
        pub position: Vec4,
        pub color: Vec3,
        pub normal: Vec3,
        pub uv: Vec2,
    }

    impl Vertex {
        pub fn create(position: Vec4, color: Vec3, normal: Vec3, uv: Vec2) -> Self {
            Self {
                position,
                color,
                normal,
                uv,
            }
        }
    }

    impl Add for Vertex {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            let position = self.position + rhs.position;
            let color = self.color + rhs.color;
            let normal = self.normal + rhs.normal;
            let uv = self.uv + rhs.uv;
            Self {
                position,
                color,
                normal,
                uv,
            }
        }
    }

    impl Sub for Vertex {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            let position = self.position - rhs.position;
            let color = self.color - rhs.color;
            let normal = self.normal - rhs.normal;
            let uv = self.uv - rhs.uv;
            Self {
                position,
                color,
                normal,
                uv,
            }
        }
    }

    impl Mul<f32> for Vertex {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            let position = self.position * rhs;
            let color = self.color * rhs;
            let normal = self.normal * rhs;
            let uv = self.uv * rhs;
            Self {
                position,
                color,
                normal,
                uv,
            }
        }
    }

    impl MulAssign<f32> for Vertex {
        fn mul_assign(&mut self, rhs: f32) {
            self.position *= rhs;
            self.color *= rhs;
            self.normal *= rhs;
            self.uv *= rhs;
        }
    }

    // Triangle
    #[derive(Debug, Copy, Clone)]
    pub struct Triangle {
        pub v0: Vertex,
        pub v1: Vertex,
        pub v2: Vertex,
    }

    pub enum VerticesOrder {
        ABC,
        ACB,
        BAC,
        BCA,
        CAB,
        CBA,
    }

    impl Triangle {
        pub fn create(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
            Self { v0, v1, v2 }
        }

        pub fn transform(&self, matrix: &Mat4) -> Self {
            let mut result = *self;
            result.v0.position = *matrix * self.v0.position.xyz().extend(1.0);
            result.v1.position = *matrix * self.v1.position.xyz().extend(1.0);
            result.v2.position = *matrix * self.v2.position.xyz().extend(1.0);
            result
        }

        pub fn reorder(&self, order: VerticesOrder) -> Self {
            match order {
                VerticesOrder::ABC => *self,
                VerticesOrder::ACB => Self::create(self.v0, self.v2, self.v1),
                VerticesOrder::BAC => Self::create(self.v1, self.v0, self.v2),
                VerticesOrder::BCA => Self::create(self.v1, self.v2, self.v0),
                VerticesOrder::CAB => Self::create(self.v2, self.v0, self.v1),
                VerticesOrder::CBA => Self::create(self.v2, self.v1, self.v0),
            }
        }
    }

    // Mesh
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

        pub fn get_vertices(&self) -> &Vec<Vertex> {
            &self.vertices
        }

        pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [&Vertex; 3] {
            [
                &self.vertices[triangle.x as usize],
                &self.vertices[triangle.y as usize],
                &self.vertices[triangle.z as usize],
            ]
        }

        pub fn from_vertices(triangles: &[UVec3], vertices: &[Vertex]) -> Self {
            let mut mesh = Mesh::create();
            mesh.add_section_from_vertices(triangles, vertices);
            mesh
        }

        pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
            let offset = self.vertices.len() as u32;
            let triangles: Vec<UVec3> = triangles.iter().map(|index| *index + offset).collect();
            self.triangles.extend_from_slice(&triangles);
            self.vertices.extend_from_slice(vertices);
        }

        pub fn add_section_from_buffers(
            &mut self,
            triangles: &[UVec3],
            positions: &[Vec3],
            colors: &[Vec3],
            normals: &[Vec3],
            uvs: &[Vec2],
        ) {
            self.triangles.extend_from_slice(triangles);

            let has_uvs = !uvs.is_empty();
            let has_colors = !colors.is_empty();
            let has_normals = !normals.is_empty();

            for i in 0..positions.len() {
                let vertex = Vertex::create(
                    positions[i].extend(1.0),
                    if has_colors { colors[i] } else { Vec3::ONE },
                    if has_normals { normals[i] } else { Vec3::ONE },
                    if has_uvs { uvs[i] } else { Vec2::ZERO },
                );
                self.vertices.push(vertex)
            }
        }

        pub fn from_gltf_mesh(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Mesh {
            let mut positions: Vec<Vec3> = Vec::new();
            let mut tex_coords: Vec<Vec2> = Vec::new();
            let mut normals: Vec<Vec3> = Vec::new();
            let mut indices = vec![];

            let mut result = Mesh::create();
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(indices_reader) = reader.read_indices() {
                    indices_reader.into_u32().for_each(|i| indices.push(i));
                }
                if let Some(positions_reader) = reader.read_positions() {
                    positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
                }
                if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                    tex_coord_reader
                        .into_f32()
                        .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
                }
                if let Some(normals_reader) = reader.read_normals() {
                    normals_reader.for_each(|p| normals.push(Vec3::new(p[0], p[1], p[2])));
                }
            }

            let colors: Vec<Vec3> = positions.iter().map(|_| Vec3::ONE).collect();
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());
            println!("normals: {:?}", normals.len());

            let triangles: Vec<UVec3> = indices
                .chunks_exact(3)
                .map(|tri| UVec3::new(tri[0], tri[1], tri[2]))
                .collect();
            result.add_section_from_buffers(&triangles, &positions, &colors, &normals, &tex_coords);

            result
        }
    }

    impl Default for Mesh {
        fn default() -> Self {
            Self::create()
        }
    }

    impl Add for Mesh {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            let mut result = Self::from_vertices(self.get_triangles(), self.get_vertices());
            result.add_section_from_vertices(rhs.get_triangles(), rhs.get_vertices());
            result
        }
    }

    impl AddAssign for Mesh {
        fn add_assign(&mut self, rhs: Self) {
            self.add_section_from_vertices(rhs.get_triangles(), rhs.get_vertices());
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
    use crate::utils::geometry::Mesh;
    use glam::{Mat4, Vec2, Vec3};
    use std::path::Path;

    pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn from_u32_u8(color: u32) -> (u8, u8, u8) {
        let r: u8 = (color >> 16) as u8;
        let g: u8 = (color >> 8) as u8;
        let b: u8 = color as u8;
        (r, g, b)
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

    pub fn barycentric_coordinates(
        pos: Vec2,
        v0: Vec2,
        v1: Vec2,
        v2: Vec2,
        area: f32,
    ) -> Option<Vec3> {
        let area_res = 1.0 / area;

        let ef0 = edge_function(pos, v1, v2) * area_res;
        let ef1 = edge_function(pos, v2, v0) * area_res;
        let ef2 = 1.0 - ef0 - ef1;

        if ef0 >= 0.0 && ef1 >= 0.0 && ef2 >= 0.0 {
            Some(glam::vec3(ef0, ef1, ef2))
        } else {
            None
        }
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

    pub fn load_gltf(path: &Path) -> Mesh {
        // handle loading textures, cameras, meshes here
        let (document, buffers, _images) = gltf::import(path).unwrap();

        for scene in document.scenes() {
            for node in scene.nodes() {
                println!(
                    "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                    node.index(),
                    node.children().count(),
                    node.camera(),
                    node.mesh().is_some(),
                    node.transform(),
                );
                println!(
                    "Node #{} has transform: trans {:?}, rot {:?}, scale {:?},",
                    node.index(),
                    node.transform().decomposed().0,
                    node.transform().decomposed().1,
                    node.transform().decomposed().2,
                );
                if let Some(mesh) = node.mesh() {
                    return Mesh::from_gltf_mesh(&mesh, &buffers);
                }
            }
        }

        Mesh::create()
    }

    pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul<f32, Output = T>
            + std::ops::Add<Output = T>
            + Copy,
    {
        start + (end - start) * alpha
    }

    //https://github.com/graphitemaster/normals_revisited
    pub fn minor(
        src: &[f32; 16],
        r0: usize,
        r1: usize,
        r2: usize,
        c0: usize,
        c1: usize,
        c2: usize,
    ) -> f32 {
        src[4 * r0 + c0]
            * (src[4 * r1 + c1] * src[4 * r2 + c2] - src[4 * r2 + c1] * src[4 * r1 + c2])
            - src[4 * r0 + c1]
                * (src[4 * r1 + c0] * src[4 * r2 + c2] - src[4 * r2 + c0] * src[4 * r1 + c2])
            + src[4 * r0 + c2]
                * (src[4 * r1 + c0] * src[4 * r2 + c1] - src[4 * r2 + c0] * src[4 * r1 + c1])
    }

    pub fn cofactor(matrix: &Mat4) -> Mat4 {
        let src: [f32; 16] = matrix.to_cols_array();
        let mut dst: [f32; 16] = [0.0; 16];
        dst[0] = minor(&src, 1, 2, 3, 1, 2, 3);
        dst[1] = -minor(&src, 1, 2, 3, 0, 2, 3);
        dst[2] = minor(&src, 1, 2, 3, 0, 1, 3);
        dst[3] = -minor(&src, 1, 2, 3, 0, 1, 2);
        dst[4] = -minor(&src, 0, 2, 3, 1, 2, 3);
        dst[5] = minor(&src, 0, 2, 3, 0, 2, 3);
        dst[6] = -minor(&src, 0, 2, 3, 0, 1, 3);
        dst[7] = minor(&src, 0, 2, 3, 0, 1, 2);
        dst[8] = minor(&src, 0, 1, 3, 1, 2, 3);
        dst[9] = -minor(&src, 0, 1, 3, 0, 2, 3);
        dst[10] = minor(&src, 0, 1, 3, 0, 1, 3);
        dst[11] = -minor(&src, 0, 1, 3, 0, 1, 2);
        dst[12] = -minor(&src, 0, 1, 2, 1, 2, 3);
        dst[13] = minor(&src, 0, 1, 2, 0, 2, 3);
        dst[14] = -minor(&src, 0, 1, 2, 0, 1, 3);
        dst[15] = minor(&src, 0, 1, 2, 0, 1, 2);
        Mat4::from_cols_array(&dst)
    }
}
