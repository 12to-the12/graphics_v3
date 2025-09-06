use crate::{
    geometry::primitives::Vector,
    lighting::{black_spectra, const_spectra, monochroma_spectra, RadiometricUnit, Spectra},
    ray_tracing::rendering_equation::{lamberts_law, BRDF},
};
use std::f32::consts::PI;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum MaterialType {
    Void,
    MatteWhite,
    BSDF,
}

/// physical object in space with associated data
// I want shaders to simply be a trait
// any function that takes in all the necessary data and returns a light value is a shader
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Material {
    material_type: MaterialType,
}

// pub trait Material {
//   fn shade(&self) -> Spectra;
// }

impl BRDF for Material {
    fn rendering_equation(
        &self,
        x: &Vector,       // position vector of equation
        ω_i: &Vector,     // vector to light
        ω_o: &Vector,     // light exit path
        normal: &Vector,  // surface normal
        incoming_radiance: Spectra, // the radiant flux of the lightsource encoded as a spectrum
    ) -> Spectra {

        let lamberts_law = lamberts_law(&ω_i, &normal);
        let outgoing_radiance: Spectra =
            lamberts_law * incoming_radiance;
        return outgoing_radiance;
    }
}

impl Material {
    pub const fn new() -> Material {
        Material {
            material_type: MaterialType::MatteWhite,
        }
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
