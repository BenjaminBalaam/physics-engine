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
}

impl World {
    pub fn new(gl: GlGraphics, objects: Vec<Object>) -> World {
        World {gl, objects}
    }

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

            object.velocity += Vector::new(0.0, 0.01);
        }
    }
}