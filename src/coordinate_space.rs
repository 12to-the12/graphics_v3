pub trait Orientation {
    fn get_orientation(&self);
}

/// encodes an orientation in polar coordinates
pub struct Polar {
    pub θ: f32,
    pub φ: f32,
}

/// allows for objects to be defined in spaces that differ from global
/// in the end, everything is compiled to global anyways
struct CoordinateSystem {

}