use super::Vector;

pub enum Shape {
    Particle(Particle),
    Rectangle(Rectangle)
}

impl Shape {
    pub fn get_points(&mut self) -> Vec<Vector> {
        match self {
            Shape::Particle(particle) => particle.get_points(),
            Shape::Rectangle(rectangle) => rectangle.get_points(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Particle {
    
}

impl Particle {
    pub fn new() -> Particle {
        return Particle {};
    }

    fn get_points(&mut self) -> Vec<Vector> {
        return vec![Vector::new(0.0, 0.0)];
    }

    fn _centre(&mut self) {

    }
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    point1: Vector,
    point4: Vector,
}

impl Rectangle {
    pub fn new(point1: Vector, point4: Vector) -> Rectangle {
        let mut rect = Rectangle {point1, point4};

        rect.centre();

        return rect;
    }

    fn get_points(&mut self) -> Vec<Vector> {
        return vec![self.get_point1(), self.get_point2(), self.get_point3(), self.get_point4()];
    }

    fn centre(&mut self) {
        let average_x = (self.get_point4().x - self.get_point1().x) / 2.0;
        let average_y = (self.get_point4().y - self.get_point1().y) / 2.0;

        self.point1 = Vector::new(self.get_point1().x - average_x, self.get_point1().y - average_y);
        self.point4 = Vector::new(self.get_point4().x - average_x, self.get_point4().y - average_y);
    }

    pub fn get_point1(&mut self) -> Vector {
        return self.point1;
    }
    pub fn get_point2(&mut self) -> Vector {
        return Vector::new(self.point1.x, self.point4.y);
    }
    pub fn get_point3(&mut self) -> Vector {
        return Vector::new(self.point4.x, self.point1.y);
    }
    pub fn get_point4(&mut self) -> Vector {
        return self.point4;
    }
}