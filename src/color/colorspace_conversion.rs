#![allow(nonstandard_style)]

use image::Rgb;
use itertools::Itertools;
use ndarray::arr2;

use crate::{
    color::cie_color_matching_functions::{
        integrated_x_response, integrated_y_response, integrated_z_response,
    },
    lighting::{norm_black_body, Spectra},
};

pub fn _black_body_xyY(temp: f32) -> (f32, f32, f32) {
    let spectra = norm_black_body(temp);
    let XYZ = spectra_to_CIEXYZ(&spectra);

    CIEXYZ_to_xyY(XYZ)
}

pub fn _black_body_sRGB(temp: f32) -> (f32, f32, f32) {
    _spectra_to_sRGB(&norm_black_body(temp))
}

pub fn _spectra_to_sRGB(spectra: &Spectra) -> (f32, f32, f32) {
    let XYZ = spectra_to_CIEXYZ(spectra);
    let xyY = CIEXYZ_to_xyY(XYZ);

    xyY_to_sRGB(xyY)
}

pub fn spectra_to_display(spectra: &Spectra) -> Rgb<u8> {
    let XYZ = spectra_to_CIEXYZ(spectra);
    let xyY = CIEXYZ_to_xyY(XYZ);
    let sRGB = xyY_to_sRGB(xyY);

    sRGB_to_display(sRGB)
}

pub fn spectra_to_CIEXYZ(spectra: &Spectra) -> (f32, f32, f32) {
    let X = integrated_x_response(spectra);
    let Y = integrated_y_response(spectra);
    let Z = integrated_z_response(spectra);
    if X + Y + Z == 0. {
        (1e-10, 1e-10, 1e-10)
    } else {
        (X, Y, Z)
    }
}

/// CANNOT HANDLE ZEROES
pub fn CIEXYZ_to_xyY(XYZ: (f32, f32, f32)) -> (f32, f32, f32) {
    let X = XYZ.0;
    let Y = XYZ.1;
    let Z = XYZ.2;
    let x = X / (X + Y + Z);
    let y = Y / (X + Y + Z);
    (x, y, Y)
}

pub fn xyY_to_CIEXYZ(xyY: (f32, f32, f32)) -> (f32, f32, f32) {
    let (x, y, Y) = xyY;
    let sum = Y / y;
    //X=x*Y/y
    let X = x * (Y / y);
    let Z = sum - X - Y;
    (X, Y, Z)
}

/// I think? it might supposed to be CIEXYZ?
/// NOTE: this is not u8 encoded! it's possibly negative!
pub fn xyY_to_sRGB(xyY: (f32, f32, f32)) -> (f32, f32, f32) {
    // yeah, yeah, bad vector implementation I know
    let x: f32 = xyY.0;
    let y: f32 = xyY.1;
    let Y: f32 = xyY.2;
    let z = 1. - x - y;

    let mut sR_linear = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let mut sG_linear = -0.969266 * x + 1.8760108 * y + 0.0415560 * z;
    let mut sB_linear = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

    // I pray this is the correct way to do things
    sR_linear *= Y;
    sG_linear *= Y;
    sB_linear *= Y;

    // NOTE: THIS DEVIATES FROM AN ACCURATE COLOR MODEL
    // HAS TO BE GREATER THAN ONE FOR BLACKS TO WORK
    // I think this will desaturate?
    // let factor = 1.2;
    let ɛ: f32 = 1e-12;
    let factor = 1. + ɛ;
    // let factor = 1.04;
    let sR_linear = (sR_linear + ɛ) / factor;
    let sG_linear = (sG_linear + ɛ) / factor;
    let sB_linear = (sB_linear + ɛ) / factor;

    let sR = sRGB_apply_gamma(sR_linear);
    let sG = sRGB_apply_gamma(sG_linear);
    let sB = sRGB_apply_gamma(sB_linear);
    (sR, sG, sB)
}

pub fn sRGB_to_xyY(sRGB: (f32, f32, f32)) -> (f32, f32, f32) {
    let (sR, sG, sB) = sRGB;
    let sR_linear = sRGB_remove_gamma(sR);
    let sG_linear = sRGB_remove_gamma(sG);
    let sB_linear = sRGB_remove_gamma(sB);

    let ɛ: f32 = 1e-12;
    let factor = 1. + ɛ;
    let mut sR_linear = (sR_linear * factor) - ɛ;
    let mut sG_linear = (sG_linear * factor) - ɛ;
    let mut sB_linear = (sB_linear * factor) - ɛ;

    let Y = 1.0;
    sR_linear /= Y;
    sG_linear /= Y;
    sB_linear /= Y;

    let inversion_matrix = arr2(&[
        [0.412_456_42, 0.357_576_07, 0.180_437_48],
        [0.212_672_84, 0.715_152_14, 0.072_174_996],
        [0.019_333_905, 0.119_192_03, 0.950_304_1],
    ]);
    // let RGB_linear = arr2(&[
    //     [sR_linear],
    //     [sG_linear],
    //     [sB_linear],
    // ]);
    let RGB_linear = ndarray::Array::from_vec(vec![sR_linear, sG_linear, sB_linear]);

    println!("inversion_matrix: {inversion_matrix:?}");
    println!("RGB_linear: {RGB_linear:?}");
    let xyz = inversion_matrix.dot(&RGB_linear);
    println!("xyz: {xyz:?}");
    let binding = xyz.into_raw_vec();
    let (x, y, _z) = binding.iter().collect_tuple().unwrap();
    (*x, *y, Y)
}

