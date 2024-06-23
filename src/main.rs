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
mod path_tracing;
mod load_object_file;

extern crate stopwatch;

use std::{thread, time::Duration};
use image::{ImageBuffer, ImageFormat, Rgb};
use stopwatch::Stopwatch;

use crate::geometry_pipeline::geometry_pipeline;
use crate::scene::simple_scene;

use crate::load_object_file::load_obj;

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

// builds a scene and renders it over and over
fn main_loop() {
    let mut scene;
    let mut counter: usize = 0;
    loop {
        println!("");
        scene = simple_scene();
        scene.tick = counter;

        let render = geometry_pipeline(scene);
        save_image(render);
        sleep(REST);
        counter += 1;
        // counter %= 360.0;
    }
}
// REST is ms per frame
// const REST: u64 = 1000 / 1 as u64; // ms/frame @ 1 fps
// const REST: u64 = 1000 / 8 as u64; // ms/frame @ 8 fps
const REST: u64 = 1000 / 12 as u64; // const REST: u64 = 1000/12 as u64;// ms/frame @ 12 fps
// const REST: u64 = 1000 / 24 as u64; // const REST: u64 = 1000/24 as u64;// ms/frame @ 24 fps
// const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000/60 as u64;// ms/frame @ 60 fps

fn main() {
    check_debug();
    // load_obj("models/cube.obj".to_string());
    main_loop()
}
