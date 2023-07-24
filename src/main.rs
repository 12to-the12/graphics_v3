use image::{ImageBuffer, Rgb, RgbImage};

extern crate stopwatch;
use stopwatch::Stopwatch;

struct Point {
    x: i32,
    y: i32,
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

const COLOR: Rgb<u8> = Rgb([255, 0, 255]);

fn main() {
    // ImageBuffer is static, while DynamicImage is dynamic
    // new variable canvas, mutable, type RgbImage, equal to a new ImageBuffer of WIDTH and HEIGHT
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    // do something that takes some time

    let point_0 = Point { x: 0, y: 0 };
    // let point_1 = Point { x: 0, y: 0 };

    let sw = Stopwatch::start_new();
    for x in 0..100 {
        let point_1 = Point { x: x * 10, y: 1000 };
        draw_line(&mut canvas, &point_0, &point_1, COLOR);
    }
    // draw_line(&mut canvas, &point_0, &point_1);
    println!("{:?}", sw.elapsed());

    // draw(&mut canvas);
    println!("saving");
    canvas.save("output.bmp").unwrap();
    println!("and saved");
}

fn draw_line(canvas: &mut RgbImage, p0: &Point, p1: &Point, color: Rgb<u8> ) {
    let mut p0 = p0;
    let mut p1 = p1;
    if p0.x > p1.x {
        // println!("called");
        // let (p1, p0) = (p0, p1);

        let temp = p0;
        p0 = p1;
        p1 = temp;
    }
    // println!("{:?}", b_x);
    // rasterizes lines
    let slope: f32 = (p1.y - p0.y) as f32 / (p1.x - p0.x) as f32;
    // println!("{p0.x} {p0.y} {p1.x} {p1.y}");
    // println!("slope: {slope}");
    if slope.abs() > 1.0 {
        // it's better to place a pixel for every y value
        // println!("it's better to place a pixel for every y value");
        let mut x: f32 = p0.x as f32;
        for y in p0.y..(p1.y + 1) {
            // println!("{x}, {y} {slope}");
            canvas.put_pixel((x + 0.5) as u32, y as u32, color);
            x += 1.0 / slope;
        }
    } else {
        // it's better to place a pixel for every x value
        // println!("it's better to place a pixel for every x value");
        let mut y: f32 = p0.y as f32;
        for x in p0.x..(p1.x + 1) {
            // println!("{x}, {} {slope}", (y+ 0.5) as u32);
            canvas.put_pixel(x as u32, (y + 0.5) as u32, color);
            y += slope;
        }
    }
}

// fn box_blur(canvas: &RgbImage, size: u32) -> RgbImage {
//     let mut blurred_canvas: RgbImage = canvas.clone();
//     // loop over every pixel, then loop over it's effect
//     for pixel in canvas.enumerate_pixels(){
//         blurred_canvas.put_pixel(pixel.0, pixel.1, pixel.2);
//     }
//     blurred_canvas
//
// }
