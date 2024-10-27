pub fn comet_shader(_nx: f32, _ny: f32, time: f32) -> u32 {
    let brightness = (time * 5.0).sin().abs();
    let r = (brightness * 255.0) as u32;
    let g = ((1.0 - brightness) * 50.0) as u32;
    let b = ((1.0 - brightness) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}
