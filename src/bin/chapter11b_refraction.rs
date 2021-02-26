use rust_tracer::{
    camera::Camera,
    checkers_pattern,
    color::{BLACK, RED, WHITE},
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    patterns::{CheckersPattern, Pattern},
    plane, point,
    shapes::{Plane, Shape, Sphere},
    solid, sphere,
    tuple::Tuple,
    vector,
    world::World,
};
use std::f64::consts::PI;

fn main() -> std::io::Result<()> {
    let mut floor_and_walls_pattern = checkers_pattern!(RED, WHITE);
    floor_and_walls_pattern.set_transform(Matrix::scaling(0.15, 0.15, 0.15));
    let floor_and_walls_material = MaterialBuilder::default()
        .pattern(floor_and_walls_pattern)
        .specular(0.)
        .build()
        .unwrap();

    let mut floor = plane!();
    floor.set_transform(Matrix::scaling(10., 0.01, 10.));
    floor.set_material(floor_and_walls_material);

    let mut left_wall = plane!();
    left_wall.set_transform(
        Matrix::translation(0., 0., 5.)
            * Matrix::rotation_y(-PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    left_wall.set_material(floor_and_walls_material);

    let mut right_wall = plane!();
    right_wall.set_transform(
        Matrix::translation(0., 0., 5.)
            * Matrix::rotation_y(PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    right_wall.set_material(floor_and_walls_material);

    let mut middle = sphere!();
    middle.set_transform(Matrix::translation(-0.5, 1., 0.5));
    middle.set_material(
        MaterialBuilder::default()
            .reflective(0.2)
            .transparency(1.0)
            .refractive_index(1.5)
            .pattern(solid!(BLACK))
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10., 10., -10.), WHITE);

    let world = World::new(light_source, vec![floor, left_wall, right_wall, middle]);
    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(0., 1.5, -5.),
        point!(0., 1., 0.),
        vector!(0., 1., 0.),
    ));

    let canvas = camera.render(&world);
    canvas.save("/tmp/11b_refraction.png")?;

    Ok(())
}
