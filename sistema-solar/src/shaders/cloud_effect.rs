pub fn cloud_effect(x: f32, y: f32, time: f32) -> u32 {
    let cloud_movement = (x * 10.0 + time).sin() * (y * 10.0 + time).cos();
    let base_color = 0x87CEEB; // Azul cielo
    let alpha = ((cloud_movement + 1.0) * 0.5 * 255.0) as u32;
    (alpha << 24) | base_color
}
