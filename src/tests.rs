pub mod tests {
    use crate::camera::Camera;
    use crate::texture::*;
    use crate::transform::Transform;
    use crate::utils::geometry::*;
    use crate::Screen;
    use glam::Vec3Swizzles;
    use std::path::Path;

    pub fn _test_indices(screen: &mut Screen) {
        screen._output_index();
    }

    pub fn _test_coords(screen: &mut Screen) {
        screen._output_coords();
    }

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

        screen._output_edge_function(v0.position.xy(), v1.position.xy());
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

        screen._output_triangle1(v0.position.xy(), v1.position.xy(), v2.position.xy());
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

        screen._output_triangle2(v0.position.xy(), v1.position.xy(), v2.position.xy());
    }

    pub fn _test_barycentric(screen: &mut Screen) {
        // Nice triangle
        let v0 = Vertex {
            position: glam::vec3(100.0, 100.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v1 = Vertex {
            position: glam::vec3(250.0, 400.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };

        let v2 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 0.0, 1.0),
            uv: glam::vec2(0.0, 0.0),
        };

        screen._output_barycentric(&v0, &v1, &v2);
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
            uv: glam::vec2(0.5, 1.0),
        };

        let v2 = Vertex {
            position: glam::vec3(400.0, 100.0, 1.0),
            color: glam::vec3(0.0, 0.0, 1.0),
            uv: glam::vec2(1.0, 0.0),
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        screen.raster_triangle(&v0, &v1, &v2, Some(&texture), None, None);
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

        screen.raster_triangle(&v0, &v2, &v1, Some(&texture), None, None);
        screen.raster_triangle(&v0, &v3, &v2, Some(&texture), None, None);
    }

    pub fn _test_camera(screen: &mut Screen, rot: &mut f32) {
        // Quad
        let v0 = Vertex {
            position: glam::vec3(-2.0, -2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        };
        let v1 = Vertex {
            position: glam::vec3(-2.0, 2.0, 0.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };
        let v2 = Vertex {
            position: glam::vec3(2.0, 2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(1.0, 0.0),
        };
        let v3 = Vertex {
            position: glam::vec3(2.0, -2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 1.0),
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        let aspect_ratio: f32 = 1.0;

        let camera = Camera {
            far_plane: 100.0,
            aspect_ratio,
            transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
            ..Default::default()
        };

        *rot += 0.05;

        let transform =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, *rot, 0.0, 0.0));

        screen.raster_triangle(
            &v0,
            &v2,
            &v1,
            Some(&texture),
            Some(&camera),
            Some(&transform.get_local()),
        );
        screen.raster_triangle(
            &v0,
            &v3,
            &v2,
            Some(&texture),
            Some(&camera),
            Some(&transform.get_local()),
        );
    }

    pub fn _test_raster_mesh(screen: &mut Screen) {
        // Quad
        let v0 = Vertex {
            position: glam::vec3(-2.0, -2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        };
        let v1 = Vertex {
            position: glam::vec3(-2.0, 2.0, 0.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };
        let v2 = Vertex {
            position: glam::vec3(2.0, 2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(1.0, 0.0),
        };
        let v3 = Vertex {
            position: glam::vec3(2.0, -2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 1.0),
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        // Camera
        let aspect_ratio: f32 = 1.0;
        let camera = Camera {
            far_plane: 100.0,
            aspect_ratio,
            transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
            ..Default::default()
        };

        let transform =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0));

        // Mesh
        let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)];
        let vertices = vec![v0, v1, v2, v3];
        let mesh = Mesh::from_vertices(&triangles, &vertices);

        screen.raster_mesh(
            &mesh,
            Some(&transform.get_local()),
            Some(&texture),
            Some(&camera),
        );
    }

    pub fn _test_textured_cube(screen: &mut Screen, rot: &mut f32) {
        // Quad
        let v0 = Vertex {
            position: glam::vec3(-1.0, -1.0, 1.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        };
        let v1 = Vertex {
            position: glam::vec3(-1.0, 1.0, 1.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };
        let v2 = Vertex {
            position: glam::vec3(1.0, 1.0, 1.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(1.0, 0.0),
        };
        let v3 = Vertex {
            position: glam::vec3(1.0, -1.0, 1.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 1.0),
        };

        // Camera
        let aspect_ratio: f32 = 1.0;
        let camera = Camera {
            far_plane: 100.0,
            aspect_ratio,
            transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
            ..Default::default()
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        // Mesh
        let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)];
        let vertices = vec![v0, v1, v2, v3];
        let mesh = Mesh::from_vertices(&triangles, &vertices);

        // Transforms
        let transform0 = Transform::IDENTITY;

        // Rotate
        *rot += 0.05;

        //-z
        let transform1 = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            -std::f32::consts::PI,
            0.0,
            0.0,
        ));
        //+y
        let transform2 = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        ));
        //-y
        let transform3 = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        ));
        //+x
        let transform4 = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            -std::f32::consts::FRAC_PI_2,
            0.0,
        ));
        //-x
        let transform5 = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            std::f32::consts::FRAC_PI_2,
            0.0,
        ));

        let parent_local =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, *rot, *rot, 0.0))
                .get_local();

        screen.raster_mesh(
            &mesh,
            Some(&(parent_local * transform0.get_local())),
            Some(&texture),
            Some(&camera),
        );
        screen.raster_mesh(
            &mesh,
            Some(&(parent_local * transform1.get_local())),
            Some(&texture),
            Some(&camera),
        );
        screen.raster_mesh(
            &mesh,
            Some(&(parent_local * transform2.get_local())),
            Some(&texture),
            Some(&camera),
        );
        screen.raster_mesh(
            &mesh,
            Some(&(parent_local * transform3.get_local())),
            Some(&texture),
            Some(&camera),
        );

        screen.raster_mesh(
            &mesh,
            Some(&(parent_local * transform4.get_local())),
            Some(&texture),
            Some(&camera),
        );
        screen.raster_mesh(
            &mesh,
            Some(&(parent_local * transform5.get_local())),
            Some(&texture),
            Some(&camera),
        );
    }

    pub fn _test_camera_inputs(screen: &mut Screen, rot: &mut f32, camera: &Camera) {
        // Quad
        let v0 = Vertex {
            position: glam::vec3(-2.0, -2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        };
        let v1 = Vertex {
            position: glam::vec3(-2.0, 2.0, 0.0),
            color: glam::vec3(1.0, 0.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        };
        let v2 = Vertex {
            position: glam::vec3(2.0, 2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 0.0),
            uv: glam::vec2(1.0, 0.0),
        };
        let v3 = Vertex {
            position: glam::vec3(2.0, -2.0, 0.0),
            color: glam::vec3(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 1.0),
        };

        // Texture
        let texture = Texture::load(Path::new("assets/bojan.jpg"));

        *rot += 0.05;

        let transform =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, *rot, 0.0, 0.0));

        // Mesh
        let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)];
        let vertices = vec![v0, v1, v2, v3];
        let mesh = Mesh::from_vertices(&triangles, &vertices);

        screen.raster_mesh(
            &mesh,
            Some(&transform.get_local()),
            Some(&texture),
            Some(&camera),
        );
    }
}
