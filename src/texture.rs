use stb_image;
use std::path::Path;

use crate::{from_coords_index, from_u8_rgb};

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

    pub fn sample_at_uv(&self, u: f32, v: f32) -> u32 {
        let mapped_u = u * self.width as f32;
        let mapped_v = v * self.height as f32;

        let index = from_coords_index(glam::vec2(mapped_u, mapped_v), self.width);
        if index < self.data.len() {
            self.data[index]
        } else {
            0
        }
    }
}
