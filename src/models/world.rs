extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use super::Object;

pub struct World {
    pub gl: GlGraphics, // OpenGL drawing backend
    pub objects: Vec<Object>,
}

impl World {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);

        for object in self.objects.iter() {
            self.gl.draw(args.viewport(), |c, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                let transform = c.transform
                    .trans(object.position.x, object.position.y)
                    .rot_rad(object.rotation);
                
                rectangle(object.colour, square, transform, gl);
            });
        }
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        // // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;

        // // Fall down over time
        // self.height -= 0.5;
    }
}