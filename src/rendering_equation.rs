use std::f32::consts::PI;

use crate::{
    lighting::{black_spectra, const_spectra, Spectra},
    primitives::Vector,
};

pub fn lamberts_law(ω_i: &Vector, normal: &Vector) -> f32 {
    let divisor: f32 = ω_i.magnitude() * normal.magnitude();

    return ω_i.dot(normal) / divisor;
}

/// BRDF
/// returns a radiance value
pub fn void(_x: &Vector, _ω_i: &Vector, _ω_o: &Vector, _spectra: &Spectra) -> Spectra {
    return black_spectra();
}

/// BRDF
/// returns a radiance value as a Spectra
/// correctness needs to be checked
pub fn diffuse_white(_x: &Vector, _ω_i: &Vector, _ω_o: &Vector, _spectra: &Spectra) -> Spectra {
    return const_spectra(PI / 2.);
}

/// defines how much radiance we're receiving in every wavelength with our parameters.
pub fn fuck_incoming_spectral_radiance(x: &Vector, ω_i: &Vector, spectra: &Spectra) -> Spectra {
    return black_spectra();
}

/// emission function candidate
pub fn no_emission(_x: &Vector, _ω_0: &Vector) -> Spectra {
    return black_spectra();
}

/// emission function candidate
pub fn bright_white_emission(_x: &Vector, _ω_0: &Vector) -> Spectra {
    return const_spectra(1.);
}

/// the rendering equation! Currently implemented as a single instance scene wise, that will change
/// note that I'm handling all wavelengths through a single function call
pub fn rendering_equation<
    BRDF: Fn(&Vector, &Vector, &Vector, &Spectra) -> Spectra,
    EMISSION: Fn(&Vector, &Vector) -> Spectra,
    INCOMING_RADIANCE: Fn(&Vector, &Vector, &Spectra) -> Spectra,
>(
    x: &Vector,       // position vector of equation
    ω_i: &Vector,     // vector to light
    ω_o: &Vector,     // light exit path
    normal: &Vector,  // surface normal
    spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
    brdf: BRDF,       // the BSDF function itself
    emission: EMISSION,
    incoming_radiance: INCOMING_RADIANCE,
) -> Spectra {
    let emitted_radiance = emission(x, ω_o);
    let scattering = brdf(&x, &ω_i, &ω_o, &spectra);
    let incoming_radiance = incoming_radiance(&x, &ω_i, &spectra);
    let lamberts_law = lamberts_law(&ω_i, &normal);
    let outgoing_radiance: Spectra =
        emitted_radiance + lamberts_law * scattering * incoming_radiance;

    return outgoing_radiance;
}

#[cfg(test)]
mod tests {
    use crate::{
        lighting::{black_body, black_spectra, Spectra},
        primitives::vector,
        rendering_equation::lamberts_law,
    };

    #[test]
    fn test_lamberts_law() {
        let a = vector(1., 0., 0.);
        let b = vector(0., 1., 0.);
        let c = vector(-1., 0., 0.);
        // perpindicular
        assert_eq!(lamberts_law(&a, &b), 0.);
        // negative
        assert_eq!(lamberts_law(&a, &c), -1.);
        // parallel
        assert_eq!(lamberts_law(&a, &a), 1.);
    }

    fn test_weakening_function() {}
}
