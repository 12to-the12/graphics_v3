use image::{ImageBuffer, Rgb, RgbImage};

extern crate stopwatch;
use stopwatch::Stopwatch;

// const WIDTH: u32 = 3840;
// const HEIGHT: u32 = 2160;

// const WIDTH: u32 = 1920;
// const HEIGHT: u32 = 1080;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;

// const WIDTH: u32 = 100;
// const HEIGHT: u32 = 100;

const COLOR: u8 = 0;

// fn main() {
//     let mut count = 0;
//     let sw = Stopwatch::start_new();
//     loop {
//         count += 1;
//         if sw.elapsed_ms() > 1000 {
//             break;
//         }
//     }
//     println!("{}", count);
// }
fn main() {
    // ImageBuffer is static, while DynamicImage is dynamic
    // new variable canvas, mutable, type RgbImage, equal to a new ImageBuffer of WIDTH and HEIGHT
    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    let sw = Stopwatch::start_new();
    // do something that takes some time
    let frames = 100;
    for _x in 0..frames {
        draw(&mut canvas);
    }
    let ms = sw.elapsed_ms() as f64;
    let seconds = ms / 1000.0;
    let fps = frames as f64 / seconds;
    let pixels = (WIDTH as i64) * (HEIGHT as i64) * frames;
    let mega = 1.0 / 1_000_000.0;
    println!("got {}ms per frame", ms / frames as f64);
    println!("got {} frames per second", fps);
    println!(
        "got {} megapixels through per second",
        pixels as f64 / seconds * mega
    );
    println!(
        "or {} megabytes through per second",
        pixels as f64 / seconds * mega * 24.0
    );

    // println!("120 fps is {}ms per frame", 1000.0/120.0);
    println!("60 fps is {}ms per frame", 1000.0 / 60.0);
    println!("30 fps is {}ms per frame", 1000.0 / 30.0);
    // println!("15 fps is {}ms per frame", 1000.0/15.0);
    canvas.save("output.png").unwrap();
}

fn draw(canvas: &mut RgbImage) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // println!("{}",(y ;
            canvas.put_pixel(x, y, Rgb([COLOR, COLOR, COLOR]));
            // canvas.put_pixel(x, y, Rgb([(y as f64/8.0) as u8, 255, (x as f64/8.0) as u8]));
        }
    }
}
