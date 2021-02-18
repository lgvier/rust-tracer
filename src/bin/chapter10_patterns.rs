use rust_tracer::{
    camera::Camera,
    checkers_pattern,
    color::{Color, BLUE, RED, WHITE},
    gradient_pattern,
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    patterns::{CheckersPattern, GradientPattern, Pattern, RingPattern, StripePattern},
    plane, point, ring_pattern,
    shapes::{Plane, Shape, Sphere},
    sphere, stripe_pattern,
    tuple::Tuple,
    vector,
    world::World,
};
use std::f64::consts::PI;

fn main() -> std::io::Result<()> {
    let floor_pattern = checkers_pattern!(RED, BLUE);
    let floor_material = MaterialBuilder::default()
        .pattern(floor_pattern)
        .specular(0.)
        .build()
        .unwrap();

    let mut floor = plane!();
    floor.set_transform(Matrix::scaling(10., 0.01, 10.));
    floor.set_material(floor_material);

    let mut middle = sphere!();
    middle.set_transform(Matrix::translation(-0.5, 1., 0.5));
    let mut middle_pattern = stripe_pattern!(0.5, 1., 0.1; 1., 0.8, 0.1);
    middle_pattern.set_transform(Matrix::scaling(0.5, 0.5, 0.5));
    middle.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(middle_pattern)
            .build()
            .unwrap(),
    );

    let mut left = sphere!();
    left.set_transform(Matrix::translation(-1.5, 0.33, -0.55) * Matrix::scaling(0.33, 0.33, 0.33));
    let mut left_pattern = gradient_pattern!(0.5, 1., 0.1; 1., 0.8, 0.1);
    left_pattern.set_transform(Matrix::rotation_z(PI / 2.));
    left.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(left_pattern)
            .build()
            .unwrap(),
    );

    let mut right = sphere!();
    right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    let mut right_pattern = ring_pattern!(RED, BLUE);
    right_pattern.set_transform(Matrix::scaling(0.1, 0.1, 0.1) * Matrix::rotation_z(PI / 3.));
    right.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(right_pattern)
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
