// sourced from
// http://hyperphysics.phy-astr.gsu.edu/hbase/vision/efficacy.html

use ndarray::Array;

use crate::lighting::Spectra;

// photopic conversion (lm/w)
// 380-770
const PHOTOPIC_CONVERSION: [f32; 40] = [
    0.027, 0.082, 0.270, 0.826, 2.732, 7.923, 15.709, 25.954, 40.980, 62.139, 94.951, 142.078,
    220.609, 343.549, 484.930, 588.746, 651.582, 679.551, 679.585, 650.216, 594.210, 517.031,
    430.973, 343.549, 260.223, 180.995, 119.525, 73.081, 41.663, 21.856, 11.611, 5.607, 2.802,
    1.428, 0.715, 0.355, 0.170, 0.082, 0.041, 0.020,
];

// luminous efficiency
// equal to the photopic conversion normalized to 1 for 555nm (divided by 683)
const LUMINOUS_EFFICACY: [f32; 40] = [
    0.000039, 0.000120, 0.000396, 0.001210, 0.004000, 0.011600, 0.023000, 0.038000, 0.060000,
    0.090980, 0.139020, 0.208020, 0.323000, 0.503000, 0.710000, 0.862000, 0.954000, 0.994950,
    0.995000, 0.952000, 0.870000, 0.757000, 0.631000, 0.503000, 0.381000, 0.265000, 0.175000,
    0.107000, 0.061000, 0.032000, 0.017000, 0.008210, 0.004102, 0.002091, 0.001047, 0.000520,
    0.000249, 0.000120, 0.000060, 0.000030,
];

// neither of these are valid, they need to average or something, not sum
// lm/w
pub fn photopic_conversion(spectra: Spectra) -> f32 {
    // println!("{:?}", &spectra.spectra);

    let watts = Array::from_vec(PHOTOPIC_CONVERSION.to_vec());
    let lumens_spectra = watts * spectra.spectra; // watts* (lumens/watt)
    let lumens = Array::sum(&lumens_spectra);
    return lumens;
}


// neither of these are valid, they need to average or something, not sum
pub fn luminous_efficacy(spectra: Spectra) -> f32 {
    // println!("{:?}", &spectra.spectra);

    let efficacy = Array::from_vec(LUMINOUS_EFFICACY.to_vec());
    let lumens_spectra = efficacy * spectra.spectra;
    let result = Array::sum(&lumens_spectra);
    return result;
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::{
        lighting::{black_body, black_spectra, Spectra},
        color::luminous_efficiency::{luminous_efficacy, photopic_conversion},
    };

    // use super::PHOTOPIC_CONVERSION;

    #[test]
    fn test_luminous_efficacy_of_darkness() {
        let spectra: Spectra = black_spectra();
        let efficacy: f32 = photopic_conversion(spectra);
        assert_eq!(efficacy, 0.);
    }
    #[test]
    fn test_luminous_efficacy_of_a_single_wavelength() {
        let mut radiant_flux: Spectra = black_spectra();
        radiant_flux.set_from_λ(550., 1.);
        let lumens: f32 = photopic_conversion(radiant_flux);
        assert_eq!(lumens, 679.551);

        let mut radiant_flux: Spectra = black_spectra();
        radiant_flux.set_from_λ(560., 1.);
        let lumens: f32 = photopic_conversion(radiant_flux);
        assert_eq!(lumens, 679.585);

        let mut radiant_flux: Spectra = black_spectra();
        radiant_flux.set_from_λ(560., 1.);
        radiant_flux.set_from_λ(550., 1.);
        let lumens: f32 = photopic_conversion(radiant_flux);
        assert_eq!(lumens, 1359.136);


    }

    // #[test]
    // fn test_luminous_efficacy_of_blackbody() {
    //     // in terms of radiance (watts/meter^2/steradian), we want radiant flux (watts)
    //     let radiance: Spectra = black_body(2700.);
    //     let radiant_exitance: Spectra = Spectra {
    //         spectra: radiance.spectra.clone() * 4. * PI,
    //     };
    //     let radius: f32 = 1.; // meters
    //     let area = 4.*PI*radius.powi(2);
    //     let flux: Spectra = Spectra {
    //         spectra: radiant_exitance.spectra.clone() * area,
    //     };
    //     let efficacy: f32 = luminous_efficacy(flux);
    //     assert_eq!(efficacy, 12.6);
    // }
}
