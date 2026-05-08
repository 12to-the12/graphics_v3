use crate::{
    geometry::primitives::Vector,
    lighting::{const_spectra, Radiance, RadiantIntensity, Spectra},
};
use std::{f32::consts::PI, fmt::Debug};

pub trait BRDF: Debug + Sync + Send {
    fn rendering_equation(
        &self,
        x: &Vector,      // position vector of equation
        ω_i: &Vector,    // vector to light
        ω_o: &Vector,    // light exit path
        normal: &Vector, // surface normal
        // incoming_radiant_intensity: Spectra, // the radiant flux of the lightsource encoded as a spectrum
        incoming_radiant_intensity: RadiantIntensity, // the radiant flux of the lightsource encoded as a spectrum
    ) -> Radiance;
}

/// Lambert's law of cosines
pub fn cosθ(ω: &Vector, normal: &Vector) -> f32 {
    let divisor: f32 = ω.magnitude() * normal.magnitude();

    ω.dot(normal) / divisor
}

/// physical object in space with associated data
// I want shaders to simply be a trait
// any function that takes in all the necessary data and returns a light value is a shader
#[derive(Clone, Debug, PartialEq)]
pub struct Diffuse {
    pub metallic: f32,
    pub roughness: f32,
    pub albedo: Spectra,
}

impl Default for Diffuse {
    fn default() -> Self {
        Diffuse {
            metallic: 0.0,
            roughness: 1.0,
            albedo: const_spectra(0.3),
        }
    }
}

impl BRDF for Diffuse {
    fn rendering_equation(
        &self,
        _x: &Vector,     // position vector of equation
        ω_i: &Vector,    // vector to light
        ω_o: &Vector,    // light exit path
        normal: &Vector, // surface normal
        // incoming_radiant_intensity: Spectra, // the radiant flux of the lightsource encoded as a spectrum
        incoming_radiant_intensity: RadiantIntensity,
    ) -> Radiance {
        // radiant intensity W/sr obeys cosine law
        // radiance W/sr/m^2 is the same in all directions

        let _incident_factor = cosθ(ω_i, normal); // per unit solid angle (area)
        let _outgoing_incident_factor = cosθ(ω_o, normal); // per unit solid angle (area
                                                           // let incoming: Spectra = incoming_radiant_intensity.s;
                                                           // let surface_irradiance: RadiantExitance = ((1. / (r * r))
                                                           //     * (incident_factor * incoming_radiant_intensity.0)
                                                           //     * self.albedo.clone())
                                                           // .into();
                                                           // irradiance over a hemisphere divided by outgoing incidence
                                                           // let isotrophic_surface_radiance =
                                                           // (1. / (2. * PI)) / outgoing_incident_factor * surface_irradiance.0;
                                                           // with that value we know exactly how bright our surface is in the viewing direction
                                                           // now we need the distance to observer and sensor area to compute watts delivered

        // let observer_radiantexitance: RadiantExitance =
        // ((1. / (r_o * r_o)) * isotrophic_surface_radiance).into();
        // observer_radiantexitance
        let lambertian: Radiance = ((1. / PI) * incoming_radiant_intensity.0).into();
        lambertian
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct Mirror {
//     pub metallic: f32,
//     pub roughness: f32,
//     pub albedo: Spectra,
// }

// impl Default for Mirror {
//     fn default() -> Self {
//         Mirror {
//             metallic: 0.0,
//             roughness: 1.0,
//             albedo: white_spectra(),
//         }
//     }
// }

// impl BRDF for Mirror {
//     fn rendering_equation(
//         &self,
//         _x: &Vector,     // position vector of equation
//         ω_i: &Vector,    // vector to light
//         ω_o: &Vector,    // light exit path
//         normal: &Vector, // surface normal
//         // incoming_radiant_intensity: Spectra, // the radiant flux of the lightsource encoded as a spectrum
//         incoming_radiant_intensity: RadiantIntensity,
//     ) -> RadiantExitance {
//         let r = ω_i.magnitude(); // dist to light source

//         let r_o = ω_o.magnitude(); // dist to observer
//         let incident_factor = cosθ(&ω_i, &normal); // per unit solid angle (area)
//         let outgoing_incident_factor = cosθ(&ω_o, &normal); // per unit solid angle (area
//                                                             // let incoming: Spectra = incoming_radiant_intensity.s;
//         let surface_irradiance: RadiantExitance = ((1. / (r * r))
//             * (incident_factor * incoming_radiant_intensity.0)
//             * self.albedo.clone())
//         .into();
//         // irradiance over a hemisphere divided by outgoing incidence
//         let isotrophic_surface_radiance =
//             (1. / (2. * PI)) / outgoing_incident_factor * surface_irradiance.0;
//         // with that value we know exactly how bright our surface is in the viewing direction
//         // now we need the distance to observer and sensor area to compute watts delivered

//         let observer_radiantexitance: RadiantExitance =
//             ((1. / (r_o * r_o)) * isotrophic_surface_radiance).into();
//         return observer_radiantexitance;
//     }
// }
