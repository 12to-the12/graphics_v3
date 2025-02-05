use std::f32::consts::PI;

use crate::{
    geometry::primitives::Vector,
    lighting::{black_spectra, const_spectra, Spectra},
};

pub fn lamberts_law(ω_i: &Vector, normal: &Vector) -> f32 {
    let divisor: f32 = ω_i.magnitude() * normal.magnitude();

    return ω_i.dot(normal) / divisor;
}

/// BRDF
/// returns a radiance value
pub fn _void(_x: &Vector, _ω_i: &Vector, _ω_o: &Vector, _spectra: &Spectra) -> Spectra {
    return black_spectra();
}

/// BRDF
/// returns a radiance value as a Spectra
/// correctness needs to be checked
/// *not* multipled by the provided Spectra, that's there for when the function differs over the spectrum, this function is isotrophic
pub fn diffuse_white(_x: &Vector, _ω_i: &Vector, _ω_o: &Vector, _spectra: &Spectra) -> Spectra {
    return const_spectra(PI / 2.);
}

/// as if bro. Lights don't exist.
pub fn _fuck_incoming_spectral_radiance(_x: &Vector, _: &Vector, _spectra: &Spectra) -> Spectra {
    return black_spectra();
}

/// defines how much radiance we're receiving in every wavelength with our parameters.
/// because the current lights are all isotrophic, the function is trivium
pub fn normal_incoming_spectral_radiance(
    _x: &Vector, _ω_i: &Vector, spectra: &Spectra
) -> Spectra {
    return spectra.clone();
}

/// emission function candidate
pub fn no_emission(_x: &Vector, _ω_0: &Vector) -> Spectra {
    return black_spectra();
}

/// emission function candidate
pub fn _bright_white_emission(_x: &Vector, _ω_0: &Vector) -> Spectra {
    return const_spectra(1.);
}

/// the rendering equation! Currently implemented as a single instance scene wise, that will change
/// note that I'm handling all wavelengths through a single function call
// #[allow(non_camel_case_types)]
pub fn rendering_equation<
    BRDF: Fn(&Vector, &Vector, &Vector, &Spectra) -> Spectra,
    EMISSION: Fn(&Vector, &Vector) -> Spectra,
    IncomingRadiance: Fn(&Vector, &Vector, &Spectra) -> Spectra,
>(
    x: &Vector,       // position vector of equation
    ω_i: &Vector,     // vector to light
    ω_o: &Vector,     // light exit path
    normal: &Vector,  // surface normal
    spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
    brdf: BRDF,       // the BSDF function itself
    emission: EMISSION,
    incoming_radiance: IncomingRadiance,
) -> Spectra {
    let emitted_radiance = emission(x, ω_o);
    let scattering = brdf(&x, &ω_i, &ω_o, &spectra);
    let incoming_radiance = incoming_radiance(&x, &ω_i, &spectra);
    let lamberts_law = lamberts_law(&ω_i, &normal);
    // println!("{:?}",incoming_radiance);
    let outgoing_radiance: Spectra =
        emitted_radiance + lamberts_law * scattering * incoming_radiance;
    // println!("{:?}",outgoing_radiance);
    return outgoing_radiance;
}

pub fn white_matte_equation(
    x: &Vector,       // position vector of equation
    ω_i: &Vector,     // vector to light
    ω_o: &Vector,     // light exit path
    normal: &Vector,  // surface normal
    spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
) -> Spectra {
    return rendering_equation(
        x,
        ω_i,
        ω_o,
        normal,
        spectra,
        diffuse_white,
        no_emission,
        normal_incoming_spectral_radiance,
    );
}

pub fn _white_emission_equation(
    x: &Vector,       // position vector of equation
    ω_i: &Vector,     // vector to light
    ω_o: &Vector,     // light exit path
    normal: &Vector,  // surface normal
    spectra: Spectra, // the radiant flux of the lightsource encoded as a spectrum
) -> Spectra {
    return rendering_equation(
        x,
        ω_i,
        ω_o,
        normal,
        spectra,
        _void,
        _bright_white_emission,
        _fuck_incoming_spectral_radiance,
    );
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::{
        geometry::primitives::{vector, _ORIGIN},
        lighting::{const_spectra, Spectra},
        ray_tracing::rendering_equation::lamberts_law,
    };

    use super::diffuse_white;

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

    #[test]
    fn test_diffuse_white() {
        let spectra: Spectra = diffuse_white(&_ORIGIN, &_ORIGIN, &_ORIGIN, &const_spectra(1.));
        assert_eq!(spectra.from_λ(550.), PI / 2.);
    }
}
