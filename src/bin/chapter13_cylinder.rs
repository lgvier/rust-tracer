use rand::Rng;
use std::{f64::consts::PI, str::FromStr};

use rust_tracer::{
    arena::Arena,
    camera::Camera,
    checkers_pattern,
    color::{Color, BLACK, WHITE},
    cone, cylinder,
    light::PointLight,
    material::MaterialBuilder,
    matrix::Matrix,
    plane, point, ring_pattern, solid, vector,
    world::World,
};

const COLOR_PALETTE: &[&str] = &["#E27D60", "#85DCB", "#E8A87C", "#C38D9E", "#41B3A3"];

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..COLOR_PALETTE.len() - 1);
    Color::from_str(COLOR_PALETTE[idx]).unwrap()
}

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
        let mut pattern = checkers_pattern!(random_color(), random_color());
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
        let mut pattern = checkers_pattern!(random_color(), random_color());
        pattern.set_transform(Matrix::scaling(0.05, 0.05, 0.05));
        let material = MaterialBuilder::default().pattern(pattern).build().unwrap();
        right_wall.set_material(material);
    };

    let mut back_wall = plane!();
    back_wall.set_transform(
        Matrix::translation(10., 0., -5.)
            * Matrix::rotation_y(-PI / 4.)
            * Matrix::rotation_x(PI / 2.)
            * Matrix::scaling(10., 0.01, 10.),
    );
    {
        let mut pattern = ring_pattern!(random_color(), random_color());
        pattern.set_transform(Matrix::scaling(0.05, 0.05, 0.05));
        let material = MaterialBuilder::default().pattern(pattern).build().unwrap();
        back_wall.set_material(material);
    };

    let mut left_obj = cylinder!(0., 1.);
    left_obj.set_transform(Matrix::translation(-3., 1., 0.5) * Matrix::rotation_z(PI / 2.));
    left_obj.set_material(
        MaterialBuilder::default()
            .ambient(0.01)
            .diffuse(0.01)
            .reflective(0.9)
            .pattern(solid!(random_color()))
            .build()
            .unwrap(),
    );

    let mut center_obj = cylinder!(0., 1., true);
    center_obj.set_transform(Matrix::translation(0.0, 1., 0.) * Matrix::rotation_z(PI / 1.3));
    center_obj.set_material(
        MaterialBuilder::default()
            .ambient(0.01)
            .diffuse(0.01)
            .reflective(0.9)
            .pattern(solid!(random_color()))
            .build()
            .unwrap(),
    );

    let mut right_obj = cylinder!();
    right_obj.set_transform(Matrix::translation(1.1, 1., 2.));
    right_obj.set_material(
        MaterialBuilder::default()
            .reflective(0.2)
            .transparency(0.2)
            .refractive_index(1.5)
            .pattern(solid!(random_color()))
            .build()
            .unwrap(),
    );

    let mut right_obj2 = cone!(-0.5, 0.5, true);
    right_obj2.set_transform(Matrix::translation(2.1, 0.5, 1.));
    right_obj2.set_material(
        MaterialBuilder::default()
            .reflective(0.2)
            .transparency(0.2)
            .refractive_index(1.5)
            .pattern(solid!(random_color()))
            .build()
            .unwrap(),
    );

    let light_source = PointLight::new(point!(-10., 10., -10.), WHITE);

    let mut arena = Arena::new();
    let object_ids = vec![
        arena.add(floor),
        arena.add(left_wall),
        arena.add(right_wall),
        arena.add(back_wall),
        arena.add(left_obj),
        arena.add(center_obj),
        arena.add(right_obj),
        arena.add(right_obj2),
    ];

    let world = World::new(light_source, arena, object_ids);

    let hsize = 800;
    let mut camera = Camera::new(hsize, hsize / 2, PI / 3.);
    camera.set_transform(Matrix::view_transform(
        point!(3., 1.5, -5.),
        point!(0., 1., 0.),
        vector!(0., 1., 0.),
    ));

    let canvas = camera.render(&world, true);
    canvas.save("/tmp/13_cylinder.png")?;

    Ok(())
}
