use image::{Rgb, RgbImage};

use crate::geometry::primitives::{Point, Triangle};

fn plot_line_low(canvas: &mut RgbImage, p0: &Point, p1: &Point, color: Rgb<u8>) {
    let dx = p1.x - p0.x; // difference in x
    let mut dy = p1.y - p0.y; // difference in y
    let mut yi = 1;
    if dy < 0 {
        // if end point is to the left of start point
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = p0.y;

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
    let dy = p1.y - p0.y; // difference in y
    let mut xi = 1;
    if dx < 0 {
        // if end point is to the left of start point
        xi = -1;
        dx = -dx;
    }
    let mut d = (2 * dx) - dy;
    let mut x = p0.x;

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

pub fn _plot_line_naive(canvas: &mut RgbImage, p0: &Point, p1: &Point, color: Rgb<u8>) {
    // naive implementation
    let start_x = p0.x as f32;
    let start_y = p0.y as f32;
    let end_x = p1.x as f32;
    let end_y = p1.y as f32;

    let m: f32;

    if (end_x - start_x) <= 0.0 {
        m = 1_000_000.0;
    } else {
        m = (end_y - start_y) / (end_x - start_x);
    }
    let b = start_y as f32 - (m * start_x as f32);

    for x in p0.x..p1.x + 1 {
        let y = (m * x as f32 + b).round();
        canvas.put_pixel(x as u32, y as u32, color);
    }
}

pub fn plot_triangle(triangle: Triangle, canvas: &mut RgbImage, color: Rgb<u8>) {
    let a: &Point = &triangle.a;
    let b: &Point = &triangle.b;
    let c: &Point = &triangle.c;

    #[cfg(debug_assertions)]
    assert!(a.x as u32 <= canvas.width());

    #[cfg(debug_assertions)]
    assert!(a.y as u32 <= canvas.height());

    #[cfg(debug_assertions)]
    assert!(b.x as u32 <= canvas.width());

    #[cfg(debug_assertions)]
    assert!(b.y as u32 <= canvas.height());

    #[cfg(debug_assertions)]
    assert!(c.x as u32 <= canvas.width());

    #[cfg(debug_assertions)]
    assert!(c.y as u32 <= canvas.height());

    plot_line(canvas, a, b, color);
    plot_line(canvas, b, c, color);
    plot_line(canvas, c, a, color);
}
