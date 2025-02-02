#![allow(mixed_script_confusables)] // allows unicode characters
                                    // use std::time::Duration;
mod application;
mod camera;
mod geometry_pipeline;
mod lighting;
mod line_plotting;
mod load_object_file;
mod luminous_efficiency;
mod orientation;
mod path_tracing;
mod pixel_shader;
mod primitives;
mod rasterization;
mod rendering_equation;
mod scene;
mod transformations;

extern crate stopwatch;

use image::{ImageBuffer, ImageFormat, Rgb};
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
                                                               // const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000/60 as u64;// ms/frame @ 60 fps

fn main() {
    check_debug();
    // load_obj("models/cube.obj".to_string());
    main_loop()
    // single(0)
}
