
pub struct World {
    gl: GlGraphics, // OpenGL drawing backend
    objects: Vec<Object>,
}

impl World {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);

        for object in objects.iter() {
            self.gl.draw(args.viewport(), |c, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                let transform = c.transform
                    .trans(object.position.x, object.position.y)
                    .rot_rad(object.rotation);
                
                rectangle(object.colour, square, transform, gl);
            });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;

        // // Fall down over time
        // self.height -= 0.5;
    }
}