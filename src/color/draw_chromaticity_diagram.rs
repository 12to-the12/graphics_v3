#![allow(nonstandard_style)]
use image::{Rgb, RgbImage};

use crate::{
    color::colorspace_conversion::{
        sRGB_to_display, spectra_to_CIEXYZ, xyY_to_sRGB, CIEXYZ_to_xyY,
    },
    lighting::{black_body, monochroma_spectra},
};

pub fn _draw_colors_in_xyz(canvas: &mut RgbImage) {
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let x: f32 = (i as f32) / (canvas.width() as f32);
            let y: f32 = (canvas.height() - j - 1) as f32 / canvas.height() as f32;

            canvas.put_pixel(i, j, sRGB_to_display(xyY_to_sRGB((x, y, 0.))));
        }
    }

    canvas.put_pixel(
        (0.333 * (canvas.width() as f32)) as u32,
        (0.666 * (canvas.height() as f32)) as u32,
        Rgb([255, 255, 255]),
    );
}

pub fn coloring_book(canvas: &mut RgbImage) {
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let x: f32 = (i as f32) / (canvas.width() as f32);
            let y: f32 = (canvas.height() - j - 1) as f32 / canvas.height() as f32;

            canvas.put_pixel(i, j, sRGB_to_display(xyY_to_sRGB((x, y, 1.))));
        }
    }

    for λ in 380..700 {
        let spectra =
            monochroma_spectra((λ as f32) / 1., 1., crate::lighting::RadiometricUnit::Flux);
        let xyY = CIEXYZ_to_xyY(spectra_to_CIEXYZ(&spectra));
        canvas.put_pixel(
            (xyY.0 * (canvas.width() as f32)) as u32,
            (canvas.height() as f32 - xyY.1 * (canvas.height() as f32)) as u32,
            Rgb([255, 255, 255]),
        );
    }
    // let div = 2;
    // for i in 1..(div) {
    //     for j in 0..canvas.height() {
    //         canvas.put_pixel(canvas.width() / div * i, j, Rgb([255, 255, 255]));
    //         canvas.put_pixel(j, canvas.height() / div * i, Rgb([255, 255, 255]));
    //     }
    // }

    // let div = 3;
    // for i in 1..(div) {
    //     for j in 0..canvas.height() {
    //         canvas.put_pixel(canvas.width() / div * i, j, Rgb([255, 255, 255]));
    //         canvas.put_pixel(j, canvas.height() / div * i, Rgb([255, 255, 255]));
    //     }
    // }

    for temp in 1_000..100_000 {
        let spectra = black_body(temp as f32, crate::lighting::RadiometricUnit::Flux);
        let xyY = CIEXYZ_to_xyY(spectra_to_CIEXYZ(&spectra));

        canvas.put_pixel(
            (xyY.0 * (canvas.width() as f32)) as u32,
            (canvas.height() as f32 - xyY.1 * (canvas.height() as f32)) as u32,
            Rgb([255, 255, 255]),
        );
    }
}
