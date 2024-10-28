use noise::{NoiseFn, Perlin};

pub fn earth_shader(nx: f32, ny: f32, time: f32) -> u32 {
    let perlin = Perlin::new(42);

    // Varias capas de ruido Perlin para mayor detalle
    let noise_value = (
        0.5 * perlin.get([nx as f64, ny as f64, time as f64])
        + 0.25 * perlin.get([nx as f64 * 2.0, ny as f64 * 2.0, time as f64])
        + 0.125 * perlin.get([nx as f64 * 4.0, ny as f64 * 4.0, time as f64])
    ) as f32;

    let threshold = 0.2; // Umbral ajustado para patrones coherentes

    // Transición suave entre océanos y continentes
    let smooth_value = smooth_step(threshold - 0.05, threshold + 0.05, noise_value);

    if smooth_value > 0.5 {
        0x32CD32 // Verde para continentes
    } else {
        0x1E90FF // Azul para océanos
    }
}

// Función de suavizado
fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn render_earth_with_rotation_and_translation(
    buffer: &mut [u32],
    time: f32,
    width: usize,
    height: usize,
) {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    // Radio de la órbita (traslación)
    let orbit_radius = width as f32 * 0.3;

    // Hacemos la traslación más rápida multiplicando el tiempo por 0.5
    let earth_x = center_x + orbit_radius * (time * 2.5).cos();
    let earth_y = center_y + orbit_radius * (time * 2.5).sin();

    let scale = (width as f32 * 0.15) / 2.0;

    // Dibujamos el planeta con rotación más rápida
    for y in 0..height {
        for x in 0..width {
            // Aumentamos la velocidad de rotación multiplicando el tiempo por 1.0
            let angle = time * 3.0;
            let nx = ((x as f32 - earth_x) * angle.cos()
                - (y as f32 - earth_y) * angle.sin())
                / scale;
            let ny = ((x as f32 - earth_x) * angle.sin()
                + (y as f32 - earth_y) * angle.cos())
                / scale;

            if nx * nx + ny * ny <= 1.0 {
                let color = earth_shader(nx, ny, time);
                buffer[y * width + x] = color;
            }
        }
    }
}
