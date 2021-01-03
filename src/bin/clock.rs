use std::f64::consts::TAU;

use rust_tracer::{
    canvas::Canvas,
    color::{Color, BLUE, GREEN, WHITE},
    point,
    tuple::Tuple,
};

fn main() -> std::io::Result<()> {
    let size = 500;
    let center = 250f64;
    let radius = 220f64;
    let twelve = point!(0., 0., 1.);

    let mut c = Canvas::new(size, size);

    let mut plot_line =
        |i: usize, count: usize, line_radius: f64, line_width: usize, color: Color| {
            let p = twelve.rotate_y(i as f64 * (TAU / count as f64));
            for li in 0..line_width {
                let x = (p.x * (line_radius + li as f64)) + center;
                let y = -(p.z * (line_radius + li as f64)) + center;
                //println!("{} / {}: {:?} -> {}, {}", i, count, p, x, y);
                c.write_pixel(x as usize, y as usize, color);
            }
        };

    for m in 0..60 {
        plot_line(m, 60, radius, 10, BLUE);
    }

    for h in 0..12 {
        plot_line(h, 12, radius, 20, GREEN);
    }

    plot_line(2, 12, 0., 100, WHITE);
    plot_line(30, 60, 0., 170, WHITE);

    c.save("/tmp/clock.png")?;
    Ok(())
}
