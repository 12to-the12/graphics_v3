use crate::geometry::primitives::Vector;

#[derive(Clone, Debug)]
pub struct Orientation {
    _top: Vector,
    _front: Vector,
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
pub const _K: Vector = Vector {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const _NI: Vector = Vector {
    x: -1.,
    y: 0.,
    z: 0.,
};
pub const _NJ: Vector = Vector {
    x: -1.,
    y: 0.,
    z: 0.,
};
pub const _NK: Vector = Vector {
    x: -1.,
    y: 0.,
    z: 0.,
};

// oriented
pub const RIGHT: Orientation = Orientation { _top: J, _front: I };
pub const _UP: Orientation = Orientation { _top: _NK, _front: J };
pub const _FORWARDS: Orientation = Orientation { _top: J, _front: _K };
