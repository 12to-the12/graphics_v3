use image::{ImageBuffer, Rgb, RgbImage};

extern crate stopwatch;
use stopwatch::Stopwatch;

struct Point {
    x: i32,
    y: i32,
}

const WIDTH: u32 = 255;
const HEIGHT: u32 = 255;

const COLOR: u8 = 200;

fn main() {
    // ImageBuffer is static, while DynamicImage is dynamic
    // new variable canvas, mutable, type RgbImage, equal to a new ImageBuffer of WIDTH and HEIGHT
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let sw = Stopwatch::start_new();
    // do something that takes some time

    let point_0 = Point { x: 0, y: 0 };

    // for x in 0..100 {
    //     // draw(&mut canvas);
    //     let point_1 = Point { x: 2 as i32, y: 50 };
    //     draw_line(&mut canvas, &point_0, &point_1);
    // }
    draw(&mut canvas);
    println!("{} ms", sw.elapsed_ms());
    println!("saving");
    canvas.save("output.bmp").unwrap();
    println!("and saved");

    // let mut result = factorial(10);
    // let mut result = 0;
    // println!("{}", result);
    // let 🦀 = 12;
    let sw = Stopwatch::start_new();
    // println!("{}", result);
    println!("{} ms", sw.elapsed_ms());
}


// fn interpolate(i0: u32, d0: u32, i1: u32, d1: u32) {
//     let a = (d1 - d0) / (i1 - i0);
//     let mut d = d0;
//     for _i in i0..i1 {
//         println!("{}", d);
//         d += a;
//     }
// }
// fn draw_line(canvas: &mut RgbImage, p0: &Point, p1: &Point) {
//     let mut p0 = p0;
//     let mut p1 = p1;
//     if p0.x > p1.x {
//         // println!("called");
//         // let (p1, p0) = (p0, p1);

//         let temp = p0;
//         p0 = p1;
//         p1 = temp;
//     }
//     canvas.put_pixel(p0, p1, Rgb([255, 225, 255]));
// }
// fn box_blur(canvas: &RgbImage, size: u32) -> RgbImage {
//     let mut blurred_canvas: RgbImage = canvas.clone();
//     // loop over every pixel, then loop over it's effect
//     for pixel in canvas.enumerate_pixels(){
//         blurred_canvas.put_pixel(pixel.0, pixel.1, pixel.2);
//     }
//     blurred_canvas
//
// }
fn draw(canvas: &mut RgbImage) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // println!("{}",(y ;

            canvas.put_pixel(x, y, Rgb([x as u8, y as u8, ((x+y)/2) as u8]));
            // canvas.put_pixel(x, y, Rgb([(y as f64/8.0) as u8, 255, (x as f64/8.0) as u8]));
        }
    }
}
