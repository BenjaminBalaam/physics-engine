extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashMap;

use piston::input::UpdateArgs;

use crate::models::Vector;
use crate::models::World;
use crate::models::Object;
use crate::models::shape::*;

#[test]
fn test_pp_collision_downwards_static() {
    let objects = vec![
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(0.0, 10.0),
            0.0,
            Vector::new(0.0, -1.0),
            HashMap::new(),
            0.0,
        ),
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(0.0, 0.0),
            0.0,
            Vector::new(0.0, 0.0),
            HashMap::new(),
            1.0,
        ),
    ];

    let mut world = World::new(Option::None, objects, Vector::new(20.0, 20.0));

    for _ in 0..10 {
        world.update(&UpdateArgs {dt: 1.0});
    }

    assert!(world.objects[0].velocity.y > 0.0);
    assert!(world.objects[1].velocity.y < 0.0);
}

#[test]
fn test_pp_collision_left_static() {
    let objects = vec![
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(0.0, 0.0),
            0.0,
            Vector::new(0.0, 0.0),
            HashMap::new(),
            0.0,
        ),
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(10.0, 0.0),
            0.0,
            Vector::new(-1.0, 0.0),
            HashMap::new(),
            1.0,
        ),
    ];

    let mut world = World::new(Option::None, objects, Vector::new(20.0, 20.0));

    for _ in 0..10 {
        world.update(&UpdateArgs {dt: 1.0});
    }

    assert!(world.objects[0].velocity.x < 0.0);
    assert!(world.objects[1].velocity.x > 0.0);
}

#[test]
fn test_pp_collision_dynamic() {
    let objects = vec![
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(30.0, -10.0),
            0.0,
            Vector::new(-3.0, 1.0),
            HashMap::new(),
            0.0,
        ),
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(-20.0, 50.0),
            0.0,
            Vector::new(2.0, -5.0),
            HashMap::new(),
            1.0,
        ),
    ];

    let mut world = World::new(Option::None, objects, Vector::new(20.0, 20.0));

    for _ in 0..10 {
        world.update(&UpdateArgs {dt: 1.0});
    }

    assert!(world.objects[0].velocity.x < 0.0);
    assert!(world.objects[0].velocity.y < 0.0);
    assert!(world.objects[1].velocity.x > 0.0);
    assert!(world.objects[1].velocity.y > 0.0);
}

#[test]
fn test_pp_collision_passive() {
    let objects = vec![
        Object::new(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            1.0,
            Vector::new(0.0, 10.0),
            0.0,
            Vector::new(0.0, -1.0),
            HashMap::new(),
            0.0,
        ),
        Object::new_passive(
            [0.0, 0.0, 0.0, 1.0],
            Shape::Particle(Particle::new()),
            Vector::new(0.0, 0.0),
            0.0,
            Vector::new(0.0, 0.0),
            HashMap::new(),
            1.0,
        ),
    ];

    let mut world = World::new(Option::None, objects, Vector::new(20.0, 20.0));

    for _ in 0..10 {
        world.update(&UpdateArgs {dt: 1.0});
    }

    assert!(world.objects[0].velocity.y > 0.0);
    assert!(world.objects[1].velocity.y == 0.0);
}