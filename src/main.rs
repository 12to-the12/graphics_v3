#![allow(mixed_script_confusables)] // allows unicode characters
                                    // use std::time::Duration;
mod coordinate_space;
mod line_plotting;
mod primitives;
mod transformations;
mod camera;
mod geometry_pipeline;
mod lighting;
mod scene;
mod application;
mod rasterization;
mod pixel_shader;

extern crate stopwatch;

use std::{thread, time::Duration};
use image::{ImageBuffer, ImageFormat, Rgb};
use stopwatch::Stopwatch;

use crate::geometry_pipeline::geometry_pipeline;
use crate::scene::simple_scene;

fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms as u64));
}

#[inline]
pub fn save_image(canvas: ImageBuffer<Rgb<u8>, Vec<u8>>) -> () {
    let mut image_save = Stopwatch::start_new();

    canvas
        .save_with_format("rust-output.bmp", ImageFormat::Bmp)
        .unwrap();
    image_save.stop();
    println!("image_save: {:?}", image_save.elapsed());
}

pub fn check_debug() {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    println!("Debugging disabled");
}

fn main_loop() {
    let mut scene;
    let mut counter: f32 = 0.0;
    loop {
        println!("");
        scene = simple_scene();

        let render = geometry_pipeline(scene, counter);
        save_image(render);
        sleep(REST);
        counter += 1.0;
        counter %= 360.0;
    }
}
// const REST: u64 = 1000 / 1 as u64; // ms/frame @ 1 fps
// const REST: u64 = 1000 / 8 as u64; // ms/frame @ 8 fps
const REST: u64 = 1000 / 12 as u64; // const REST: u64 = 1000/12 as u64;// ms/frame @ 12 fps
// const REST: u64 = 1000 / 24 as u64; // const REST: u64 = 1000/24 as u64;// ms/frame @ 24 fps
// const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000/60 as u64;// ms/frame @ 60 fps

fn main() {
    check_debug();

    main_loop()
}
