pub fn rings_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let rings = (nx * 10.0 + time).sin().abs();
    let color = if rings > 0.5 { 0xAAAAAA } else { 0x000000 };
    color
}
