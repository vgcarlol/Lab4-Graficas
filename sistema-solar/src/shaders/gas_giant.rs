use nalgebra::Vector3;

pub fn gas_giant_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let stripe = (ny * 5.0 + time).sin();
    let r = ((0.5 + 0.5 * stripe) * 255.0) as u32;
    let g = ((0.3 + 0.5 * stripe) * 170.0) as u32;
    let b = ((0.6 + 0.4 * stripe) * 204.0) as u32;
    ((r << 16) | (g << 8) | b)
}
