#![allow(mixed_script_confusables)] // allows unicode characters
                                    // use std::time::Duration;
mod application;
mod camera;
mod color;
// cie_color_matching_functions;
// mod colorspace_conversion;
mod geometry_pipeline;
mod lighting;
mod load_object_file;

mod scene;

mod geometry;
mod rasterization;
mod ray_tracing;

extern crate stopwatch;

use color::colorspace_conversion::{spectra_to_CIEXYZ, spectra_to_sRGB, CIEXYZ_to_xyY};
use color::draw_chromaticity_diagram::coloring_book;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use lighting::monochroma_spectra;
use std::{thread, time::Duration};
use stopwatch::Stopwatch;

use crate::geometry_pipeline::geometry_pipeline;
use crate::scene::simple_scene;

fn sleep(ms: Duration) {
    thread::sleep(ms);
}

#[inline]
pub fn save_image(canvas: ImageBuffer<Rgb<u8>, Vec<u8>>) -> () {
    canvas
        .save_with_format("rust-output.png", ImageFormat::Png)
        .unwrap();
}

pub fn check_debug() {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    println!("Debugging disabled");
}

// builds a scene and renders it over and over
fn main_loop() {
    // let mut scene;
    let mut counter: usize = 0;
    loop {
        let mut frame = Stopwatch::start_new();

        println!("");
        single(counter);
        // sleep(REST);
        frame.stop();
        println!("frame: {:?}", frame.elapsed());
        if REST > frame.elapsed() {
            let wait = REST - frame.elapsed(); // accounts for time already elapsed
            sleep(wait);
        }

        counter += 1;
        // counter %= 360.0;
    }
}
fn single(i: usize) {
    let mut scene;
    scene = simple_scene();
    scene.tick = i;
    let render = geometry_pipeline(scene);
    save_image(render);
}
// REST is ms per frame
// const REST: u64 = 1000 / 1 as u64; // ms/frame @ 1 fps
const REST: Duration = Duration::from_millis(1000 / 8 as u64); // ms/frame @ 8 fps
                                                               // const REST: u64 = 1000 / 12 as u64; // const REST: u64 = 1000/12 as u64;// ms/frame @ 12 fps
                                                               // const REST: u64 = 1000 / 24 as u64; // const REST: u64 = 1000/24 as u64;// ms/frame @ 24 fps
fn draw_colors() {
    let horizontal_res = 1_000;
    let vertical_res = horizontal_res;
    let mut canvas: RgbImage = ImageBuffer::new(horizontal_res, vertical_res);
    coloring_book(&mut canvas);
    // draw_colors_in_xyz(&mut canvas);
    canvas
        .save_with_format("color_gamut.png", ImageFormat::Png)
        .unwrap();
}
fn main() {
    check_debug();
    println!("{:?}", (spectra_to_sRGB(&monochroma_spectra(500., 1.))));
    println!("{:?}", (spectra_to_CIEXYZ(&monochroma_spectra(500., 1.))));
    println!(
        "{:?}",
        CIEXYZ_to_xyY(spectra_to_CIEXYZ(&monochroma_spectra(500., 1.)))
    );

    draw_colors();

    main_loop();
    // single(0)
}
