use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/geometry_pipeline.rs"]
mod geometry_pipeline;

#[path = "../src/scene.rs"]
mod scene;

// #[path = "../src/main.rs"]
// mod main;


#[path = "../src/coordinate_space.rs"]
mod coordinate_space;

#[path = "../src/line_plotting.rs"]
mod line_plotting;

#[path = "../src/primitives.rs"]
pub mod primitives;

#[path = "../src/transformations.rs"]
mod transformations;

// mod window;
// mod draw_box;
#[path = "../src/camera.rs"]
mod camera;

#[path = "../src/lighting.rs"]
mod lighting;

// use crate::geometry_pipeline::geometry_pipeline;

// use crate::scene::simple_scene;



// use main::save_image;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let scene = scene::simple_scene();
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("test scene", |b| b.iter(|| geometry_pipeline::geometry_pipeline(black_box(scene.clone()))));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);