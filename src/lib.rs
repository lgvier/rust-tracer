pub mod canvas;
pub mod color;
pub mod matrix;
pub mod tuple;

pub const EPSILON: f64 = 0.00001;

pub fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
