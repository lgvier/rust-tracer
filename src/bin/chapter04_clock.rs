use chrono::{Local, Timelike};
use std::{f64::consts::TAU, fs::create_dir_all, thread, time::Duration};

use rust_tracer::{
    canvas::Canvas,
    color::{Color, BLUE, GREEN, RED, WHITE},
    point,
    tuple::Tuple,
};

fn main() -> std::io::Result<()> {
    let size = 500;
    let center = 250f64;
    let radius = 220f64;
    let twelve = point!(0., 0., 1.);

    let folder = "/tmp/04_clock";
    create_dir_all(folder)?;

    for _ in 0..10 {
        let mut c = Canvas::new(size, size);

        let mut plot_line =
            |i: f64, count: usize, line_radius: f64, line_width: usize, color: Color| {
                let p = twelve.rotated_y(i * (TAU / count as f64));
                for li in 0..line_width {
                    let x = (p.x * (line_radius + li as f64)) + center;
                    // not sure why I had to flip y...
                    let y = -(p.z * (line_radius + li as f64)) + center;
                    //println!("{} / {}: {:?} -> {}, {}", i, count, p, x, y);
                    c.write_pixel(x as usize, y as usize, color);
                }
            };

        for m in 0..60 {
            plot_line(m as f64, 60, radius, 10, BLUE);
        }

        for h in 0..12 {
            plot_line(h as f64, 12, radius, 20, GREEN);
        }

        let now = Local::now();
        let (_, hour) = now.hour12();
        let minute = now.minute();
        let second = now.second();
        println!("It's {}:{}:{}", hour, minute, second);
        plot_line(hour as f64 + (minute as f64 / 60.), 12, 0., 100, WHITE);
        plot_line(minute as f64 + (second as f64 / 60.), 60, 0., 150, WHITE);
        plot_line(second as f64, 60, -20., 190, RED);

        let path = format!("{}/{}_{}_{}.png", folder, hour, minute, second);
        c.save(path.as_ref())?;

        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
