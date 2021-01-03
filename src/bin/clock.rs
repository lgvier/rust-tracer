use std::f64::consts::PI;

use rust_tracer::canvas::Canvas;
use rust_tracer::color::{Color, BLUE, GREEN};
use rust_tracer::matrix::Matrix;
use rust_tracer::point;
use rust_tracer::tuple::Tuple;

fn main() -> std::io::Result<()> {
    let size = 100;
    let center = 50f64;
    let radius = 40f64;
    let twelve = point!(0., 0., 1.);

    let mut c = Canvas::new(size, size);

    let mut plot = |p: Tuple, p_radius: f64, color: Color| {
        let x = p.x * p_radius + center;
        let y = p.z * p_radius + center;
        println!("{}, {}", x, y);
        c.write_pixel(x as usize, y as usize, color);
    };

    let mut plot_line = |i: usize, count: usize, line_width: usize, color: Color| {
        let r = Matrix::rotation_y(i as f64 * 2. * PI / count as f64);
        let hp = r * twelve;
        for i in 0..line_width {
            plot(hp, radius - i as f64, color);
        }
    };

    for m in 0..60 {
        plot_line(m, 60, 3, BLUE);
    }

    for h in 0..12 {
        plot_line(h, 12, 7, GREEN);
    }

    c.save("/tmp/clock.png")?;
    Ok(())
}
