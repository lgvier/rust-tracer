#[macro_use]
extern crate derive_builder;

pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod patterns;
pub mod ray;
pub mod shapes;
pub mod transformations;
pub mod tuple;
pub mod world;

pub const EPSILON: f64 = 0.00001;

pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
