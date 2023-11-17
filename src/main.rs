#![allow(mixed_script_confusables)] // allows unicode characters
                                    // use std::time::Duration;

// use image::open;

extern crate stopwatch;
use crate::transformations::{
    build_scale_matrix, build_x_rotation_matrix, build_y_rotation_matrix, build_z_rotation_matrix,
    Transform,
};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

use crate::primitives::vector;
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

use std::{thread, time::Duration};

use stopwatch::Stopwatch;

// mod import_config;
// use import_config::Config;
use crate::geometry_pipeline::geometry_pipeline;

use crate::scene::{simple_scene, Scene};

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

pub fn render_scene(scene: Scene) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut render_time = Stopwatch::start_new();
    let render = geometry_pipeline(scene);
    render_time.stop();
    println!("render: {:?}", render_time.elapsed());
    render
}

fn transform(scene: &mut Scene, counter: f32) {
    let mesh = &mut scene.meshes[0];
    mesh.add_transform(build_scale_matrix(vector(2.0, 1.0, 1.0)));
    mesh.add_transform(build_x_rotation_matrix(counter.to_radians()));
    mesh.add_transform(build_y_rotation_matrix(-counter.to_radians()));
    mesh.add_transform(build_z_rotation_matrix(-0.5 * counter.to_radians()));
}
fn main_loop() {
    let mut scene;
    let mut render: ImageBuffer<Rgb<u8>, Vec<u8>>;
    let mut counter: f32 = 0.0;
    loop {
        scene = simple_scene();
        transform(&mut scene, counter);

        render = render_scene(scene);
        save_image(render);
        sleep(16);
        counter += 1.0;
    }
}
fn main() {
    check_debug();

    main_loop()
}
