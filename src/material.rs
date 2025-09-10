use crate::{
    geometry::primitives::Vector,
    lighting::{white_spectra, RadiometricUnit, Spectra},
};
use std::{f32::consts::PI, fmt::Debug};

/// physical object in space with associated data
// I want shaders to simply be a trait
// any function that takes in all the necessary data and returns a light value is a shader
#[derive(Clone, Debug, PartialEq)]
pub struct PBR {
    pub metallic: f32,
    pub roughness: f32,
    pub albedo: Spectra,
}

/// Lambert's law of cosines
pub fn cosθ(ω: &Vector, normal: &Vector) -> f32 {
    let divisor: f32 = ω.magnitude() * normal.magnitude();

    return ω.dot(normal) / divisor;
}

pub trait BRDF: Debug + Sync + Send {
    fn rendering_equation(
        &self,
        x: &Vector,                          // position vector of equation
        ω_i: &Vector,                        // vector to light
        ω_o: &Vector,                        // light exit path
        normal: &Vector,                     // surface normal
        incoming_radiant_intensity: Spectra, // the radiant flux of the lightsource encoded as a spectrum
    ) -> Spectra;
}

impl BRDF for PBR {
    fn rendering_equation(
        &self,
        _x: &Vector,                         // position vector of equation
        ω_i: &Vector,                        // vector to light
        ω_o: &Vector,                        // light exit path
        normal: &Vector,                     // surface normal
        incoming_radiant_intensity: Spectra, // the radiant flux of the lightsource encoded as a spectrum
    ) -> Spectra {
        let r = ω_i.magnitude();

        let r_o = ω_o.magnitude();
        let incident_factor = cosθ(&ω_i, &normal); // cos(θ)
        let outgoing_incident_factor = cosθ(&ω_o, &normal); // cos(θ)
        let surface_irradiance = (1. / (r * r))
            * (incident_factor * incoming_radiant_intensity.clone())
            * self.albedo.clone();
        // irradiance over a hemisphere divided by outgoing incidence
        let isotrophic_surface_radiance =
            (1. / (2. * PI)) / outgoing_incident_factor * surface_irradiance;
        // with that value we know exactly how bright our surface is in the viewing direction
        // now we need the distance to observer and sensor area to compute watts delivered

        let observer_irradiance: Spectra = ((1. / (r_o * r_o)) * isotrophic_surface_radiance)
            .set_unit(RadiometricUnit::Irradiance);
        return observer_irradiance;
    }
}

impl PBR {
    pub const fn _new(metallic: f32, roughness: f32, albedo: Spectra) -> PBR {
        PBR {
            metallic,
            roughness,
            albedo,
        }
    }
}

impl Default for PBR {
    fn default() -> Self {
        PBR {
            metallic: 0.0,
            roughness: 1.0,
            albedo: white_spectra(RadiometricUnit::Flux),
        }
    }
}
