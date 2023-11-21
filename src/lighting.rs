use crate::primitives::{vector, Vector, Vertex};
#[derive(Clone)]
pub struct Light {
    pub position: Vertex,
    pub direction: Vector,
    pub power: f32, // to be replaced with something more comprehensive
}

pub fn sun_light(position: Vertex, power: f32) -> Light {
    let direction = vector(0., -1., 0.);
    Light { position, direction, power }
}
