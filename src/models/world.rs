extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use super::Vector;
use super::Object;
use super::shape::*;

use crate::collision;

pub struct World {
    pub gl: Option<GlGraphics>,
    pub objects: Vec<Object>,
    pub size: Vector,
}

impl World {
    pub fn new(gl: Option<GlGraphics>, objects: Vec<Object>, size: Vector) -> World {
        let mut world = World {gl, objects, size};

        for _object in world.objects.iter_mut() {
            //object.forces.insert("Gravity".to_string(), Vector::new(0.0, -9.81));
        }

        world
    }

    pub fn render(&mut self, args: &RenderArgs) {
        if self.gl.is_some() {
            use graphics::*;

            self.gl.as_mut().unwrap().draw(args.viewport(), |c, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                for object in self.objects.iter_mut() {
                    object.render(args, c, gl, self.size);
                }
            });
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for object in self.objects.iter_mut() {
            object.update(args);
        }

        self.objects.sort_by(|a, b| a.z_index.total_cmp(&b.z_index));

        self.check_collisions();
    }

    pub fn check_collisions(&mut self) {
        for i in 0..self.objects.len() {
            for j in i+1..self.objects.len() {
                let object1 = self.objects.get(i).unwrap();
                let object2 = self.objects.get(j).unwrap();
                
                if let Shape::Particle(_p1) = object1.shape {
                    if let Shape::Particle(_p2) = object2.shape {
                        if (object1.position - object2.position).magnitude() < 2.0 {
                            let (v1, v2) = collision::pp_collision(object1, object2);

                            {
                                let obj_1 = self.objects.get_mut(i).unwrap();
                                obj_1.velocity = v1;
                            }
                            {
                                let obj_2 = self.objects.get_mut(j).unwrap();
                                obj_2.velocity = v2;
                            }
                        }
                    }
                }
            }
        }
    }
}