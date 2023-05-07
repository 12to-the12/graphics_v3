use image::{ImageBuffer, Rgb, RgbImage};

extern crate stopwatch;
use stopwatch::{Stopwatch};

const WIDTH: u32 = 255;
const HEIGHT: u32 = WIDTH;

fn main() {
    // ImageBuffer is static, while DynamicImage is dynamic
    // new variable canvas, mutable, type RgbImage, equal to a new ImageBuffer of WIDTH and HEIGHT
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    
    
    let sw = Stopwatch::start_new();
    // do something that takes some time
    
    println!("Thing took {}ms", sw.elapsed_ms());
    draw(&mut canvas);
    canvas.save("output.png").unwrap();
}

fn draw(canvas: &mut RgbImage) {
    // put a pixel on canvas
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // println!("{}", x);
            // println!("{}", y);
            let r = 255;
            let g = 1 * (x as u8);
            let b = 1 * (y as u8);
            let colors = [r, g, b];
            // println!("{}", r);
            // println!("{}", g);
            // println!("{}", b);
            // println!("end");
            canvas.put_pixel(x, y, Rgb(colors));
        }
    }
    // canvas.put_pixel(0, 0, Rgb([0, 0, 0]));
    // canvas.put_pixel(0, 1, Rgb([0, 0, 255]));
    // canvas.put_pixel(0, 2, Rgb([0, 255, 0]));
    // canvas.put_pixel(1, 0, Rgb([0, 255, 255]));
    // canvas.put_pixel(1, 1, Rgb([255, 0, 0]));
    // canvas.put_pixel(1, 2, Rgb([255, 0, 255]));
    // canvas.put_pixel(2, 0, Rgb([255, 255, 0]));
    // canvas.put_pixel(2, 1, Rgb([255, 255, 255]));
    // canvas.put_pixel(2, 2, Rgb([0, 0, 0]));
}
