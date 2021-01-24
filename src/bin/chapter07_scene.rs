use rust_tracer::{
    camera::Camera,
    color,
    color::{Color, WHITE},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    point, point_light,
    shapes::{Shape, Sphere},
    sphere,
    tuple::Tuple,
    vector,
    world::World,
};
use std::f64::consts::PI;

fn main() -> std::io::Result<()> {
    let floor_and_walls_material = Material::default()
        .with_color(color!(1., 0.9, 0.9))
        .with_specular(0.);

    let mut floor = sphere!();
    floor.set_transform(Matrix::scaling(10., 0.01, 10.));
    floor.set_material(floor_and_walls_material);

    let mut left_wall = sphere!();
    left_wall.set_transform(
        Matrix::translation(0., 0., 5.)
            * Matrix::rotation_y(-PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    left_wall.set_material(floor_and_walls_material);

    let mut right_wall = sphere!();
    right_wall.set_transform(
        Matrix::translation(0., 0., 5.)
            * Matrix::rotation_y(PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    right_wall.set_material(floor_and_walls_material);

    let spheres_material = Material::default().with_diffuse(0.7).with_specular(0.3);

    let mut middle = sphere!();
    middle.set_transform(Matrix::translation(-0.5, 1., 0.5));
    middle.set_material(spheres_material.with_color(color!(0.1, 1., 0.5)));

    let mut left = sphere!();
    left.set_transform(Matrix::translation(-1.5, 0.33, -0.55) * Matrix::scaling(0.33, 0.33, 0.33));
    left.set_material(spheres_material.with_color(color!(1., 0.8, 0.1)));

    let mut right = sphere!();
    right.set_transform(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5));
    right.set_material(spheres_material.with_color(color!(0.5, 1., 0.1)));

    let light_source = point_light!(point!(-10., 10., -10.), WHITE);

    let world = World::new(
        light_source,
        vec![
            Shape::Sphere(floor),
            Shape::Sphere(left_wall),
            Shape::Sphere(right_wall),
            Shape::Sphere(middle),
            Shape::Sphere(left),
            Shape::Sphere(right),
        ],
    );
    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(0., 1.5, -5.),
        point!(0., 1., 0.),
        vector!(0., 1., 0.),
    ));

    let canvas = camera.render(&world);
    canvas.save("/tmp/07_scene.png")?;
    Ok(())
}
