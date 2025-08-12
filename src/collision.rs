use std::f64::consts::PI;

use crate::models::Vector;
use crate::models::Object;

pub fn vector_pow(v: Vector, pow: f64) -> Vector {
    Vector::new(v.x.powf(pow), v.y.powf(pow))
}

fn solve_for_velocities(m1: f64, m2: f64, u1: Vector, u2: Vector) -> (Vector, Vector) {
    let total_momentum = u1 * m1 + u2 * m2;
    let total_energy = vector_pow(u1, 2.0) * m1 + vector_pow(u2, 2.0) * m2;

    /*
    Formula for solution to simutaneous equation:
        m1 * u1 + m2 * u2 = m1 * v1 + m2 * v2
        1/2 * m1 * u1^2 + 1/2 * m2 * u2^2 = 1/2 * m1 * v1^2 + 1/2 * m2 * v2^2
    (Calculated by hand)
    */
    
    let mut v2 = ( total_momentum * 2.0 * m2 - vector_pow( vector_pow(total_momentum, 2.0) * 4.0 * m2.powf(2.0) + total_energy * 4.0 * m1.powf(2.0) * m2 + total_energy * 4.0 * m1 * m2.powf(2.0), 0.5)) / (m1 * m2 * 2.0 + m2.powf(2.0));

    if (u2 - v2).angle() < PI / 2.0 {
        v2 = ( total_momentum * 2.0 * m2 + vector_pow( vector_pow(total_momentum, 2.0) * 4.0 * m2.powf(2.0) + total_energy * 4.0 * m1.powf(2.0) * m2 + total_energy * 4.0 * m1 * m2.powf(2.0), 0.5)) / (m1 * m2 * 2.0 + m2.powf(2.0));
    }
    
    let v1 = (total_momentum - v2 * m2) / m1;

    return (v1, v2);
}

pub fn pp_collision(particle1: &Object, particle2: &Object) -> (Vector, Vector) {
    if particle1.passive || particle2.passive {
        return (-particle1.velocity, -particle2.velocity);
    }

    return solve_for_velocities(particle1.mass, particle2.mass, particle1.velocity, particle2.velocity);
}

pub fn ps_collision(particle: &Object, shape: &Object) -> ((Vector, f64), (Vector, f64)) {
    let p1 = Vector::new(particle.position.x + 1.0, particle.position.y);
    let p2 = Vector::new(particle.position.x - 1.0, particle.position.y);

    let u1 = particle.velocity;
    let u2 = particle.velocity;

    let u3 = shape.velocity + (p1 - shape.position) * shape.angular_velocity;
    let u4 = shape.velocity + (p1 - shape.position) * shape.angular_velocity;

    return ((Vector::new(0.0, 0.0), 0.0), (Vector::new(0.0, 0.0), 0.0));
}