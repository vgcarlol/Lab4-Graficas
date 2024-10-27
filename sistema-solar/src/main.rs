use minifb::{Key, Window, WindowOptions};
use nalgebra::{Vector3, Matrix4};
use obj::Obj;
use std::path::Path;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

struct Planet {
    vertices: Vec<Vector3<f32>>,
    shader: fn(f32, f32) -> u32,
    position: Vector3<f32>,
    scale: f32,
}

impl Planet {
    fn render(&self, buffer: &mut [u32], time: f32, transform: Matrix4<f32>) {
        for vertex in &self.vertices {
            // Convertimos los vértices a puntos 4D para aplicar la transformación.
            let vertex_4d = vertex.insert_row(3, 1.0);
            let transformed = transform * vertex_4d;

            let (x, y) = (
                ((transformed.x / transformed.w + 1.0) * 0.5 * WIDTH as f32) as usize,
                ((1.0 - transformed.y / transformed.w) * 0.5 * HEIGHT as f32) as usize,
            );

            if x < WIDTH && y < HEIGHT {
                let color = (self.shader)(vertex.x, vertex.y + time);
                buffer[y * WIDTH + x] = color;
            }
        }
    }
}

fn load_sphere(path: &str) -> Vec<Vector3<f32>> {
    let sphere = Obj::load(Path::new(path)).expect("Error al cargar el modelo");
    sphere
        .data
        .position
        .into_iter()
        .map(|v| Vector3::new(v[0], v[1], v[2]))
        .collect()
}

fn sun_shader(x: f32, y: f32) -> u32 {
    let intensity = ((x * x + y * y).sqrt()).cos();
    (255.0 * intensity) as u32 | ((255.0 * intensity) as u32) << 8
}

fn rocky_planet_shader(x: f32, y: f32) -> u32 {
    let noise = (x * 10.0).sin() * (y * 10.0).cos();
    0x0000FF | ((noise * 255.0) as u32) << 16
}

fn gas_giant_shader(x: f32, y: f32) -> u32 {
    let stripes = ((x * 20.0).sin() + (y * 20.0).cos()) * 0.5 + 0.5;
    let r = (stripes * 255.0) as u32;
    r | (r << 8) | (r << 16)
}

fn main() {
    let mut window = Window::new(
        "Sistema Solar - Rust",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("No se pudo abrir la ventana");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut time = 0.0;

    let sphere_vertices = load_sphere("assets/sphere.obj");

    let sun = Planet {
        vertices: sphere_vertices.clone(),
        shader: sun_shader,
        position: Vector3::new(0.0, 0.0, 0.0),
        scale: 1.0,
    };

    let rocky_planet = Planet {
        vertices: sphere_vertices.clone(),
        shader: rocky_planet_shader,
        position: Vector3::new(-0.6, 0.0, 0.0),
        scale: 0.5,
    };

    let gas_giant = Planet {
        vertices: sphere_vertices,
        shader: gas_giant_shader,
        position: Vector3::new(0.6, 0.0, 0.0),
        scale: 0.7,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.iter_mut().for_each(|pixel| *pixel = 0);

        let transform = Matrix4::new_scaling(0.5);

        sun.render(&mut buffer, time, transform);
        rocky_planet.render(&mut buffer, time * 0.2, transform);
        gas_giant.render(&mut buffer, time * 0.1, transform);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Error al actualizar la ventana");

        time += 0.01;
    }
}
