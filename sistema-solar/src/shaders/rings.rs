pub fn rings_shader(nx: f32, ny: f32, time: f32) -> u32 {
    // Aplicamos un ángulo de rotación basado en el tiempo
    let angle = time; // Usamos el tiempo como ángulo de rotación
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    // Rotación de las coordenadas usando una matriz de rotación
    let rotated_x = nx * cos_angle - ny * sin_angle;
    let rotated_y = nx * sin_angle + ny * cos_angle;

    // Calculamos la distancia al centro del planeta
    let distance = (rotated_x * rotated_x + rotated_y * rotated_y).sqrt();

    // Definimos los colores: marrón para el planeta y blanco para los anillos
    let brown = 0x8B4513; // Café para el centro del planeta
    let white = 0xFFFFFF; // Blanco para los anillos

    // Lógica de renderizado para los anillos y el planeta
    if distance < 0.4 {
        brown // Centro del planeta
    } else if distance > 0.5 && distance < 0.6 {
        // Aquí hacemos que los anillos varíen en un patrón dinámico
        let dynamic_ring = ((rotated_x * 10.0).sin().abs() * 255.0) as u32;
        (dynamic_ring << 16) | (dynamic_ring << 8) | dynamic_ring
    } else {
        0x000000 // Fondo negro
    }
}
