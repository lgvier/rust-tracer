use rust_tracer::{
    camera::Camera,
    color::{Color, BLUE, RED, WHITE},
    gradient,
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    patterns::{GradientPattern, Pattern, StripePattern},
    plane, point,
    shapes::{Plane, Shape, Sphere},
    solid, sphere, stripes,
    tuple::Tuple,
    vector,
    world::World,
};
use std::f64::consts::PI;

fn main() -> std::io::Result<()> {
    let floor_material = MaterialBuilder::default()
        .pattern(solid!(1., 0.9, 0.9))
        .specular(0.)
        .build()
        .unwrap();

    let mut floor = plane!();
    floor.set_transform(Matrix::scaling(10., 0.01, 10.));
    floor.set_material(floor_material);

    let mut middle = sphere!();
    middle.set_transform(Matrix::translation(-0.5, 1., 0.5));
    middle.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(stripes!(0.5, 1., 0.1; 1., 0.8, 0.1))
            .build()
            .unwrap(),
    );

    let mut left = sphere!();
    left.set_transform(Matrix::translation(-1.5, 0.33, -0.55) * Matrix::scaling(0.33, 0.33, 0.33));
    left.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(gradient!(0.5, 1., 0.1; 1., 0.8, 0.1))
            .build()
            .unwrap(),
    );

    let mut right = sphere!();
    right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    right.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(gradient!(RED, BLUE))
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10., 10., -10.), WHITE);

    let world = World::new(light_source, vec![floor, middle, left, right]);
    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(0., 1.5, -5.),
        point!(0., 1., 0.),
        vector!(0., 1., 0.),
    ));

    let canvas = camera.render(&world);
    canvas.save("/tmp/10_patterns.png")?;
    Ok(())
}
