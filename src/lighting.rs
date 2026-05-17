#![allow(non_upper_case_globals)]
use std::{
    f32::consts::{E, PI},
    fmt::Debug,
};
const _π: f32 = PI;
use crate::{
    color::luminous_efficiency::luminous_efficacy,
    geometry::{
        orientation::{Orientation, UP},
        primitives::{Vector, ORIGIN},
    },
    object::Entity,
    scene::scene::EntityKey,
};
extern crate ndarray;
use ndarray::prelude::*;

pub trait _Spectral {}
/// this program uses Radiometry not Photometry because it's more physically accurate
/// Radiant Flux is total radiation, Watts
/// Radiant Intensity is radiation per solid angle, Watts/steridian
/// Radiant Exitance / Irradiance is radiation per area,  Watts/m2
/// Radiance is radiation per solid angle per area, Watts/sr/m2

/// the trait Light guarantees you can calculate the irradiance at a given point
/// APPARENTLY ADDING Where: Sized CAN IN FACT CAUSE ERRORS. NOTED.
pub trait Light: Debug + Sync + Send + Entity {
    /// the apex is where the light source is observed from
    /// as it currently stands, all position vectors exist in worldspace
    fn radiant_intensity(&self, apex: Vector) -> RadiantIntensity;
}

/// Isotrophic light source with output measured in watts in each wavelength
/// an isotrophic light source has a radiant intensity of it's radiant flux / 4π
#[derive(Clone, Debug)]
pub struct PointLight {
    pub position: Vector, // as always, this is relative to it's parent
    pub orientation: Orientation,
    pub radiant_flux: RadiantFlux, // power in each wavelength
    pub children: Vec<EntityKey>,
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            position: ORIGIN,
            orientation: UP,
            children: Vec::new(),

            radiant_flux: incandescent_spectra(2000., 1000.),
        }
    }
}
impl PointLight {
    pub fn new(
        position: Vector,
        orientation: Orientation,
        radiant_flux: RadiantFlux,
    ) -> PointLight {
        PointLight {
            position,
            orientation,
            radiant_flux,
            ..PointLight::default()
        }
    }
}
impl Light for PointLight {
    /// isotrophic point light sources don't care about the apex position
    fn radiant_intensity(&self, _apex: Vector) -> RadiantIntensity
    where
        Self: Sized,
    {
        (1. / (4. * _π) * self.radiant_flux.clone().0).into()
    }
}

