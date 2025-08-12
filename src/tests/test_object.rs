use std::collections::HashMap;

use piston::UpdateArgs;

use crate::models::Vector;
use crate::models::Object;
use crate::models::shape::*;

#[test]
fn test_add_force() {
    let mut obj = Object::new(
        [0.0, 0.0, 0.0, 1.0],
        Shape::Particle(Particle::new()),
        1.0,
        Vector::new(0.0, 0.0),
        0.0,
        Vector::new(0.0, 0.0),
        HashMap::new(),
        0.0,
    );

    obj.add_force("Gravity".to_string(), Vector::new(0.0, -9.81));

    let force = obj.forces.get("Gravity");

    assert_eq!(*force.unwrap(), Vector::new(0.0, -9.81));
}

#[test]
fn test_remove_force() {
    let mut obj = Object::new(
        [0.0, 0.0, 0.0, 1.0],
        Shape::Particle(Particle::new()),
        1.0,
        Vector::new(0.0, 0.0),
        0.0,
        Vector::new(0.0, 0.0),
        HashMap::new(),
        0.0,
    );

    obj.add_force("Gravity".to_string(), Vector::new(0.0, -9.81));

    obj.remove_force("Gravity".to_string());

    assert_eq!(obj.forces.contains_key("Gravity"), false);
}

#[test]
fn test_update() {
    let mut obj = Object::new(
        [0.0, 0.0, 0.0, 1.0],
        Shape::Particle(Particle::new()),
        1.0,
        Vector::new(0.0, 0.0),
        0.0,
        Vector::new(0.0, -9.81),
        HashMap::new(),
        0.0,
    );

    obj.add_force("Gravity".to_string(), Vector::new(0.0, -9.81));

    obj.update(&UpdateArgs {dt: 1.0});

    assert_eq!(obj.position, Vector::new(0.0, -9.81));
    assert_eq!(obj.velocity, Vector::new(0.0, -19.62));
}

#[test]
fn test_update_passive() {
    let mut obj = Object::new_passive(
        [0.0, 0.0, 0.0, 1.0],
        Shape::Particle(Particle::new()),
        Vector::new(0.0, 0.0),
        0.0,
        Vector::new(0.0, -9.81),
        HashMap::new(),
        0.0,
    );

    obj.add_force("Gravity".to_string(), Vector::new(0.0, -9.81));

    obj.update(&UpdateArgs {dt: 1.0});

    assert_eq!(obj.position, Vector::new(0.0, 0.0));
    assert_eq!(obj.velocity, Vector::new(0.0, -9.81));
}