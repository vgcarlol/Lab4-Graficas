mod shaders;
mod utils;

use minifb::{Key, Window, WindowOptions};
use shaders::{sun_shader, rocky_planet_shader, gas_giant_shader, comet_shader, rings_shader};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

struct Planet {
    shader: fn(f32, f32, f32) -> u32,
    scale: f32,
}

impl Planet {
    fn render(&self, buffer: &mut [u32], time: f32) {
        let center_x = WIDTH as f32 / 2.0;
        let center_y = HEIGHT as f32 / 2.0;
        let scale = (WIDTH as f32 * self.scale) / 2.0;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let nx = (x as f32 - center_x) / scale;
                let ny = (y as f32 - center_y) / scale;

                if nx * nx + ny * ny <= 1.0 {
                    let color = (self.shader)(nx, ny, time);
                    buffer[y * WIDTH + x] = color;
                }
            }
        }
    }
}

fn draw_text(buffer: &mut [u32], text: &str, x: usize, y: usize, color: u32) {
    let chars = text.chars().enumerate();
    let mut pos_x = x;

    for (i, c) in chars {
        let char_code = c as usize;
        let offset = y * WIDTH + pos_x;
        if offset < buffer.len() {
            buffer[offset] = color;
        }
        pos_x += 10; // Espaciado entre caracteres
    }
}


fn char_to_pattern(c: char) -> Option<[u8; 8]> {
    match c {
        'S' => Some([0x7E, 0x81, 0x01, 0x7E, 0x80, 0x81, 0x7E, 0x00]),
        'o' => Some([0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00]),
        'l' => Some([0x7F, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00]),
        'P' => Some([0x7E, 0x81, 0x81, 0x7E, 0x80, 0x80, 0x80, 0x00]),
        'a' => Some([0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00]),
        'n' => Some([0x7E, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x00]),
        'R' => Some([0x7E, 0x81, 0x81, 0x7E, 0x90, 0x88, 0x84, 0x00]),
        _ => None, // Caracter no encontrado
    }
}

fn draw_char(buffer: &mut [u32], pattern: [u8; 8], x: usize, y: usize, color: u32) {
    for (row, line) in pattern.iter().enumerate() {
        for col in 0..8 {
            if line & (1 << (7 - col)) != 0 {
                let pixel_index = (y + row) * WIDTH + (x + col);
                if pixel_index < buffer.len() {
                    buffer[pixel_index] = color;
                }
            }
        }
    }
}



fn main() {
    let mut window = Window::new(
        "Sistema Solar Mejorado - Rust",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo abrir la ventana");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut time = 0.0;

    let sun = Planet { shader: sun_shader, scale: 1.0 };
    let rocky_planet = Planet { shader: rocky_planet_shader, scale: 0.5 };
    let gas_giant = Planet { shader: gas_giant_shader, scale: 0.7 };
    let gas_giant_with_rings = Planet { shader: rings_shader, scale: 1.0 };
    let comet = Planet { shader: comet_shader, scale: 0.3 };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.iter_mut().for_each(|pixel| *pixel = 0);
        let mut displayed_name = "";

        if window.is_key_down(Key::Key1) {
            sun.render(&mut buffer, time);
            draw_text(&mut buffer, "Sol", 10, 10, 0xFFFFFF);
        }
        if window.is_key_down(Key::Key2) {
            rocky_planet.render(&mut buffer, time);
            draw_text(&mut buffer, "Planeta Rocoso", 10, 10, 0xFFFFFF);
        }
        if window.is_key_down(Key::Key3) {
            gas_giant.render(&mut buffer, time);
            draw_text(&mut buffer, "Gigante Gaseoso", 10, 10, 0xFFFFFF);
        }
        if window.is_key_down(Key::Key4) {
            comet.render(&mut buffer, time);
            draw_text(&mut buffer, "Cometa", 10, 10, 0xFFFFFF);
        }
        if window.is_key_down(Key::Key5) {
            gas_giant_with_rings.render(&mut buffer, time);
            draw_text(&mut buffer, "Gigante con Anillos", 10, 10, 0xFFFFFF);
        }
        
        
        

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Error al actualizar la ventana");

        time += 0.02;
    }
}
