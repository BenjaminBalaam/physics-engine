extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::ops;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use super::*;

pub struct World {
    pub gl: GlGraphics, // OpenGL drawing backend
    pub objects: Vec<Object>,
}

impl World {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            for object in self.objects.iter() {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                let square = rectangle::square(-25.0, -25.0, 50.0);

                let transform = c.transform
                    .trans(object.position.x, object.position.y)
                    .rot_rad(object.rotation);
                
                rectangle(object.colour, square, transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for object in self.objects.iter_mut() {
            object.update(args);

            object.velocity += Vector {x: 0.0, y: 0.01};
        }
    }
}

pub struct Object {
    pub colour: [f32; 4],
    pub position: Vector,
    pub rotation: f64,
    pub velocity: Vector,
    pub passive: bool,
}

impl Object {
    pub fn new(colour: [f32; 4], position: Vector, rotation: f64, velocity: Vector) {
        Object {colour: colour, position: position, rotation: rotation, velocity: velocity, passive: false}
    }

    pub fn new_passive(colour: [f32; 4], position: Vector, rotation: f64, velocity: Vector) {
        Object {colour: colour, position: position, rotation: rotation, velocity: velocity, passive: true}
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if (!self.passive) {
            self.position += self.velocity;
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}