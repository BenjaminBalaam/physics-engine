use std::f64::consts::PI;

use crate::models::Vector;
use crate::models::Object;

pub fn vector_pow(v: Vector, pow: f64) -> Vector {
    Vector::new(v.x.powf(pow), v.y.powf(pow))
}

pub fn pp_collision(particle1: &Object, particle2: &Object) -> (Vector, Vector) {
    if particle1.passive || particle2.passive {
        return (-particle1.velocity, -particle2.velocity);
    }

    let m1 = particle1.mass;
    let u1 = particle1.velocity;

    let m2 = particle2.mass;
    let u2 = particle2.velocity;

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