use crate::lighting::{black_spectra, Spectra};

#[derive(Clone)]
pub enum ShaderType {
    Void,
    MatteWhite
}


/// physical object in space with associated data
// I want shaders to simply be a trait
// any function that takes in all the necessary data and returns a light value is a shader
pub struct Void {
    pub color: f32,
}

pub trait Shader {
  fn shade(&self) -> Spectra;
}

impl Shader for Void {
    fn shade(&self) -> Spectra {
        black_spectra(crate::lighting::RadiometricUnit::Radiance)
    }
}

// /// the defaults
// pub const OBJECT: Object = Object {
//     position: ORIGIN,
//     _orientation: _UP,
//     _scale: 1.,
//     _children: Vec::new(),
//     _shaders: Vec::new(),
//     meshes: Vec::new(),
// };

// struct MeshPool {
//     meshes: Vec
// }

// #[cfg(test)]
// mod tests {
//     use crate::geometry::primitives::_unit_cube;

//     use super::{Object, OBJECT};

//     /// useful table: https://www.nikonians.org/reviews/fov-tables
//     #[test]
//     fn test_radius() {
//         let mymesh = _unit_cube();
//         let myobject: Object = Object {
//             meshes: vec![mymesh],
//             ..OBJECT
//         };
//         assert_eq!(myobject.get_radius(), f32::sqrt(3.));
//     }
// }
