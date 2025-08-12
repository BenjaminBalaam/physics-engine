use piston::input::UpdateArgs;

use super::Vector;

pub struct Object {
    pub colour: [f32; 4],
    pub position: Vector,
    pub rotation: f64,
    pub velocity: Vector,
    pub passive: bool,
}

impl Object {
    pub fn new(colour: [f32; 4], position: Vector, rotation: f64, velocity: Vector) -> Object {
        Object {colour, position, rotation, velocity, passive: false}
    }

    pub fn new_passive(colour: [f32; 4], position: Vector, rotation: f64, velocity: Vector) -> Object {
        Object {colour, position, rotation, velocity, passive: true}
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        if !self.passive {
            self.position += self.velocity;
        }
    }
}