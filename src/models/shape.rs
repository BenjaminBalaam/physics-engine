use super::Vector;

#[derive(Clone)]
pub struct Shape {
    pub points: Vec<Vector>,
}

impl Shape {
    pub fn new(points: Vec<Vector>) -> Shape {
        return Shape {points};
    }

    pub fn centre(&mut self) {
        let mut total_x = 0.0;
        let mut total_y = 0.0;

        for point in self.points.iter() {
            total_x += point.x;
            total_y += point.y;
        }

        let average_x = total_x / (self.points.len() as f64);
        let average_y = total_y / (self.points.len() as f64);

        for point in self.points.iter_mut() {
            point.x -= average_x;
            point.y -= average_y;
        }
    }

    pub fn contains(&mut self, position: Vector) -> bool {
        let mut lines: Vec<(Vector, Vector)> = vec![];

        for i in 0..(self.points.len() - 1) {
            lines.push((self.points[i], self.points[i+1]));
        }

        let mut outside: Vector = Vector::new(0.0, 0.0);

        for point in self.points.iter_mut() {
            if point.x > outside.x {
                outside.x = point.x;
            }
            if point.y > outside.y {
                outside.y = outside.y;
            }
        }

        outside.x += 1.0;
        outside.y += 1.0;

        let direction: Vector = position - outside;

        let mut crosses = 0;

        for line in lines {
            // if (direction.x + line.0.x - line.1.x) == 0.0 || (direction.y + line.0.y - line.1.y) == 0.0 {
            //     continue;
            // }

            let alpha = direction.x * (line.0.y - line.1.y) - direction.y * (line.0.x - line.1.x);

            if alpha == 0.0 {
                continue;
            }

            let lambda = ((line.0.y - line.1.y) * (line.0.x - position.x) + (line.1.x - line.0.x) * (line.0.y - position.y)) / alpha;
            // let mu = (-direction.y * (line.0.x - position.x) + direction.x * (line.0.y - position.y)) / alpha;

            let crossing_point = position + direction * lambda;

            if Self::point_on_line(line, crossing_point) {
                crosses += 1;
            }
        }

        if crosses % 2 == 0 {
            return false;
        }
        else {
            return true;
        }
    }

    fn point_on_line(line: (Vector, Vector), point: Vector) -> bool {
        if line.0.x <= line.1.x {
            if !(line.0.x < point.x && line.1.x > point.x) {
                return false;
            }
        } else {
            if !(line.0.x > point.x && line.1.x < point.x) {
                return false;
            }
        }

        if line.0.y <= line.1.y {
            if !(line.0.y < point.y && line.1.y > point.y) {
                return false;
            }
        } else {
            if !(line.0.y > point.y && line.1.y < point.y) {
                return false;
            }
        }

        return true;
    }
}