use crate::line_plotting::plot_triangle;
use crate::primitives::{Point, Triangle};
use image::{Rgb, RgbImage};
/// solid color rasterization
pub fn rasterize_triangle(triangle: Triangle, canvas: &mut RgbImage) {
    let color = Rgb([0, 255, 0]);
    plot_triangle(triangle.clone(), canvas, color);
    // canvas.put_pixel(x as u32, y as u32, color);

    // get bounding box
    // sort points by Y from smallest,
    // if they're on the same line, draw the left one first
}

fn _sort_points(triangle: &Triangle) -> Vec<&Point> {
    let p0 = &triangle.a;
    let p1 = &triangle.b;
    let p2 = &triangle.c;
    let mut points = vec![p0, p1, p2];

    // 3 input naive sorting network
    // first and last
    if p2.y == p0.y {
        if p2.x < p0.x {
            points.swap(0, 1)
        }
    }
    if p2.y > p0.y {
        points.swap(0, 1)
    }

    // first two
    if p1.y == p0.y {
        if p1.x < p0.x {
            points.swap(0, 1)
        }
    }
    if p1.y > p0.y {
        points.swap(0, 1)
    }

    // last two
    if p2.y == p1.y {
        if p2.x < p1.x {
            points.swap(0, 1)
        }
    }
    if p2.y > p1.y {
        points.swap(0, 1)
    }
    return points;
}
