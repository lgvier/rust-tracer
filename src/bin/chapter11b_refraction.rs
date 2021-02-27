use rust_tracer::{
    camera::Camera,
    checkers_pattern,
    color::{BLACK, GREEN, RED, WHITE, YELLOW},
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
    let mut floor = plane!();
    floor.set_transform(Matrix::scaling(10., 0.01, 10.));
    {
        let mut pattern = checkers_pattern!(BLACK, WHITE);
        pattern.set_transform(Matrix::scaling(0.05, 0.05, 0.05));
        let material = MaterialBuilder::default()
            .pattern(pattern)
            .specular(0.2)
            .reflective(0.5)
            .build()
            .unwrap();
        floor.set_material(material);
    }

    let mut left_wall = plane!();
    left_wall.set_transform(
        Matrix::translation(0., 0., 5.)
            * Matrix::rotation_y(-PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    {
        let mut pattern = checkers_pattern!(RED, WHITE);
        pattern.set_transform(Matrix::scaling(0.05, 0.05, 0.05));
        let material = MaterialBuilder::default().pattern(pattern).build().unwrap();
        left_wall.set_material(material);
    }

    let mut right_wall = plane!();
    right_wall.set_transform(
        Matrix::translation(0., 0., 5.)
            * Matrix::rotation_y(PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    {
        let mut pattern = checkers_pattern!(GREEN, YELLOW);
        pattern.set_transform(Matrix::scaling(0.05, 0.05, 0.05));
        let material = MaterialBuilder::default().pattern(pattern).build().unwrap();
        right_wall.set_material(material);
    };

    let mut left_ball = sphere!();
    left_ball.set_transform(Matrix::translation(-1.5, 1., 0.5));
    left_ball.set_material(
        MaterialBuilder::default()
            .ambient(0.01)
            .diffuse(0.01)
            .reflective(0.9)
            .transparency(1.0)
            .refractive_index(1.5)
            .pattern(solid!(BLACK))
            .build()
            .unwrap(),
    );

    let mut right_ball = sphere!();
    right_ball.set_transform(Matrix::translation(1.1, 1., 0.5));
    right_ball.set_material(
        MaterialBuilder::default()
            .reflective(0.9)
            .transparency(1.0)
            .refractive_index(1.5)
            .pattern(solid!(BLACK))
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10., 10., -10.), WHITE);

    let world = World::new(
        light_source,
        vec![floor, left_wall, right_wall, left_ball, right_ball],
    );
    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(3., 1.5, -5.),
        point!(0., 1., 0.),
        vector!(0., 1., 0.),
    ));

    let canvas = camera.render(&world);
    canvas.save("/tmp/11b_refraction.png")?;

    Ok(())
}
