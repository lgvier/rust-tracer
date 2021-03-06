use std::f64::consts::PI;

use rust_tracer::{
    arena::Arena, canvas::Canvas, color::WHITE, intersection::Intersection, light::PointLight,
    material::MaterialBuilder, matrix::Matrix, point, ray, solid, sphere,
};

fn main() -> std::io::Result<()> {
    let ray_origin = point!(0, 0, -5);
    let wall_z = 10;
    let wall_size = 7.;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);

    let arena = Arena::new();
    let mut shape = sphere!();
    //sphere.set_transform(Matrix::scaling(1, 0.5, 1));
    //sphere.set_transform(Matrix::scaling(0.5, 1, 1));
    shape.set_transform(Matrix::rotation_z(PI / 4.) * Matrix::scaling(0.5, 1, 1));
    //sphere.set_transform(Matrix::shearing(1, 0, 0, 0, 0, 0) * Matrix::scaling(0.5, 1, 1));

    let material = MaterialBuilder::default()
        .pattern(solid!(1, 0.2, 1))
        .ambient(0.2)
        .build()
        .unwrap();
    shape.set_material(material);

    let light_position = point!(-10, 10, -10);
    let light_color = WHITE;
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = point!(world_x, world_y, wall_z);

            let r = ray!(ray_origin, (position - ray_origin).normalize());

            let xs = shape.intersect(&arena, &r);
            if let Some(hit) = Intersection::hit(xs) {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&arena, point);
                let eye = -r.direction;

                let color = hit
                    .object
                    .material()
                    .lightning(&shape, &light, point, eye, normal, false);
                c.write_pixel(x, y, color);
            }
        }
    }

    c.save("/tmp/06_sphere.png")?;
    Ok(())
}
