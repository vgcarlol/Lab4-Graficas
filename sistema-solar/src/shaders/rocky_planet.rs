use crate::utils::lambert_shading;
use nalgebra::Vector3;

pub fn rocky_planet_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let light_dir = Vector3::new(0.0, 1.0, 1.0).normalize();
    let intensity = lambert_shading(nx, ny, light_dir);

    let r = (intensity * 200.0) as u32;
    let g = (intensity * 100.0) as u32;
    let b = (intensity * 50.0) as u32;

    (r << 16) | (g << 8) | b
}
