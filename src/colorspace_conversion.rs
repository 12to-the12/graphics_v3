use image::{ImageBuffer, Rgb, RgbImage};

use crate::{
    cie_color_matching_functions::{
        integrated_x_response, integrated_y_response, integrated_z_response,
    },
    lighting::{black_body, monochroma_spectra, norm_black_body, peak_blackbody, Spectra},
    primitives::{vector, Vector},
};

pub fn draw_colors_in_xyz(canvas: &mut RgbImage) {
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let x: f32 = (i as f32) / (canvas.width() as f32);
            let y: f32 = (canvas.height() - j - 1) as f32 / canvas.height() as f32;

            canvas.put_pixel(i, j, sRGB_to_display(xyY_to_sRGB(vector(x, y, 0.))));
        }
    }

    canvas.put_pixel(
        (0.333 * (canvas.width() as f32)) as u32,
        (0.666 * (canvas.height() as f32)) as u32,
        Rgb([255, 255, 255]),
    );
}

pub fn black_body_xyY(temp: f32) -> Vector {
    // println!("peak wavelength: {}", peak_blackbody(temp));
    let spectra = norm_black_body(temp);
    let XYZ = spectra_to_CIEXYZ(&spectra);
    let xyY = CIEXYZ_to_xyY(XYZ);
    xyY
}

pub fn black_body_sRGB(temp: f32) -> Vector {
    spectra_to_sRGB(&norm_black_body(temp))
}
pub fn coloring_book(canvas: &mut RgbImage) {
    // let color = Rgb([255,0,0]);

    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let x: f32 = (i as f32) / (canvas.width() as f32);
            let y: f32 = (canvas.height() - j - 1) as f32 / canvas.height() as f32;

            canvas.put_pixel(i, j, sRGB_to_display(xyY_to_sRGB(vector(x, y, 1.))));
        }
    }
    for λ in 380..780 {
        let spectra = monochroma_spectra(λ as f32, 1.);
        let xyY = CIEXYZ_to_xyY(spectra_to_CIEXYZ(&spectra));
        let sRGB = spectra_to_sRGB(&spectra);
        // println!("{:?}", sRGB);

        // println!("{:?}", sRGB);
        canvas.put_pixel(
            (xyY.x * (canvas.width() as f32)) as u32,
            (canvas.height() as f32 - xyY.y * (canvas.height() as f32)) as u32,
            Rgb([255, 255, 255]),
        );
    }
    let div = 2;
    for i in 1..(div) {
        for j in 0..canvas.height() {
            canvas.put_pixel(canvas.width() / div * i, j, Rgb([255, 255, 255]));
            canvas.put_pixel(j, canvas.height() / div * i, Rgb([255, 255, 255]));
        }
    }

    let div = 3;
    for i in 1..(div) {
        for j in 0..canvas.height() {
            canvas.put_pixel(canvas.width() / div * i, j, Rgb([255, 255, 255]));
            canvas.put_pixel(j, canvas.height() / div * i, Rgb([255, 255, 255]));
        }
    }

    // let div = 10;
    // for i in 1..(div) {
    //     for j in 0..canvas.height() {
    //         canvas.put_pixel(canvas.width()/div*i, j, Rgb([255, 255, 255]));
    //         canvas.put_pixel(j, canvas.height()/div*i, Rgb([255, 255, 255]));
    //     }
    // }

    for temp in 1_000..100_000 {
        let spectra = black_body(temp as f32);
        let xyY = CIEXYZ_to_xyY(spectra_to_CIEXYZ(&spectra));
        let sRGB = spectra_to_sRGB(&spectra);
        // println!("{:?}", sRGB);

        // println!("{:?}", sRGB);
        canvas.put_pixel(
            (xyY.x * (canvas.width() as f32)) as u32,
            (canvas.height() as f32 - xyY.y * (canvas.height() as f32)) as u32,
            Rgb([255, 255, 255]),
        );
    }

    // for i in 0..canvas.width() {
    //     for j in 0..canvas.height() {
    //         canvas.put_pixel(i, j, sRGB);
    //     }
    // }
}

// conversions

pub fn spectra_to_sRGB(spectra: &Spectra) -> Vector {
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

pub fn spectra_to_CIEXYZ(spectra: &Spectra) -> Vector {
    let X = integrated_x_response(&spectra);
    let Y = integrated_y_response(&spectra);
    let Z = integrated_z_response(&spectra);
    return vector(X, Y, Z);
}

pub fn CIEXYZ_to_xyY(XYZ: Vector) -> Vector {
    let X = XYZ.x;
    let Y = XYZ.y;
    let Z = XYZ.z;
    let x = X / (X + Y + Z);
    let y = Y / (X + Y + Z);
    return vector(x, y, Y);
}

/// I think? it might supposed to be CIEXYZ?
/// NOTE: this is not u8 encoded! it's possibly negative!
pub fn xyY_to_sRGB(xyY: Vector) -> Vector {
    // yeah, yeah, bad vector implementation I know
    let x: f32 = xyY.x;
    let y: f32 = xyY.y;
    let Y: f32 = xyY.z;
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
    let factor = 2.5;
    let sR_linear = (sR_linear + (factor - 1.)) / factor;
    let sG_linear = (sG_linear + (factor - 1.)) / factor;
    let sB_linear = (sB_linear + (factor - 1.)) / factor;

    let sR = sRGB_apply_gamma(sR_linear);
    let sG = sRGB_apply_gamma(sG_linear);
    let sB = sRGB_apply_gamma(sB_linear);
    vector(sR, sG, sB)
}

/// constrains sRGB to only positive, displayable values
pub fn sRGB_to_display(sRGB: Vector) -> Rgb<u8> {
    let sR = sRGB.x;
    let sG = sRGB.y;
    let sB = sRGB.z;





    if sR < 0. || sG < 0. || sB < 0. {
        // if sR > 1. || sG > 1. || sB > 1. || sR < 0. || sG < 0. || sB < 0. {

        // if sR > 1. || sG > 1. || sB > 1. {
        let sR = pixel_ready(sR / 5.);
        let sG = pixel_ready(sG / 5.);
        let sB = pixel_ready(sB / 5.);
        // return Rgb([sR, sG, sB]);

        return Rgb([0, 0, 0]);
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
