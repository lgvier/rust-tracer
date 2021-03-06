use std::f64::consts::PI;

use rust_tracer::{
    camera::Camera,
    checkers_pattern,
    color::{BLACK, BLUE, RED, WHITE, YELLOW},
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    plane, point, ring_pattern, solid, sphere, stripe_pattern, vector,
    world::World,
};

fn main() -> std::io::Result<()> {
    let floor_pattern = checkers_pattern!(BLACK, WHITE);
    let floor_material = MaterialBuilder::default()
        .pattern(floor_pattern)
        .specular(0)
        .reflective(0.5)
        .build()
        .unwrap();

    let mut floor = plane!();
    floor.set_transform(Matrix::scaling(10, 0.01, 10));
    floor.set_material(floor_material);

    let mut middle = sphere!();
    middle.set_transform(Matrix::translation(-0.5, 1, 0.5));
    let mut middle_pattern = stripe_pattern!(0.5, 1, 0.1; 1, 0.8, 0.1);
    middle_pattern.set_transform(Matrix::scaling(0.5, 0.5, 0.5));
    middle.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .reflective(0.1)
            .pattern(middle_pattern)
            .build()
            .unwrap(),
    );

    let mut left = sphere!();
    left.set_transform(Matrix::translation(-1.5, 0.33, -0.55) * Matrix::scaling(0.33, 0.33, 0.33));
    left.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .reflective(0.1)
            .pattern(solid!(YELLOW))
            .build()
            .unwrap(),
    );

    let mut right = sphere!();
    right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    let mut right_pattern = ring_pattern!(RED, BLUE);
    right_pattern.set_transform(Matrix::scaling(0.1, 0.1, 0.1) * Matrix::rotation_x(PI / 1.5));
    right.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .reflective(0.1)
            .pattern(right_pattern)
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10, 10, -10), WHITE);

    let world = World::new(light_source, vec![floor, middle, left, right]);

    let hsize = 800 * 2;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(0, 1.5, -5),
        point!(0, 1, 0),
        vector!(0, 1, 0),
    ));

    let canvas = camera.render(&world, true);
    canvas.save("/tmp/11a_reflection.png")?;
    Ok(())
}
