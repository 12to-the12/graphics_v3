#![allow(non_upper_case_globals)]
use std::f32::consts::{E, PI};
const _π: f32 = PI;
use crate::{
    color::luminous_efficiency::luminous_efficacy, geometry::{orientation::Orientation, primitives::{Vector, Vertex}},
};
extern crate ndarray;
use ndarray::prelude::*;

#[derive(Clone, Debug)]
pub enum RadiometricUnit {
    Flux,
    Intensity, // per sr
    Irradiance, // per m2
    Radiance // per sr perm2
}


#[derive(Clone)]
pub enum LightType {
    PointLight(PointLight),
}

/// this program uses Radiometry not Photometry because it's more physically accurate
/// Radiant Flux is total radiation, Watts
/// Radiant Intensity is radiation per solid angle, Watts/steridian
/// Radiant Exitance / Irradiance is radiation per area,  Watts/m2
/// Radiance is radiation per solid angle per area, Watts/sr/m2


/// the trait Light guarantees you can calculate the irradiance at a given point
pub trait Light {
    /// the apex is where the light source is observed from
    /// as it currently stands, all position vectors exist in worldspace
    fn radiant_intensity(&self, apex: Vector) -> Spectra;
}


/// Isotrophic light source with output measured in watts in each wavelength
/// an isotrophic light source has a radiant intensity of it's radiant flux / 4π
#[derive(Clone)]
pub struct PointLight {
    pub position: Vector, // as always, this is relative to it's parent
    pub _orientation: Orientation,
    pub radiant_flux: Spectra, // power in each wavelength
}
impl Light for PointLight {
    /// isotrophic point light sources don't care about the apex position
    fn radiant_intensity(&self, _apex: Vector) -> Spectra {
        return 1. / (4. * _π) * self.radiant_flux.clone();
    }
}


pub fn point_light(
    position: Vector,
    orientation: Orientation,
    radiant_flux: Spectra,
) -> PointLight {
    PointLight {
        position,
        _orientation: orientation,
        radiant_flux,
    }
}

// associates an unspecified value with 40 different wavelengths
// 380nm -> 780nm non inclusive at 10nm intervals

// this can be used to track radiance, radiant flux, "brightness", whatever
// this is a very memory intensive thing to track, so optimization will be key
#[derive(Clone, Debug)]
pub struct Spectra {
    pub spectra: Array1<f32>,
    pub unit: RadiometricUnit
}

impl std::ops::Mul<Spectra> for f32 {
    type Output = Spectra;
    fn mul(self, rhs: Spectra) -> Spectra {
        return Spectra {
            spectra: rhs.spectra * self,
            unit: rhs.unit
        };
    }
}

impl std::ops::Mul<Spectra> for Spectra {
    type Output = Spectra;
    fn mul(self, rhs: Spectra) -> Spectra {
        return Spectra {
            spectra: rhs.spectra * self.spectra,
            unit: rhs.unit
        };
    }
}

impl std::ops::Add<Spectra> for Spectra {
    type Output = Spectra;
    fn add(self, rhs: Spectra) -> Spectra {
        return Spectra {
            spectra: rhs.spectra + self.spectra,
            unit: rhs.unit
        };
    }
}

impl Spectra {
    pub fn from_λ(&self, λ: f32) -> f32 {
        let index: usize = (λ as usize - 380) / 10;
        return self.spectra[index];
    }
    pub fn set_from_λ(&mut self, λ: f32, value: f32) {
        let index: usize = (λ as usize - 380) / 10;
        self.spectra[index] = value;
    }
    /// the band of wavelengths that a single sample covers
    pub fn get_sample_width(&self) -> f32 {
        return 10.;
    }
    pub fn luminance(&self) -> f32 {
        return luminous_efficacy(self.clone());
    }
    pub fn total(&self) -> f32 {
        return self.spectra.sum();
    }
}
pub fn black_spectra(unit: RadiometricUnit) -> Spectra {
    Spectra {
        spectra: Array::zeros(40),
        unit: unit
    }
}

