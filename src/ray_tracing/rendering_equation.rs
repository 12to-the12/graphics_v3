use std::fmt::Debug;

use crate::{geometry::primitives::Vector, lighting::Spectra};

// #[derive(Clone, Debug, PartialEq, Copy)]
// pub struct Material {}

pub fn lamberts_law(ω: &Vector, normal: &Vector) -> f32 {
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
    // fn evaluate_BRDF(&self, x: &Vector, ω_i: &Vector, ω_0: &Vector, spectra: &Spectra) -> Spectra;
}
// /// BRDF
// /// returns a radiance value
// #[derive(Clone, Debug, PartialEq, Copy)]
// pub(crate) struct _Void {}

// impl BRDF for _Void {
//     fn evaluate_BRDF(&self, _x: &Vector, _ω_i: &Vector, _ω_o: &Vector, _spectra: &Spectra) -> Spectra {
//         return black_spectra(RadiometricUnit::Radiance);
//     }
// }

// /// BRDF
// /// returns a radiance value as a Spectra
// /// correctness needs to be checked
// /// *not* multiplied by the provided Spectra, that's there for when the function differs over the spectrum, this function is isotrophic
// #[derive(Clone, Debug, PartialEq, Copy)]
// struct DiffuseWhite {}
// impl BRDF for DiffuseWhite {
//     fn evaluate_BRDF(&self, _x: &Vector, _ω_i: &Vector, _ω_o: &Vector, _spectra: &Spectra) -> Spectra {
//         return const_spectra(PI / 2.,RadiometricUnit::Radiance);
//     }
// }

// /// as if bro. Lights don't exist.
// /// this is not a BRDF
// pub fn _no_incoming_spectral_radiance(_x: &Vector, _: &Vector, _spectra: &Spectra) -> Spectra {
//     return black_spectra(RadiometricUnit::Radiance);
// }

// /// as if bro. Lights don't exist.
// /// this is not a BRDF
// pub fn normal_incoming_spectral_radiance(_x: &Vector, _: &Vector, spectra: &Spectra) -> Spectra {
//     return spectra.clone();
// }

// #[derive(Clone, Debug, PartialEq, Copy)]
// struct _IsotrophicMaterial {}
// /// defines how much radiance we're receiving in every wavelength with our parameters.
// /// because the current lights are all isotrophic, the function is trivium
// impl BRDF for _IsotrophicMaterial {
//     fn evaluate_BRDF(&self, _x: &Vector, _ω_i: &Vector, _ω_o: &Vector, spectra: &Spectra) -> Spectra {
//         return spectra.clone();
//     }
// }

// pub trait EMMISIVITY {
//     fn evaluate_BRDF(&self, x: &Vector, ω_o: &Vector) -> Spectra;
// }

// pub struct NoEmission {}

// impl EMMISIVITY for NoEmission {
//     fn evaluate_BRDF(&self, _x: &Vector, _ω_o: &Vector) -> Spectra {
//         return black_spectra(RadiometricUnit::Radiance);
//     }
// }

// struct _BrightWhiteEmission {}

// impl EMMISIVITY for _BrightWhiteEmission {
//     fn evaluate_BRDF(&self, _x: &Vector, _ω_o: &Vector) -> Spectra {
//         return const_spectra(1.,RadiometricUnit::Radiance);
//     }
// }

// / the rendering equation! Currently implemented as a single instance scene wise, that will change
// / note that I'm handling all wavelengths through a single function call
// #[allow(non_camel_case_types)]
// pub fn rendering_equation<IncomingRadiance: Fn(&Vector, &Vector, &Spectra) -> Spectra>(
//     x: &Vector,       // position vector of equation
//     ω_i: &Vector,     // vector to light
//     ω_o: &Vector,     // light exit path
//     normal: &Vector,  // surface normal
//     spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
//     brdf: impl BRDF,  // the BRDF function itself
//     emission: impl EMMISIVITY,
//     incoming_radiance: IncomingRadiance,
// ) -> Spectra {
//     let emitted_radiance = emission.evaluate_BRDF(x, ω_o);
//     let scattering = brdf.evaluate_BRDF(&x, &ω_i, &ω_o, &spectra);
//     let incoming_radiance = incoming_radiance(&x, &ω_i, &spectra);
//     let lamberts_law = lamberts_law(&ω_i, &normal);
//     let outgoing_radiance: Spectra =
//         emitted_radiance + lamberts_law * scattering * incoming_radiance;
//     return outgoing_radiance;
// }

// pub fn white_matte_equation(
//     x: &Vector,       // position vector of equation
//     ω_i: &Vector,     // vector to light
//     ω_o: &Vector,     // light exit path
//     normal: &Vector,  // surface normal
//     spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
// ) -> Spectra {
//     return rendering_equation(
//         x,
//         ω_i,
//         ω_o,
//         normal,
//         spectra,
//         DiffuseWhite {},
//         NoEmission {},
//         normal_incoming_spectral_radiance,
//     );
// }

// pub fn _white_emission_equation(
//     x: &Vector,       // position vector of equation
//     ω_i: &Vector,     // vector to light
//     ω_o: &Vector,     // light exit path
//     normal: &Vector,  // surface normal
//     spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
// ) -> Spectra {
//     return rendering_equation(
//         x,
//         ω_i,
//         ω_o,
//         normal,
//         spectra,
//         _Void {},
//         _BrightWhiteEmission {},
//         normal_incoming_spectral_radiance,
//     );
// }

// #[cfg(test)]
// mod tests {
//     use std::f32::consts::PI;

//     use crate::{
//         geometry::primitives::{vector, ORIGIN},
//         lighting::{const_spectra, Spectra},
//         ray_tracing::rendering_equation::lamberts_law,
//     };

//     use super::DiffuseWhite;
//     use super::BRDF;

//     #[test]
//     fn test_lamberts_law() {
//         let a = vector(1., 0., 0.);
//         let b = vector(0., 1., 0.);
//         let c = vector(-1., 0., 0.);
//         // perpindicular
//         assert_eq!(lamberts_law(&a, &b), 0.);
//         // negative
//         assert_eq!(lamberts_law(&a, &c), -1.);
//         // parallel
//         assert_eq!(lamberts_law(&a, &a), 1.);
//     }

//     #[test]
//     fn test_diffuse_white() {
//         let brdf = DiffuseWhite {};
//         let spectra: Spectra = brdf.evaluate_BRDF(&ORIGIN, &ORIGIN, &ORIGIN, &const_spectra(1.,crate::lighting::RadiometricUnit::Radiance));
//         assert_eq!(spectra.from_λ(550.), PI / 2.);
//     }
// }
