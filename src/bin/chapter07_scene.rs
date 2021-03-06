use std::f64::consts::PI;

use rust_tracer::{
    camera::Camera, color::WHITE, light::PointLight, material::MaterialBuilder, matrix::Matrix,
    point, solid, sphere, vector, world::World,
};

fn main() -> std::io::Result<()> {
    let floor_and_walls_material = MaterialBuilder::default()
        .pattern(solid!(1, 0.9, 0.9))
        .specular(0)
        .build()
        .unwrap();

    let mut floor = sphere!();
    floor.set_transform(Matrix::scaling(10, 0.01, 10));
    floor.set_material(floor_and_walls_material);

    let mut left_wall = sphere!();
    left_wall.set_transform(
        Matrix::translation(0, 0, 5)
            * Matrix::rotation_y(-PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10, 0.01, 10),
    );
    left_wall.set_material(floor_and_walls_material);

    let mut right_wall = sphere!();
    right_wall.set_transform(
        Matrix::translation(0, 0, 5)
            * Matrix::rotation_y(PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10, 0.01, 10),
    );
    right_wall.set_material(floor_and_walls_material);

    let mut middle = sphere!();
    middle.set_transform(Matrix::translation(-0.5, 1, 0.5));
    middle.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(solid!(0.1, 1, 0.5))
            .build()
            .unwrap(),
    );

    let mut left = sphere!();
    left.set_transform(Matrix::translation(-1.5, 0.33, -0.55) * Matrix::scaling(0.33, 0.33, 0.33));
    left.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(solid!(1, 0.8, 0.1))
            .build()
            .unwrap(),
    );

    let mut right = sphere!();
    right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    right.set_material(
        MaterialBuilder::default()
            .diffuse(0.7)
            .specular(0.3)
            .pattern(solid!(0.5, 1, 0.1))
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10, 10, -10), WHITE);

    let world = World::new(
        light_source,
        vec![floor, left_wall, right_wall, middle, left, right],
    );

    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(0, 1.5, -5),
        point!(0, 1, 0),
        vector!(0, 1, 0),
    ));

    let canvas = camera.render(&world, true);
    canvas.save("/tmp/07_scene.png")?;
    Ok(())
}
