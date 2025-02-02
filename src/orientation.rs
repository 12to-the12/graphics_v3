use crate::primitives::{Vector};

#[derive(Clone, Debug)]
pub struct Orientation {
    top: Vector,
    front: Vector,
}


// Basis Vectors
pub const I: Vector = Vector {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const J: Vector = Vector {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const K: Vector = Vector {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const NI: Vector = Vector {
    x: -1.,
    y: 0.,
    z: 0.,
};
pub const NJ: Vector = Vector {
    x: -1.,
    y: 0.,
    z: 0.,
};
pub const NK: Vector = Vector {
    x: -1.,
    y: 0.,
    z: 0.,
};

// oriented
pub const RIGHT: Orientation = Orientation { top: J, front: I };
pub const UP: Orientation = Orientation { top: NK, front: J };
pub const FORWARDS: Orientation = Orientation { top: J, front: K };
