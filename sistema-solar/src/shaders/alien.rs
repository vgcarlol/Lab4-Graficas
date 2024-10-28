pub fn alien_planet_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let r = ((nx * 2.0 + time).sin().abs() * 255.0) as u32;
    let g = ((ny * 2.0 + time).cos().abs() * 255.0) as u32;
    let b = ((nx + ny + time).sin().abs() * 255.0) as u32;

    (r << 16) | (g << 8) | b // Colores dinámicos para un planeta alienígena
}