/// constrains sRGB to only positive, displayable values
pub fn sRGB_to_display(sRGB: (f32, f32, f32)) -> Rgb<u8> {
    let sR = sRGB.0;
    let sG = sRGB.1;
    let sB = sRGB.2;
    let sR = f32::max(0., sR); // lies about colorspace
    let sG = f32::max(0., sG); // lies about colorspace
    let sB = f32::max(0., sB); // lies about colorspace

    // if sR < 0. || sG < 0. || sB < 0. {
    //     // not visible light!
    //     return Rgb([
    //         f32::max(0., sR) as u8,
    //         f32::max(0., sG) as u8,
    //         f32::max(0., sB) as u8,
    //     ]);
    // }

    // println!("{x} {y}");
    let sR = pixel_ready(sR);
    let sG = pixel_ready(sG);
    let sB = pixel_ready(sB);

    Rgb([sR, sG, sB])
}

pub fn sRGB_apply_gamma(V: f32) -> f32 {
    if V <= 0.0031308 {
        V * 12.92
    } else {
        1.055 * V.powf(1. / 2.4) - 0.055
    }
}

pub fn sRGB_remove_gamma(V: f32) -> f32 {
    if V <= 0.0031308 * 12.92 {
        V / 12.92
    } else {
        ((V + 0.055) / 1.055).powf(2.4)
        // 1.055 * V.powf(1. / 2.4) - 0.055
    }
}

pub fn pixel_ready(x: f32) -> u8 {
    (255. * x) as u8
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{
        color::colorspace_conversion::{
            sRGB_apply_gamma, sRGB_remove_gamma, sRGB_to_xyY, xyY_to_CIEXYZ, xyY_to_sRGB,
            CIEXYZ_to_xyY,
        },
        geometry::primitives::Vector,
    };

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn gamma_inversion() {
        for i in -50..50 {
            let v = 10_f32.powf(i as f32 / 10.);
            let mut gamma = sRGB_apply_gamma(v);
            for _ in 0..10 {
                let vp = sRGB_remove_gamma(gamma);
                gamma = sRGB_apply_gamma(vp);
            }

            let vp = sRGB_remove_gamma(gamma);
            assert_abs_diff_eq!(v, vp, epsilon = 1e0);
        }
    }
    #[test]
    fn sRGB_xyY_white() {
        let xyY = (1., 1., 1.);
        let sRGB = xyY_to_sRGB(xyY);
        // println!("{:?}", sRGB);
        let xyY_prime = sRGB_to_xyY(sRGB);
        assert_abs_diff_eq!(xyY.0, xyY_prime.0, epsilon = 1e0);
        assert_abs_diff_eq!(xyY.1, xyY_prime.1, epsilon = 1e0);
        assert_abs_diff_eq!(xyY.2, xyY_prime.2, epsilon = 1e0);
    }
    #[test]
    fn sRGB_xyY_other() {
        let xyY = (39.2, 76.8, 55.2);
        let sRGB = xyY_to_sRGB(xyY);
        // println!("{:?}", sRGB);
        let xyY_prime = sRGB_to_xyY(sRGB);
        println!("xyY_prime: {:?}", xyY_prime);
        assert_abs_diff_eq!(xyY.0, xyY_prime.0 / xyY.2, epsilon = 1e-4);
        assert_abs_diff_eq!(xyY.1, xyY_prime.1 / xyY.2, epsilon = 1e-4);
    }
    #[test]
    fn CIEXYZ_xyY() {
        let xyY = (39.2, 76.8, 55.2);
        let XYZ = xyY_to_CIEXYZ(xyY);
        let xyY_prime = CIEXYZ_to_xyY(XYZ);
        println!("xyY_prime: {:?}", xyY_prime);
        assert_abs_diff_eq!(xyY.0, xyY_prime.0, epsilon = 1e-12);
        assert_abs_diff_eq!(xyY.1, xyY_prime.1, epsilon = 1e-12);
        assert_abs_diff_eq!(xyY.2, xyY_prime.2, epsilon = 1e-12);
    }
    // #[test]
    // fn CIEXYZ_sRGB() {
    //     let XYZ = (1., 7., 12.);
    //     let xyY = CIEXYZ_to_xyY(XYZ);
    //     let sRGB = xyY_to_sRGB(xyY);
    //     let xyY_prime = sRGB_to_xyY(sRGB);
    //     let XYZ_prime = xyY_to_CIEXYZ(xyY_prime);
    //     println!("XYZ: {:?}",XYZ);
    //     println!("XYZ_prime: {:?}",XYZ_prime);
    //     assert_abs_diff_eq!(XYZ.0, XYZ_prime.0, epsilon = 1e-12);
    //     assert_abs_diff_eq!(XYZ.1, XYZ_prime.1, epsilon = 1e-12);
    //     assert_abs_diff_eq!(XYZ.2, XYZ_prime.2, epsilon = 1e-12);
    // }
}
