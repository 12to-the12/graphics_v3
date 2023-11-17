use image::{ImageBuffer, Rgb, RgbImage};

use stopwatch::Stopwatch;
use crate::primitives::Point;

use crate::line_plotting;
// pub mod crate::line_plotting;
// pub mod dot_plotting;

const WIDTH: u32 = 1920 * 2;
const HEIGHT: u32 = 1080 * 2;
const _HALF_WIDTH: u32 = WIDTH / 2;
const LINES: u32 = WIDTH / 16;
const _FRAMERATE: u32 = 60;
const _FRAMEBUDGET: f64 = 1000.0 / _FRAMERATE as f64;

const ITERATIONS: usize = 10; // tests to do

const COLOR: Rgb<u8> = Rgb([0, 255, 0]);


pub fn benchmark() {
    let mut overwatcher = Stopwatch::start_new();
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    // canvas.fill(255);

    let mut time_samples = vec![0; ITERATIONS];

    println!("init: {:?}", overwatcher.elapsed_ms());
    for i in 0..ITERATIONS {
        let mut sw = Stopwatch::start_new();
        graphics_routine(&mut canvas);
        sw.stop();
        time_samples[i] = sw.elapsed().as_nanos();
        // println!("{:?}", time_samples[i]);
        // print_info(sw);
    }
    // println!("{:?}",time_samples);
    // ________________________________

    println!(
        "naive mean: {:?} ms @ {} iterations",
        mean(&time_samples) / 1_000_000.0,
        ITERATIONS
    );
    println!("sum: {:?} ms", sum(&time_samples) / 1_000_000.0);

    let mut time_samples = vec![0; ITERATIONS];

    for i in 0..ITERATIONS {
        let mut sw = Stopwatch::start_new();
        graphics_routine_fast(&mut canvas);
        sw.stop();
        time_samples[i] = sw.elapsed().as_nanos();
        // println!("{:?}", time_samples[i]);
        // print_info(sw);
    }
    // println!("{:?}",time_samples);

    println!(
        "fast mean: {:?} ms @ {} iterations",
        mean(&time_samples) / 1_000_000.0,
        ITERATIONS
    );
    println!("sum: {:?} ms", sum(&time_samples) / 1_000_000.0);
    let mut image_save = Stopwatch::start_new();
    // println!("saving image");
    // canvas.save("../output.bmp").unwrap();
    canvas.save("rust-output.png").unwrap();
    // println!("image saved");
    image_save.stop();
    println!("image_save: {:?}", image_save.elapsed());
    overwatcher.stop();
    println!("total: {:?}", overwatcher.elapsed());
}

// fn std(array: &[Duration]) -> f32 {
//     let mean = mean(&array);
//     for entry in array {

//     }
// }

fn mean(array: &[u128]) -> f32 {
    let sum = sum(&array);
    let mean = sum / array.len() as f32;
    mean
}

fn sum(array: &[u128]) -> f32 {
    let mut sum: f32 = 0.0;
    for entry in array {
        sum += *entry as f32;
    }
    sum
}



fn graphics_routine(canvas: &mut RgbImage) {
    let point_0 = Point { x: 0, y: 0 };
    // let point_1 = Point { x: 50, y: 20 };

    for x in 0..LINES - 1 {
        let point_1 = Point {
            x: (x * WIDTH / LINES) as i32,
            y: (HEIGHT - 1) as i32,
        };
        line_plotting::plot_line_naive(canvas, &point_0, &point_1, COLOR);
    }
}

fn graphics_routine_fast(canvas: &mut RgbImage) {
    let point_0 = Point { x: 0, y: 0 };
    // let point_1 = Point { x: 50, y: 20 };

    for x in 0..LINES - 1 {
        let point_1 = Point {
            x: (x * WIDTH / LINES) as i32,
            y: (HEIGHT - 1) as i32,
        };
        line_plotting::plot_line(canvas, &point_0, &point_1, COLOR);
    }
}
