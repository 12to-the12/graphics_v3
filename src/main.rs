use image::{ImageBuffer, Rgb, RgbImage};

extern crate stopwatch;
use stopwatch::Stopwatch;

pub mod line_plotting;


// struct Vertex { // point in 3D space
//     x: i32, 
//     y: i32,
//     z: i32
// }

// struct Polygon {
//     a: Vertex,
//     b: Vertex,
//     c: Vertex
// }



const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

const COLOR: Rgb<u8> = Rgb([0, 255, 0]);

fn main() {



    println!("{}", 39.99 as u32);
    // ImageBuffer is static, while DynamicImage is dynamic
    // new variable canvas, mutable, type RgbImage, equal to a new ImageBuffer of WIDTH and HEIGHT
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    canvas.fill(30);

    // do something that takes some time

    let point_0 = line_plotting::Point { x: 0, y: 0 };
    // let point_1 = Point { x: 50, y: 20 };

    let sw = Stopwatch::start_new();
    // for x in 0..64 {
    //     let point_1 = line_plotting::Point { x: x * 16, y: 1024 };
    //     line_plotting::plot_line(&mut canvas, &point_0, &point_1, COLOR);
    // }
    // plot_line(&mut canvas, &point_0, &point_1, COLOR);
    // let point_1 = Point { x: 50, y: 100 };
    // plot_line(&mut canvas, &point_0, &point_1, COLOR);
    println!("{:?}", sw.elapsed());

    // draw(&mut canvas);
    println!("saving");
    canvas.save("../output.bmp").unwrap();
    println!("and saved");
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
