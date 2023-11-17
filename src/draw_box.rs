
use image::{ImageBuffer, Rgb, RgbImage};

use crate::primitives::{Point, Vertex, Polygon};

use crate::line_plotting::plot_line;



pub fn draw_box() {

    // 90° lens foreshortens by half size with double distance
    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 30;
    const _HALF_WIDTH: u32 = WIDTH / 2;

    const COLOR: Rgb<u8> = Rgb([0, 255, 0]);

    let mut canvas: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    // let pointa = Point{x:10,y:10};
    // let pointb = Point{x:700,y:700};

    // plot_line(&mut canvas, &pointa, &pointb, COLOR); // canvas point point color

    

    let mut a  = Vertex{ x: 00.0, y:00.0, z:1.0};
    let mut b  = Vertex{ x: 29.0, y:00.0, z:1.0};
    let mut c  = Vertex{ x: 15.0, y:15.0, z:1.0};

    let mut mister_poly = Polygon{ a:a, b:b, c:c };

    mister_poly.draw(&mut canvas, COLOR);

    mister_poly.a.z = 2.0;
    mister_poly.b.z = 2.0;
    mister_poly.c.z = 2.0;
    
    mister_poly.draw(&mut canvas, COLOR);

    canvas.put_pixel(10, 29, COLOR);

    canvas.save("rust-output.png").unwrap();
    println!("image saved");


}
