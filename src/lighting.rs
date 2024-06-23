use crate::primitives::{vector, Vector, Vertex};
#[derive(Clone)]
pub struct Light {
    pub position: Vertex,
    pub direction: Vector,
    pub power: f32, // to be replaced with something more comprehensive
}

pub fn sun_light(position: Vertex, direction: Vector, power: f32) -> Light {
    Light {
        position,
        direction,
        power,
    }
}
