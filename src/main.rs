use std::time::Duration;

use image::{ImageBuffer, Rgb, RgbImage};

extern crate stopwatch;
use stopwatch::Stopwatch;

pub mod line_plotting;

struct Vertex {
    // point in 3D space
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    fn to_point(&self) -> line_plotting::Point {
        let factor: f32 = self.z;
        return line_plotting::Point {
            x: (self.x.round() / factor) as i32,
            y: (self.y.round() / factor) as i32,
        };
    }
}

struct Polygon {
    a: Vertex,
    b: Vertex,
    c: Vertex,
}

impl Polygon {
    fn draw(&self, canvas: &mut RgbImage) {
        line_plotting::plot_line(canvas, &self.a.to_point(), &self.b.to_point(), COLOR);
        line_plotting::plot_line(canvas, &self.b.to_point(), &self.c.to_point(), COLOR);
        line_plotting::plot_line(canvas, &self.c.to_point(), &self.a.to_point(), COLOR);
    }
}

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;
const _HALF_WIDTH: u32 = WIDTH / 2;
const LINES: u32 = WIDTH/8;
const FRAMERATE: u32 = 60;
const FRAMEBUDGET: f64 = 1000.0 / FRAMERATE as f64;

const COLOR: Rgb<u8> = Rgb([0, 255, 0]);

fn main() {
    // ImageBuffer is static, while DynamicImage is dynamic
    // new variable canvas, mutable, type RgbImage, equal to a new ImageBuffer of WIDTH and HEIGHT
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    let mut glyph: RgbImage = ImageBuffer::new(3, 3);
    glyph.put_pixel(0, 0, Rgb([0,0,0]));
    glyph.put_pixel(0, 1, Rgb([255,0,0]));
    glyph.put_pixel(0, 2, Rgb([0,0,0]));
    glyph.put_pixel(1, 0, Rgb([255,0,0]));
    glyph.put_pixel(1, 1, Rgb([255,0,0]));
    glyph.put_pixel(1, 2, Rgb([255,0,0]));
    glyph.put_pixel(2, 0, Rgb([0,0,0]));
    glyph.put_pixel(2, 1, Rgb([255,0,0]));
    glyph.put_pixel(2, 2, Rgb([0,0,0]));
    canvas.fill(30);
    let mut sw = Stopwatch::start_new();
    graphics_routine(&mut canvas);
    sw.stop();
    print_info(sw);

    println!("saving image");
    canvas.save("../output.bmp").unwrap();
    canvas.save("../output.png").unwrap();
    println!("image saved");
}

fn print_info(stopwatch: Stopwatch) {
    let million: f64 = 1_000_000.0;
    let billion: f64 = 1_000_000_000.0;
    let clock_speed: f64 = 3.3 * billion;
    // let cycle_duration = Duration { secs: (1.0 / clock_speed) as u64};
    let cycle_duration = 1.0 / clock_speed; // in seconds
    
    let lines_per_second = LINES as f64 * 1.0 / (stopwatch.elapsed().as_nanos() as f64 / billion);
    let lines_per_frame = lines_per_second / FRAMEBUDGET;

    println!("{} lines per second", lines_per_second);
    println!("{} lines per frame", lines_per_frame);
    println!("{:?} elapsed", stopwatch.elapsed());
    println!("{} lines drawn", LINES);

    println!("{:?} taken per line", stopwatch.elapsed() / LINES);
    print!(
        "or, with a frame budget of {:?} milliseconds, ",
        FRAMEBUDGET
    );
    println!("{:?} lines per frame", lines_per_frame);
    println!("{:?} trigons per frame", lines_per_frame / 3.0);
    println!("{:?} cubes per frame", lines_per_frame / 24.0);
    // println!("{} cycles in a microsecond", clock_speed / million);
}
// fn box_blur(canvas: &mut RgbImage, size: u32) {
//     // not operational
//     let mut blurred_canvas: RgbImage = ImageBuffer::new(WIDTH + (size * 2), HEIGHT + (size * 2));
//     // loop over every pixel, then loop over it's effect
//     for pixel in canvas.enumerate_pixels() {
//         for y in 0..size * 2 { // for every row
//             for x in 0..size * 2 { // for every column
//                 blurred_canvas.put_pixel(pixel.0 + x + size, pixel.1 + y + size, *pixel.2);
//             }
//         }
//     }
//     canvas.clone_from(&blurred_canvas);
// }

// fn graphics_routine(canvas: &mut RgbImage) {
//     let a = Vertex {
//         x: 0.0,
//         y: 0.0,
//         z: 1.0,
//     };
//     let b = Vertex {
//         x: 1000.0,
//         y: 0.0,
//         z: 1.0,
//     };
//     let c = Vertex {
//         x: 0.0,
//         y: 1000.0,
//         z: 1.0,
//     };
//     let triangle = Polygon { a: a, b: b, c: c };
//     triangle.draw(canvas);
// }
fn graphics_routine(canvas: &mut RgbImage) {
    let point_0 = line_plotting::Point { x: 0, y: 0 };
    // let point_1 = Point { x: 50, y: 20 };
    let lines = LINES / 2;
    for x in 0..lines {
        let point_1 = line_plotting::Point {
            x: (x * WIDTH / lines) as i32,
            y: (HEIGHT) as i32,
        };
        line_plotting::plot_line(canvas, &point_0, &point_1, COLOR);
    }
    let point_0 = line_plotting::Point {
        x: 0,
        y: (HEIGHT - 1) as i32,
    };
    for x in 0..lines {
        let point_1 = line_plotting::Point {
            x: (x * WIDTH / lines) as i32,
            y: 0,
        };
        line_plotting::plot_line(canvas, &point_0, &point_1, COLOR);
    }
    // plot_line(&mut canvas, &point_0, &point_1, COLOR);
    // let point_1 = Point { x: 50, y: 100 };
    // plot_line(&mut canvas, &point_0, &point_1, COLOR);
}
