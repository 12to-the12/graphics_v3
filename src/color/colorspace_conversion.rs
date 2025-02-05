#![allow(nonstandard_style)]
use image::Rgb;

use crate::{
    color::cie_color_matching_functions::{
        integrated_x_response, integrated_y_response, integrated_z_response,
    },
    lighting::{norm_black_body, Spectra},
};

pub fn _black_body_xyY(temp: f32) -> (f32, f32, f32) {
    // println!("peak wavelength: {}", peak_blackbody(temp));
    let spectra = norm_black_body(temp);
    let XYZ = spectra_to_CIEXYZ(&spectra);
    let xyY = CIEXYZ_to_xyY(XYZ);
    xyY
}

pub fn _black_body_sRGB(temp: f32) -> (f32, f32, f32) {
    spectra_to_sRGB(&norm_black_body(temp))
}

pub fn spectra_to_sRGB(spectra: &Spectra) -> (f32, f32, f32) {
    let XYZ = spectra_to_CIEXYZ(spectra);
    let xyY = CIEXYZ_to_xyY(XYZ);
    let sRGB = xyY_to_sRGB(xyY);
    sRGB
}

pub fn spectra_to_display(spectra: &Spectra) -> Rgb<u8> {
    let XYZ = spectra_to_CIEXYZ(spectra);
    let xyY = CIEXYZ_to_xyY(XYZ);
    let sRGB = xyY_to_sRGB(xyY);
    let display = sRGB_to_display(sRGB);
    display
}

pub fn spectra_to_CIEXYZ(spectra: &Spectra) -> (f32, f32, f32) {
    let X = integrated_x_response(&spectra);
    let Y = integrated_y_response(&spectra);
    let Z = integrated_z_response(&spectra);
    (X, Y, Z)
}

pub fn CIEXYZ_to_xyY(XYZ: (f32, f32, f32)) -> (f32, f32, f32) {
    let X = XYZ.0;
    let Y = XYZ.1;
    let Z = XYZ.2;
    let x = X / (X + Y + Z);
    let y = Y / (X + Y + Z);
    return (x, y, Y);
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
    let mut sG_linear = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let mut sB_linear = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

    // I pray this is the correct way to do things
    sR_linear *= Y;
    sG_linear *= Y;
    sB_linear *= Y;

    // NOTE: THIS DEVIATES FROM AN ACCURATE COLOR MODEL
    // I think this will desaturate?
    // let factor = 1.2;
    let factor = 1.04;
    let sR_linear = (sR_linear + (factor - 1.)) / factor;
    let sG_linear = (sG_linear + (factor - 1.)) / factor;
    let sB_linear = (sB_linear + (factor - 1.)) / factor;

    let sR = sRGB_apply_gamma(sR_linear);
    let sG = sRGB_apply_gamma(sG_linear);
    let sB = sRGB_apply_gamma(sB_linear);
    (sR, sG, sB)
}

/// constrains sRGB to only positive, displayable values
pub fn sRGB_to_display(sRGB: (f32, f32, f32)) -> Rgb<u8> {
    let sR = sRGB.0;
    let sG = sRGB.1;
    let sB = sRGB.2;

    if sR < 0. || sG < 0. || sB < 0. {
        // if sR > 1. || sG > 1. || sB > 1. || sR < 0. || sG < 0. || sB < 0. {

        // if sR > 1. || sG > 1. || sB > 1. {
        // let sR = pixel_ready(sR / 5.);
        // let sG = pixel_ready(sG / 5.);
        // let sB = pixel_ready(sB / 5.);
        // return Rgb([sR, sG, sB]);

        return Rgb([255, 0, 255]);
    }
    // println!("{x} {y}");
    let sR = pixel_ready(sR);
    let sG = pixel_ready(sG);
    let sB = pixel_ready(sB);

    return Rgb([sR, sG, sB]);
}

pub fn sRGB_apply_gamma(V: f32) -> f32 {
    if V <= 0.0031308 {
        return V * 12.92;
    } else {
        return 1.055 * V.powf(1. / 2.4) - 0.055;
    }
}
pub fn pixel_ready(x: f32) -> u8 {
    (255. * x) as u8
}
