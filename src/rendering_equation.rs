use crate::primitives::Vector;

pub fn weakening_function(ω_i: &Vector,normal: &Vector) -> f32 {
    let divisor: f32 = ω_i.magnitude()*normal.magnitude();

    return ω_i.dot(normal) / divisor;
}