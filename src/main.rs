#![allow(mixed_script_confusables)] // allows unicode characters
                                    // use std::time::Duration;
mod application;
mod camera;
mod orientation;
mod geometry_pipeline;
mod lighting;
mod line_plotting;
mod load_object_file;
mod path_tracing;
mod pixel_shader;
mod primitives;
mod rasterization;
mod scene;
mod transformations;
mod rendering_equation;
mod luminous_efficiency;

extern crate stopwatch;

use image::{ImageBuffer, ImageFormat, Rgb};
use std::{thread, time::Duration};
use stopwatch::Stopwatch;

use crate::geometry_pipeline::geometry_pipeline;
use crate::scene::simple_scene;

fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms as u64));
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
const REST: u64 = 1000 / 8 as u64; // ms/frame @ 8 fps
                                   // const REST: u64 = 1000 / 12 as u64; // const REST: u64 = 1000/12 as u64;// ms/frame @ 12 fps
                                   // const REST: u64 = 1000 / 24 as u64; // const REST: u64 = 1000/24 as u64;// ms/frame @ 24 fps
                                   // const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000 / 60 as u64; // const REST: u64 = 1000/60 as u64;// ms/frame @ 60 fps

fn main() {
    check_debug();
    // load_obj("models/cube.obj".to_string());
    main_loop()
    // single(0)
}
