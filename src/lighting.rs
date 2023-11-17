use crate::primitives::Vertex;
pub struct Light {
    pub position: Vertex,
    pub power: f32, // to be replaced with something more comprehensive
}

pub fn sun_light(position: Vertex, power: f32) -> Light{
    Light { position, power }
}
