#![allow(mixed_script_confusables)] // allows unicode characters
                                    // use std::time::Duration;

// use image::open;

extern crate stopwatch;
use image::{ImageBuffer, Rgb, RgbImage};

// extern crate ndarray;

mod benchmark;
mod coordinate_space;
mod line_plotting;
pub mod primitives;
mod transformations;
// mod window;
// mod draw_box;
mod camera;
mod geometry_pipeline;
mod lighting;
mod scene;

// use std::io;

use stopwatch::Stopwatch;

// mod import_config;
// use import_config::Config;
use crate::geometry_pipeline::geometry_pipeline;

use crate::scene::simple_scene;

#[inline]
pub fn save_image(canvas: ImageBuffer<Rgb<u8>, Vec<u8>>) -> () {
    canvas.save("rust-output.png").unwrap();
}

fn main() {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    println!("Debugging disabled");

    let scene = simple_scene();
    // println!("{}", scene.camera.sensor.horizontal_res);
    // println!("{}", scene.camera.sensor.vertical_res);
    let mut render_time = Stopwatch::start_new();
    let render = geometry_pipeline(scene);

    render_time.stop();
    println!("render: {:?}", render_time.elapsed());

    // save_image(render);
    let mut image_save = Stopwatch::start_new();
    render.save("rust-output.png").unwrap();

    image_save.stop();
    println!("image_save: {:?}", image_save.elapsed());
}
