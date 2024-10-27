pub fn comet_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let brightness = (time * 3.0).sin().abs();
    let r = (brightness * 255.0) as u32;
    let g = ((1.0 - brightness) * 100.0) as u32;
    let b = ((nx + ny + time).sin().abs() * 255.0) as u32;

    let tail_intensity = ((ny - time).sin().abs() * 100.0) as u32;
    (r << 16) | (g << 8) | tail_intensity
}
