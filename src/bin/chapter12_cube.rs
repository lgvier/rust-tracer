use rust_tracer::{
    camera::Camera,
    checkers_pattern,
    color::{BLACK, BLUE, GREEN, RED, WHITE, YELLOW},
    cube,
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    plane, point, solid, sphere, vector,
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

    let mut left_obj = sphere!();
    left_obj.set_transform(Matrix::translation(-1.5, 1., 0.5));
    left_obj.set_material(
        MaterialBuilder::default()
            .ambient(0.01)
            .diffuse(0.01)
            .reflective(0.9)
            .pattern(solid!(BLACK))
            .build()
            .unwrap(),
    );

    let mut right_obj = cube!();
    right_obj.set_transform(Matrix::translation(1.1, 1., 2.));
    right_obj.set_material(
        MaterialBuilder::default()
            .reflective(0.2)
            .transparency(0.2)
            .refractive_index(1.5)
            .pattern(solid!(BLUE))
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10., 10., -10.), WHITE);

    let world = World::new(
        light_source,
        vec![floor, left_wall, right_wall, left_obj, right_obj],
    );
    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(3., 1.5, -5.),
        point!(0., 1., 0.),
        vector!(0., 1., 0.),
    ));

    let canvas = camera.render(&world, true);
    canvas.save("/tmp/12_cube.png")?;

    Ok(())
}
