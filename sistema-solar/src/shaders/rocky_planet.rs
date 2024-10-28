// rocky_planet.rs

use crate::utils::lambert_shading;
use nalgebra::Vector3;

pub fn rocky_planet_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let angle = time * 0.5;
    let rotated_nx = nx * angle.cos() - ny * angle.sin();
    let rotated_ny = nx * angle.sin() + ny * angle.cos();

    let light_dir = Vector3::new(0.0, 1.0, 1.0).normalize();
    let intensity = lambert_shading(rotated_nx, rotated_ny, light_dir);

    let r = (intensity * 200.0) as u32;
    let g = (intensity * 100.0) as u32;
    let b = (intensity * 50.0) as u32;

    (r << 16) | (g << 8) | b
}

// Exponemos la función para su uso en otros módulos.
pub fn render_rocky_planet_with_moon(
    buffer: &mut [u32],
    time: f32,
    width: usize,
    height: usize,
) {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let scale = (width as f32 * 0.25) / 2.0;

    let moon_orbit_radius = scale * 1.5;
    let moon_x = center_x + moon_orbit_radius * (time * 0.5).cos();
    let moon_y = center_y + moon_orbit_radius * (time * 0.5).sin();
    let moon_scale = scale * 0.2;

    for y in 0..height {
        for x in 0..width {
            let moon_nx = (x as f32 - moon_x) / moon_scale;
            let moon_ny = (y as f32 - moon_y) / moon_scale;

            if moon_nx * moon_nx + moon_ny * moon_ny <= 1.0 {
                buffer[y * width + x] = moon_shader(moon_nx, moon_ny);
            }

            let nx = (x as f32 - center_x) / scale;
            let ny = (y as f32 - center_y) / scale;

            if nx * nx + ny * ny <= 1.0 {
                buffer[y * width + x] = rocky_planet_shader(nx, ny, time);
            }
        }
    }
}

fn moon_shader(nx: f32, ny: f32) -> u32 {
    let intensity = ((nx + ny).sin().abs() * 255.0) as u32;
    (intensity << 8) | (intensity << 16) | intensity
}
