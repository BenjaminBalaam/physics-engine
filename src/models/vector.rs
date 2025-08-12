use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub const fn new(x: f64, y: f64) -> Vector {
        Vector {x, y}
    }

    pub fn magnitude(&mut self) -> f64 {
        return (self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5);
    }

    pub fn angle(&mut self) -> f64 {
        return f64::atan2(self.y, self.x);
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        return Vector::new(-self.x, -self.y);
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector::new(self.x * other, self.y * other)
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector::new(self.x / other, self.y / other)
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}