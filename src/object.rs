use crate::{
    geometry::{
        orientation::{Orientation, _UP},
        primitives::{Mesh, Vector, _ORIGIN},
    },
    ray_tracing::rendering_equation::Material,
};

/// physical object in space with associated data
#[derive(Clone)]
pub(crate) struct Object {
    pub position: Vector,
    pub orientation: Orientation,
    pub scale: f32,
    pub children: Vec<Object>,
    pub shaders: Vec<Material>,
    pub meshes: Vec<Mesh>,
    // & links to textures associated with it
}

/// the defaults
pub const OBJECT: Object = Object {
    position: _ORIGIN,
    orientation: _UP,
    scale: 1.,
    children: Vec::new(),
    shaders: Vec::new(),
    meshes: Vec::new(),
};

// struct MeshPool {
//     meshes: Vec
// }
