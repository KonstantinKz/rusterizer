pub mod tests {
    use crate::texture::*;
    use crate::utils::geometry::*;
    use crate::Screen;
    use glam::Vec3Swizzles;
    use std::path::Path;

    pub fn _test_edge_function(screen: &mut Screen) {
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(250.0, 400.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        screen.output_edge_function(v0.position.xy(), v1.position.xy());
    }

    pub fn _test_triangle1(screen: &mut Screen) {
        // Nice triangle
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(250.0, 400.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v2 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 0.0, 1.0),
            uv: glam::vec2(0.0, 0.0),
        };

        screen.output_triangle1(v0.position.xy(), v1.position.xy(), v2.position.xy());
    }

    pub fn _test_triangle2(screen: &mut Screen) {
        // Nice triangle
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(250.0, 400.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v2 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 0.0, 1.0),
            uv: glam::vec2(0.0, 0.0),
        };

        screen.output_triangle2(v0.position.xy(), v1.position.xy(), v2.position.xy());
    }

    pub fn _test_barycentric(screen: &mut Screen) {
        // Nice triangle
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(250.0, 400.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v2 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 0.0, 1.0),
            uv: glam::vec2(0.0, 0.0),
        };

        screen.output_barycentric(&v0, &v1, &v2);
    }

    pub fn _test_textured_triangle(screen: &mut Screen) {
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(250.0, 400.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(1.0, 0.0),
        };

        let v2 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 0.0, 1.0),
            uv: glam::vec2(0.5, 1.0),
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        screen.raster_triangle(&v0, &v1, &v2, &texture);
    }

    pub fn _test_textured_quad(screen: &mut Screen) {
        // Quad
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 0.0),
        };
        let v1 = Vertex {
            position: glam::vec3(100.0, 400.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 1.0),
        };
        let v2 = Vertex {
            position: glam::vec3(400.0, 400.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(1.0, 1.0),
        };
        let v3 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 0.0),
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        screen.raster_triangle(&v0, &v2, &v1, &texture);
        screen.raster_triangle(&v0, &v3, &v2, &texture);
    }
}
