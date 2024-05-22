use stb_image;
use std::path::Path;

use crate::{from_coords_index, from_u32_u8, from_u8_rgb};
use glam::Vec3;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_data = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_data {
            let data = (0..image.data.len() / 3)
                .map(|id| {
                    from_u8_rgb(
                        image.data[id * 3],
                        image.data[id * 3 + 1],
                        image.data[id * 3 + 2],
                    )
                })
                .collect();
            Self {
                width: image.width,
                height: image.height,
                data,
                depth: image.depth,
            }
        } else {
            panic!("Can't load the image");
        }
    }

    pub fn uv_to_index(&self, u: f32, v: f32) -> usize {
        let (u, v) = (u * self.width as f32, v * self.height as f32);
        let (u, v) = (
            ((u as usize) % self.width) as f32,
            ((v as usize) % self.height) as f32,
        );
        from_coords_index(glam::vec2(u, v), self.width)
    }

    pub fn sample_at_uv(&self, u: f32, v: f32) -> u32 {
        let index = self.uv_to_index(u, v);
        if index < self.data.len() {
            self.data[index]
        } else {
            from_u8_rgb(255, 0, 255)
        }
    }

    pub fn sample_at_uv_rgb(&self, u: f32, v: f32) -> Vec3 {
        let color = from_u32_u8(self.sample_at_uv(u, v));
        Vec3::new(
            (color.0 as f32) / 255.0,
            (color.1 as f32) / 255.0,
            (color.2 as f32) / 255.0,
        )
    }
}
