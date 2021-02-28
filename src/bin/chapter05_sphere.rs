use std::f64::consts::PI;

use rust_tracer::{
    canvas::Canvas,
    color::RED,
    intersection::Intersection,
    matrix::Matrix,
    point, ray,
    ray::Ray,
    shapes::{sphere::Sphere, Shape},
    sphere,
    tuple::Tuple,
};

fn main() -> std::io::Result<()> {
    let ray_origin = point!(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);

    let mut shape = sphere!();
    // sphere.set_transform(Matrix::scaling(1., 0.5, 1.));
    //sphere.set_transform(Matrix::scaling(0.5, 1., 1.));
    shape.set_transform(Matrix::rotation_z(PI / 4.) * Matrix::scaling(0.5, 1., 1.));
    //sphere.set_transform(Matrix::shearing(1., 0., 0., 0., 0., 0.) * Matrix::scaling(0.5, 1., 1.));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = point!(world_x, world_y, wall_z);

            let r = ray!(ray_origin, (position - ray_origin).normalize());

            let xs = shape
                .intersect(&r)
                .iter()
                .map(|t| Intersection::new(*t, &shape))
                .collect::<Vec<Intersection>>();
            let xs_refs = xs.iter().collect::<Vec<&Intersection>>();

            if Intersection::hit(&xs_refs[..]).is_some() {
                c.write_pixel(x, y, RED);
            }
        }
    }

    c.save("/tmp/05_sphere.png")?;
    Ok(())
}