pub fn const_spectra(value: f32,unit: RadiometricUnit) -> Spectra {
    Spectra {
        spectra: Array::from_elem(40, value),
        unit: unit
    }
}

pub fn monochroma_spectra(λ: f32, value: f32,unit: RadiometricUnit) -> Spectra {
    let mut spectra = black_spectra(unit);

    spectra.set_from_λ(λ, value);
    spectra
}

// Boltzmann constant
// joules/kelvin
const k_B: f32 = 1.380649e-23;

// Planck constant
// joule second
const h: f32 = 6.62607015e-34;

// speed of light in a vacuum
const c: f32 = 299_792_458.;

// Wien's displacement constant
const _b: f32 = 2.897771955e-3;

/// get the peak wavelength of a blackbody in nanometers
///
pub fn _peak_blackbody(temp: f32) -> f32 {
    let peak_in_meters = _b / temp;
    let peak_in_nm = peak_in_meters * 1e9;
    peak_in_nm
}

pub fn norm_black_body(temp: f32) -> Spectra {
    // let λ = peak_blackbody(temp);
    // let value_at_peak = plancks_law(&λ, &temp);
    let total_power = black_body(temp,RadiometricUnit::Flux).spectra.sum();
    let factor = 1. / total_power;
    let normalized: Spectra = factor * black_body(temp,RadiometricUnit::Flux);
    normalized
}

// blackbody radiation spectra at a given temperature in Kelvin
// the spectra is in terms of watts/meter**2/steradian, radiance
pub fn black_body(temp: f32,unit: RadiometricUnit) -> Spectra {
    let mut spectra = Array::zeros(40);

    for i in 0..40 {
        let λ = i as f32 * 10. + 380.;
        // println!("{:?}", λ);
        spectra[i] = plancks_law(&λ, &temp)
    }
    return Spectra { spectra: spectra ,unit};
}

// takes wavelength in nanometers
// returns radiance in watts/meter**2/steradian
pub fn plancks_law(λ: &f32, temp: &f32) -> f32 {
    // convert nanometers to meters
    let λ: f32 = λ * 1e-9;
    // Planck's law over a million to get the units we want
    return (2. * h * c.powi(2)) / (λ.powi(5))
        * (1. / (E.powf((h * c) / (λ * k_B * temp)) - 1.))
        * 1e-6;
}

#[cfg(test)]
mod tests {
    use crate::lighting::{black_body, black_spectra, RadiometricUnit, Spectra};

    use super::plancks_law;

    #[test]
    fn test_index_by_wavelength() {
        let mut spectra = black_spectra(RadiometricUnit::Flux);
        spectra.spectra[39] = 1.;
        spectra.spectra[26] = 2.7;

        assert_eq!(spectra.spectra[0], 0.);
        assert_eq!(spectra.from_λ(770.), 1.);
        assert_eq!(spectra.spectra[25], 0.);
        assert_eq!(spectra.spectra[26], 2.7);
        assert_eq!(spectra.from_λ(640.), 2.7);
    }

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn test_planks_law() {
        // peak emission for a campfire
        let spectral_radiance: f32 = plancks_law(&2000., &1500.);
        // spectral_radiance = (spectral_radiance as f64* 1e39_f64) as f32;
        assert_eq!(spectral_radiance, 31012.611); //

        let spectral_radiance: f32 = plancks_law(&780., &2700.);
        // spectral_radiance = (spectral_radiance as f64* 1e39_f64) as f32;
        assert_eq!(spectral_radiance, 445564.66); //
    }
    #[test]
    fn test_blackbody() {
        // incandescent lightbulb
        let radiance_spectra: Spectra = black_body(2700.,RadiometricUnit::Flux);
        // 780nm
        let peak = radiance_spectra.from_λ(770.);
        // spectral_radiance = (spectral_radiance as f64* 1e39_f64) as f32;
        assert_eq!(peak, 434_868.25); //
    }
}