impl Entity for PointLight {
    fn get_position(&self) -> Vector {
        self.position
    }
    fn get_orientation(&self) -> Orientation {
        self.orientation
    }
    fn get_scale(&self) -> Vector {
        Vector::ones()
    }
    fn get_children(&mut self) -> &mut Vec<EntityKey> {
        &mut self.children
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadiantFlux(pub(crate) Spectra);

impl From<Spectra> for RadiantFlux {
    fn from(value: Spectra) -> RadiantFlux {
        RadiantFlux(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadiantIntensity(pub(crate) Spectra);
impl From<Spectra> for RadiantIntensity {
    fn from(value: Spectra) -> RadiantIntensity {
        RadiantIntensity(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadiantExitance(pub(crate) Spectra);
impl From<Spectra> for RadiantExitance {
    fn from(value: Spectra) -> RadiantExitance {
        RadiantExitance(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Radiance(pub(crate) Spectra);
impl From<Spectra> for Radiance {
    fn from(value: Spectra) -> Radiance {
        Radiance(value)
    }
}

// associates an unspecified value with 40 different wavelengths
// 380nm -> 780nm non inclusive at 10nm intervals

// this can be used to track radiance, radiant flux, "brightness", whatever
// this is a very memory intensive thing to track, so optimization will be key
#[derive(Clone, Debug, PartialEq)]
pub struct Spectra {
    pub spectra: Array1<f32>,
}

impl std::ops::Div<f32> for Spectra {
    type Output = Spectra;
    fn div(self, rhs: f32) -> Spectra {
        Spectra {
            spectra: self.spectra / rhs,
        }
    }
}

impl std::ops::Mul<Spectra> for f32 {
    type Output = Spectra;
    fn mul(self, rhs: Spectra) -> Spectra {
        Spectra {
            spectra: rhs.spectra * self,
        }
    }
}

impl std::ops::Mul<Spectra> for Spectra {
    type Output = Spectra;
    fn mul(self, rhs: Spectra) -> Spectra {
        Spectra {
            spectra: rhs.spectra * self.spectra,
        }
    }
}

impl std::ops::Add<Spectra> for Spectra {
    type Output = Spectra;
    fn add(self, rhs: Spectra) -> Spectra {
        Spectra {
            spectra: rhs.spectra + self.spectra,
        }
    }
}

impl Spectra {
    pub fn from_λ(&self, λ: f32) -> f32 {
        let index: usize = (λ as usize - 380) / 10;
        self.spectra[index]
    }
    pub fn set_from_λ(&mut self, λ: f32, value: f32) {
        let index: usize = (λ as usize - 380) / 10;
        self.spectra[index] = value;
    }
    /// the band of wavelengths that a single sample covers
    pub fn get_sample_width(&self) -> f32 {
        10.
    }
    pub fn luminance(&self) -> f32 {
        luminous_efficacy(self.clone())
    }
    pub fn integrated(&self) -> f32 {
        self.spectra.sum()
    }
}

pub fn black_spectra() -> Spectra {
    // const_spectra(1e-12)
    Spectra {
        spectra: Array::zeros(40),
    }
}

pub fn void_spectra() -> Spectra {
    // monochroma_spectra(550., 1e-8)
    black_spectra()
    // monochroma_spectra(550., 1e-20)
    // white_spectra()
}

pub fn white_spectra() -> Spectra {
    Spectra {
        spectra: Array::ones(40),
    }
}

pub fn green_spectra() -> Spectra {
    let mut spectra = const_spectra(0.1);
    for λ in 400..450 {
        spectra.set_from_λ(λ as f32, 0.3);
    }
    spectra
}
pub fn const_spectra(value: f32) -> Spectra {
    Spectra {
        spectra: Array::from_elem(40, value),
    }
}

pub fn monochroma_spectra(λ: f32, value: f32) -> Spectra {
    let mut spectra = black_spectra();

    spectra.set_from_λ(λ, value);
    spectra
}

// Boltzmann constant
// joules/kelvin
const k_B: f32 = 1.380649e-23;

// Planck constant
// joule second
const h: f32 = 6.626_07e-34;

// speed of light in a vacuum
const c: f32 = 299_792_458.;

// Wien's displacement constant
const _b: f32 = 2.897_772e-3;

/// get the peak wavelength of a blackbody in nanometers
pub fn _peak_blackbody(temp: f32) -> f32 {
    let peak_in_meters = _b / temp;

    peak_in_meters * 1e9
}

pub fn norm_black_body(temp: f32) -> Spectra {
    // let λ = peak_blackbody(temp);
    // let value_at_peak = plancks_law(&λ, &temp);
    let total_power = black_body(temp).0.integrated();
    let factor = 1. / total_power;
    let normalized: Spectra = factor * black_body(temp).0;
    normalized
}

// blackbody radiation spectra at a given temperature in Kelvin
// the spectra is in terms of watts/meter**2/steradian, radiance
pub fn black_body(temp: f32) -> Radiance {
    let mut spectra = Array::zeros(40);

    for i in 0..40 {
        let λ = i as f32 * 10. + 380.;
        // println!("{:?}", λ);
        spectra[i] = plancks_law(&λ, &temp);
    }
    Spectra { spectra }.into()
}

/// 2200 Kelvin blackbody emitting 60W of radiation
/// `RadiantFlux` from Radiance is obtained by ignoring the area
pub fn incandescent_spectra(temp: f32, power: f32) -> RadiantFlux {
    let spectra = power * norm_black_body(temp) / (4. * PI);
    spectra.into()
}

// takes wavelength in nanometers
// returns radiance in watts/meter**2/steradian
pub fn plancks_law(λ: &f32, temp: &f32) -> f32 {
    // convert nanometers to meters
    let λ: f32 = λ * 1e-9;
    // Planck's law over a million to get the units we want
    (2. * h * c.powi(2)) / (λ.powi(5)) * (1. / (E.powf((h * c) / (λ * k_B * temp)) - 1.)) * 1e-6
}

#[cfg(test)]
mod tests {
    use crate::lighting::{black_body, black_spectra, Radiance, Spectra};

    use super::plancks_law;

    #[test]
    fn test_index_by_wavelength() {
        let mut spectra = black_spectra();
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
        assert_eq!(spectral_radiance, 31012.611); //

        let spectral_radiance: f32 = plancks_law(&780., &2700.);
        assert_eq!(spectral_radiance, 445564.66); //
    }
    #[test]
    fn test_blackbody() {
        // incandescent lightbulb
        let radiance_spectra: Radiance = black_body(2700.);
        // 780nm
        let peak = radiance_spectra.0.from_λ(770.);
        assert_eq!(peak, 434_868.25); //
    }
}
