use crate::{
    geometry::primitives::Vector,
    lighting::{black_spectra, const_spectra, monochroma_spectra, RadiometricUnit, Spectra},
    ray_tracing::rendering_equation::{lamberts_law, BRDF},
};
use std::f32::consts::PI;

#[derive(Clone, Debug, PartialEq)]
pub enum ShaderNode {
    Void,
    PBR(PBR),
    Literal(Spectra),
}

/// physical object in space with associated data
// I want shaders to simply be a trait
// any function that takes in all the necessary data and returns a light value is a shader
#[derive(Clone, Debug, PartialEq)]
pub struct PBR {
    metallic: f32,
    roughness: f32,
}

// pub trait PBR {
//   fn shade(&self) -> Spectra;
// }

impl BRDF for PBR {
    fn rendering_equation(
        &self,
        x: &Vector,                          // position vector of equation
        ω_i: &Vector,                        // vector to light
        ω_o: &Vector,                        // light exit path
        normal: &Vector,                     // surface normal
        incoming_radiant_intensity: Spectra, // the radiant flux of the lightsource encoded as a spectrum
    ) -> Spectra {
        let r = ω_i.magnitude();

        let r_o = ω_o.magnitude();
        let incident_factor = lamberts_law(&ω_i, &normal); // cos(θ)
        let outgoing_incident_factor = lamberts_law(&ω_o, &normal); // cos(θ)
        let surface_irradiance =
            (1. / (r * r)) * (incident_factor * incoming_radiant_intensity.clone());

        // irradiance over a hemisphere divided by outgoing incidence
        let isotrophic_surface_radiance =
            (1. / (2. * PI)) / outgoing_incident_factor * surface_irradiance;
        // with that value we know exactly how bright our surface is in the viewing direction
        // now we need the distance to observer and sensor area to compute watts delivered
        let observer_irradiance = ((1. / (r_o * r_o)) * isotrophic_surface_radiance)
            .set_unit(RadiometricUnit::Irradiance);

        let simple: Spectra = incident_factor * incoming_radiant_intensity;
        return simple;
    }
}

impl PBR {
    pub const fn new() -> PBR {
        PBR {
            metallic: 0.23,
            roughness: 0.23,
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
