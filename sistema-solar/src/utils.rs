use nalgebra::Vector3;
use obj::Obj;
use std::path::Path;

pub fn lambert_shading(nx: f32, ny: f32, light_dir: nalgebra::Vector3<f32>) -> f32 {
    let normal = nalgebra::Vector3::new(nx, ny, (1.0 - nx * nx - ny * ny).sqrt());
    let intensity = normal.dot(&light_dir).max(0.0);
    intensity
}

pub fn load_sphere(path: &str) -> Vec<Vector3<f32>> {
    let sphere = Obj::load(Path::new(path)).expect("Error al cargar el modelo");
    sphere
        .data
        .position
        .into_iter()
        .map(|v| Vector3::new(v[0], v[1], v[2]))
        .collect()
}
