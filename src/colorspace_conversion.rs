use image::{ImageBuffer, Rgb, RgbImage};

use crate::{
    cie_color_matching_functions::{
        integrated_x_response, integrated_y_response, integrated_z_response,
    },
    lighting::{black_body, norm_black_body, peak_blackbody, Spectra},
    primitives::{vector, Vector},
};

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
/// converts from
pub fn CIEXYZ_to_sRGB(x: f32, y: f32) -> Rgb<u8> {
    let z = 1. - x - y;

    let sR_linear = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let sG_linear = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let sB_linear = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

    let sR = sRGB_apply_gamma(sR_linear);
    let sG = sRGB_apply_gamma(sG_linear);
    let sB = sRGB_apply_gamma(sB_linear);
    if sR < 0. || sG < 0. || sB < 0. {
        // if sR > 1. || sG > 1. || sB > 1. || sR < 0. || sG < 0. || sB < 0. {

        // if sR > 1. || sG > 1. || sB > 1. {
        return Rgb([0, 0, 0]);
    }
    // println!("{x} {y}");
    let sR = pixel_ready(sR);
    let sG = pixel_ready(sG);
    let sB = pixel_ready(sB);

    return Rgb([sR, sG, sB]);
}
pub fn draw_colors_in_xyz(canvas: &mut RgbImage) {
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let x: f32 = (i as f32) / (canvas.width() as f32);
            let y: f32 = (canvas.height() - j - 1) as f32 / canvas.height() as f32;

            canvas.put_pixel(i, j, CIEXYZ_to_sRGB(x, y));
        }
    }

    canvas.put_pixel(
        (0.333 * (canvas.width() as f32)) as u32,
        (0.666 * (canvas.height() as f32)) as u32,
        Rgb([255, 255, 255]),
    );
}


pub fn black_body_xyY(temp:f32)->Vector{
    println!("peak wavelength: {}", peak_blackbody(temp));
    let spectra = norm_black_body(temp);
    let XYZ = spectra_to_CIEXYZ(spectra);
    // println!("{:?}", XYZ);

    let xyY = XYZ_to_xyY(XYZ);
    // println!("{:?}", xyY);

    xyY

}


pub fn black_body_sRGB(temp:f32)->Rgb<u8>{
    let xyY = black_body_xyY(temp);
    let sRGB = CIEXYZ_to_sRGB(xyY.x, xyY.y);
    sRGB

}
pub fn coloring_book(canvas: &mut RgbImage) {

    // let color = Rgb([255,0,0]);


    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let x: f32 = (i as f32) / (canvas.width() as f32);
            let y: f32 = (canvas.height() - j - 1) as f32 / canvas.height() as f32;

            canvas.put_pixel(i, j, CIEXYZ_to_sRGB(x, y));
        }
    }
    for temp in 1_000..100_000{
            let xyY = black_body_xyY(temp as f32);
    let sRGB = black_body_sRGB(temp as f32);

    // println!("{:?}", sRGB);
    canvas.put_pixel(
        (xyY.x * (canvas.width() as f32)) as u32,
        (canvas.height() as f32 -xyY.y * (canvas.height() as f32)) as u32,
        Rgb([255, 255, 255]),
    );
    }








    // for i in 0..canvas.width() {
    //     for j in 0..canvas.height() {
    //         canvas.put_pixel(i, j, sRGB);
    //     }
    // }
}

pub fn XYZ_to_xyY(XYZ: Vector) -> Vector {
    let X = XYZ.x;
    let Y = XYZ.y;
    let x = X / XYZ.magnitude();
    let y = Y / XYZ.magnitude();
    return vector(x, y, Y);
}
pub fn spectra_to_CIEXYZ(spectra: Spectra) -> Vector {
    let X = integrated_x_response(&spectra);
    let Y = integrated_y_response(&spectra);
    let Z = integrated_z_response(&spectra);
    return vector(X, Y, Z);
}
