use std::{collections::HashMap, f64::consts::PI};

use opengl_graphics::GlGraphics;
use piston::{input::UpdateArgs, RenderArgs};
use graphics::*;

use super::Vector;
use super::Shape;

pub struct Object {
    pub colour: [f32; 4],
    pub shape: Shape,
    pub mass: f64,
    pub position: Vector,
    pub rotation: f64,
    pub velocity: Vector,
    pub angular_velocity: f64,
    pub forces: HashMap<String, Vector>,
    pub z_index: f64,
    pub passive: bool,
}

impl Object {
    pub fn new(colour: [f32; 4], shape: Shape, mass: f64, position: Vector, rotation: f64, velocity: Vector, angular_velocity: f64, forces: HashMap<String, Vector>, z_index: f64) -> Object {
        Object {colour, shape, mass, position, rotation, velocity, angular_velocity, forces, z_index, passive: false}
    }

    pub fn new_passive(colour: [f32; 4], shape: Shape, position: Vector, rotation: f64, velocity: Vector, angular_velocity: f64, forces: HashMap<String, Vector>, z_index: f64) -> Object {
        Object {colour, shape, mass: f64::INFINITY, position, rotation, velocity, angular_velocity, forces, z_index, passive: true}
    }

    pub fn add_force(&mut self, name: String, force: Vector) {
        self.forces.insert(name, force);
    }

    pub fn remove_force(&mut self, name: String) {
        self.forces.remove(&name);
    }

    pub fn get_points(&mut self) -> Vec<Vector> {
        let points: Vec<Vector> = self.shape.points.clone();

        for point in self.shape.points.iter_mut() {
            let x = point.x;
            let y = point.y;

            point.x = self.rotation.cos() * x - self.rotation.sin() * y;
            point.y = self.rotation.sin() * x + self.rotation.cos() * y;

            point.x += x;
            point.y += y;
        }

        return points;
    }

    pub fn render(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics, world_size: Vector) {
        let shape = rectangle::square(0.0, 0.0, 0.0);

        let transform = c.transform
            .trans(
                self.position.x * args.window_size[0] / world_size.x + args.window_size[0] / 2.0,
                -self.position.y * args.window_size[1] / world_size.y + args.window_size[1] / 2.0,
            )
            .rot_rad(self.rotation);
        
            circle_arc(self.colour, 10.0, 0.0, PI * 2.0, shape, transform, gl);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if !self.passive {
            self.position += self.velocity * args.dt;

            let mut acceleration = Vector::new(0.0, 0.0);

            for force in self.forces.iter() {
                acceleration += *force.1;
            }

            self.velocity += acceleration * args.dt;
        }
    }
}