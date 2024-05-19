use glam::Vec2;

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

// conversion from indives to coordinate system
pub fn from_index_coords(index: usize, width: usize) -> Vec2 {
    Vec2::new((index % width) as f32, (index / width) as f32)
}

// conversion from a coordinate system to indices
pub fn _from_coords_index(pos: Vec2, width: usize) -> usize {
    (pos.x % width as f32 + pos.y * width as f32) as usize
}

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

pub fn edge_function(pos: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let seg_a = v1 - v0;
    let seg_b = pos - v0;

    seg_a.x * seg_b.y - seg_a.y * seg_b.x
}

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

pub fn _test_triangle1(pos: Vec2, v0: Vec2, v1: Vec2, v2: Vec2) -> u32 {
    let mut color: u32 = from_u8_rgb(255, 0, 0);
    let ef0 = edge_function(pos, v0, v1);
    let ef1 = edge_function(pos, v1, v2);
    let ef2 = edge_function(pos, v2, v0);

    if ef0 > 0.0 && ef1 > 0.0 && ef2 > 0.0 || ef0 < 0.0 && ef1 < 0.0 && ef2 < 0.0 {
        color = from_u8_rgb(0, 255, 0);
    }

    color
}

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
