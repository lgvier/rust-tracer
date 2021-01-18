use rust_tracer::canvas::Canvas;
use rust_tracer::color::RED;
use rust_tracer::tuple::Tuple;
use rust_tracer::{point, vector};

#[derive(Debug)]
struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}
struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

fn tick(e: &Environment, p: &Projectile) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + e.gravity + e.wind;
    Projectile { position, velocity }
}
fn main() -> std::io::Result<()> {
    let mut p = Projectile {
        position: point!(0., 1., 0.),
        velocity: vector!(1., 1.8, 0.).normalize() * 9.4,
    };
    let e = Environment {
        gravity: vector!(0., -0.1, 0.),
        wind: vector!(0.01, 0., 0.),
    };
    let mut c = Canvas::new(900, 550);

    let mut write_pixel = |pos: &Tuple| {
        let x = (pos.x as usize).min(c.width - 1);
        let y = ((c.height as f64 - 1. - pos.y) as usize).min(c.height - 1);
        c.write_pixel(x, y, RED);
    };
    write_pixel(&p.position);
    let mut count = 0;
    while p.position.y > 0. {
        // println!("p: {:#?}", p);
        p = tick(&e, &p);
        write_pixel(&p.position);
        count = count + 1;
    }
    println!("end p: {:#?}, count: {}", p, count);

    c.save("/tmp/02_projectile.png")?;
    Ok(())
}
