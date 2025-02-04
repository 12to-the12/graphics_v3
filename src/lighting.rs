use std::f32::consts::{E, PI};
const π: f32 = PI;
use crate::{orientation::Orientation, primitives::Vertex};
extern crate ndarray;
use ndarray::prelude::*;

#[derive(Clone)]
pub struct PointLight {
    pub position: Vertex,
    pub orientation: Orientation,
    pub radiant_flux: Spectra, // power
}
impl PointLight {
    pub fn radiant_intensity(&self) -> Spectra {
        return 1. / (4. * π) * self.radiant_flux.clone();
    }
}

pub fn point_light(
    position: Vertex,
    orientation: Orientation,
    radiant_flux: Spectra,
) -> PointLight {
    PointLight {
        position,
        orientation,
        radiant_flux,
    }
}


// associates an unspecified value with 40 different wavelengths
// 380nm -> 780nm non inclusive at 10nm intervals

// this can be used to track radiance, radiant flux, "brightness", whatever
// this is a very memory intensive thing to track, so optimization will be key
#[derive(Clone,Debug)]
pub struct Spectra {
    pub spectra: Array1<f32>,
}

impl std::ops::Mul<Spectra> for f32 {
    type Output = Spectra;
    fn mul(self, rhs: Spectra) -> Spectra {
        return Spectra {
            spectra: rhs.spectra * self,
        };
    }
}

impl std::ops::Mul<Spectra> for Spectra {
    type Output = Spectra;
    fn mul(self, rhs: Spectra) -> Spectra {
        return Spectra {
            spectra: rhs.spectra * self.spectra,
        };
    }
}

impl std::ops::Add<Spectra> for Spectra {
    type Output = Spectra;
    fn add(self, rhs: Spectra) -> Spectra {
        return Spectra {
            spectra: rhs.spectra + self.spectra,
        };
    }
}

impl Spectra {
    pub fn from_λ(&self, λ: f32) -> f32 {
        let index: usize = (λ as usize - 380) / 10;
        return self.spectra[index];
    }
    pub fn set_from_λ(&mut self, λ: f32,value:f32) {
        let index: usize = (λ as usize - 380) / 10;
        self.spectra[index]=value;
    }
    /// the band of wavelengths that a single sample covers
    pub fn get_sample_width(&self)->f32{
        return 10.
    }
}
pub fn black_spectra() -> Spectra {
    Spectra {
        spectra: Array::zeros(40),
    }
}

pub fn const_spectra(value: f32) -> Spectra {
    Spectra {
        spectra: Array::from_elem(40,value),
    }
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
const b: f32 = 2.897771955e-3;


/// get the peak wavelength of a blackbody in nanometers
/// 
pub fn peak_blackbody(temp:f32)->f32{
    let peak_in_meters = b/temp;
    let peak_in_nm = peak_in_meters*1e9;
    peak_in_nm
}


pub fn norm_black_body(temp: f32)->Spectra{
    
    let λ = peak_blackbody(temp);
    let value_at_peak = plancks_law(&λ, &temp);
    let factor = 1./value_at_peak;
    let normalized: Spectra =  factor*black_body(temp);
    normalized
}

// blackbody radiation spectra at a given temperature in Kelvin
// the spectra is in terms of watts/meter**2/steradian, radiance
pub fn black_body(temp: f32) -> Spectra {
    let mut spectra = Array::zeros(40);

    for i in 0..40 {
        let λ = i as f32 * 10. + 380.;
        // println!("{:?}", λ);
        spectra[i] = plancks_law(&λ, &temp)
    }
    return Spectra { spectra: spectra };
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
    use crate::lighting::{black_body, black_spectra, Spectra};

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
        // spectral_radiance = (spectral_radiance as f64* 1e39_f64) as f32;
        assert_eq!(spectral_radiance, 31012.611); //

        let spectral_radiance: f32 = plancks_law(&780., &2700.);
        // spectral_radiance = (spectral_radiance as f64* 1e39_f64) as f32;
        assert_eq!(spectral_radiance, 445564.66); //
    }
    #[test]
    fn test_blackbody() {
        // incandescent lightbulb
        let radiance_spectra: Spectra = black_body(2700.);
        // 780nm
        let peak = radiance_spectra.from_λ(770.);
        // spectral_radiance = (spectral_radiance as f64* 1e39_f64) as f32;
        assert_eq!(peak, 434_868.25); //
    }
}
