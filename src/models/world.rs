extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use super::Vector;
use super::Object;

pub struct World {
    pub gl: GlGraphics,
    pub objects: Vec<Object>,
    pub size: Vector,
}

impl World {
    pub fn new(gl: GlGraphics, objects: Vec<Object>, size: Vector) -> World {
        let mut world = World {gl, objects, size};

        for object in world.objects.iter_mut() {
            object.forces.push(Vector::new(0.0, -9.81));
        }

        world
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for object in self.objects.iter_mut() {
                object.render(args, c, gl, self.size);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for object in self.objects.iter_mut() {
            object.update(args);
        }

        self.objects.sort_by(|a, b| a.z_index.total_cmp(&b.z_index));
    }
}