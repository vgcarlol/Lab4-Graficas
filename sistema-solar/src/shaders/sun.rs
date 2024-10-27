pub fn sun_shader(nx: f32, ny: f32, _time: f32) -> u32 {
    let r = (1.0 - (nx * nx + ny * ny).sqrt()).max(0.0); // Gradiente radial
    let g = (r * 0.8).powf(2.0);
    let b = r * 0.2;

    // Halo alrededor del sol
    let halo = ((nx * nx + ny * ny) * 2.0).min(1.0);

    let color = (
        ((255.0 * r) as u32) << 16 |
        ((150.0 * g) as u32) << 8 |
        (50.0 * b) as u32
    );

    color | ((255.0 * halo) as u32) << 24 // Añadimos el halo al alpha
}
