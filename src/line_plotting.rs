
use image::{Rgb, RgbImage};


pub struct Point {
    pub x: i32,
    pub y: i32,
}


fn plot_line_low(canvas: &mut RgbImage, p0: &Point, p1: &Point, color: Rgb<u8>) {
    let dx = p1.x - p0.x; // difference in x
    let mut dy= p1.y - p0.y; // difference in y
    let mut yi = 1;
    if dy < 0 {
        // if end point is to the left of start point
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = p0.y;
    // println!("{}, {}, {}, {}", p0.x, p0.y, p1.x, p1.y);
    for x in p0.x..p1.x {
        canvas.put_pixel(x as u32, y as u32, color);
        if d > 0 {
            y += yi;
            d = d + (2 * (dy - dx));
        } else {
            d = d + 2 * dy;
        }
    }
}

fn plot_line_high(canvas: &mut RgbImage, p0: &Point, p1: &Point, color: Rgb<u8>) {
    let mut dx = p1.x - p0.x; // difference in x
    let dy= p1.y - p0.y; // difference in y
    let mut xi = 1;
    if dx < 0 {
        // if end point is to the left of start point
        xi = -1;
        dx = -dx;
    }
    let mut d = (2 * dx) - dy;
    let mut x = p0.x;

    // println!("{}, {}, {}, {}", p0.x, p0.y, p1.x, p1.y);
    for y in p0.y..p1.y {
        canvas.put_pixel(x as u32, y as u32, color);
        if d > 0 {
            x += xi;
            d = d + (2 * (dx - dy));
        } else {
            d = d + 2 * dx;
        }
    }
}

pub fn plot_line(canvas: &mut RgbImage, p0: &Point, p1: &Point, color: Rgb<u8>) {
    if (p1.y - p0.y).abs() < (p1.x - p0.x).abs() {
        if p0.x > p1.x {
            plot_line_low(canvas, p1, p0, color);
        } else {
            plot_line_low(canvas, p0, p1, color);
        }
    } else {
        if p0.y > p1.y {
            plot_line_high(canvas, p1, p0, color);
        } else {
            plot_line_high(canvas, p0, p1, color);
        }
    }
}