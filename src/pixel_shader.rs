use crate::camera::{Camera, Lens, Sensor};
use crate::line_plotting::plot_triangle;
use crate::path_tracing::ray_polygon_intersection_test;
use crate::primitives::{polygon, ray, vector, vertex, Point, Polygon, Ray, Triangle, Vector};
use crate::scene::Scene;
use image::{Rgb, RgbImage};
use ndarray::{arr1, arr2, Array1, Array2, Axis};
use rand::distributions::{Distribution, Uniform}; // 0.6.5

pub fn shade_pixels<F: Fn(u32, u32, &Scene) -> Rgb<u8>>(
    canvas: &mut RgbImage,
    scene: &Scene,
    closure: F,
) {
    println!("{}",scene.camera.sensor.horizontal_res);
    let (width, height) = canvas.dimensions();
    for y in 0..height {
        for x in 0..width {
            let color = closure(x, y, scene);
            canvas.put_pixel(x as u32, y as u32, color)
        }
    }
}

pub fn toy_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = pixel_to_ray(x, y, scene);
    for mesh in scene.meshes.clone() {
        for poly in mesh.polygons {
            // println!("{:?}\n\n\n", mesh.output_vertices);
            let a = mesh.output_vertices[poly[0]].clone();
            let b = mesh.output_vertices[poly[1]].clone();
            let c = mesh.output_vertices[poly[2]].clone();
            let polygon = polygon(a, b, c);
            // if ray_polygon_intersection_test(&ray, &polygon) {
            //     return Rgb([255, 0, 0]);
            // } else {
            //     return Rgb([0, 0, 0]);
            // }
            return ray_polygon_intersection_test(&ray, &polygon);
        }
    }
    return Rgb([0, 150, 0]); // only happens if there is no geometry to check against
                             // Rgb([x, y, y])
}

/// yeah, the math was hard for me too 2023-11-20
pub fn pixel_to_ray(x: u32, y: u32, scene: &Scene) -> Ray {
    let x = (x as f32) + 0.5; // centers the pixels
    let y = (y as f32) + 0.5;
    let camera = &scene.camera;
    let (hres, vres) = camera.sensor.res();
    let mut horizontal_fraction: f32 = x / (hres as f32);
    let mut vertical_fraction: f32 = y / (vres as f32);

    // let mut horizontal_fraction: f32 = x;
    // let mut vertical_fraction: f32 = y;

    horizontal_fraction -= 0.5; // [0 -> 1] becomes [-0.5 -> +0.5]
    vertical_fraction -= 0.5; // [0 -> 1] becomes [-0.5 -> +0.5]

    vertical_fraction *= -1.0; // because the coordinates are inverted
    vertical_fraction /= camera.sensor.aspect_ratio(); // because the z value is derived from the horizontal field of view, this can be proportional to width

    let direction = Vector {
        x: horizontal_fraction,
        y: vertical_fraction,
        z: camera.lens.focal_length / camera.sensor.width * -1.0,
        // z is negative, and if the ray placement is scaled to one from the sensor width,
        // the focal length needs to be proportional
    };

    let mut ray = ray(vector(0.0, 0.0, 0.0), direction);
    ray.direction.norm();
    ray
}

#[cfg(test)]
mod tests {
    use crate::scene::simple_scene;

    use super::*;

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn pixel_rays() {
        let mut scene = simple_scene();
        let mut ray: Ray;

        scene.camera.lens.focal_length = 18.0;
        scene.camera.sensor.width = 36.0;

        scene.camera.sensor.horizontal_res = 3;
        scene.camera.sensor.vertical_res = 3;
        ray = pixel_to_ray(0, 2, &scene);

        println!("direction: {:?}", ray.direction);
        assert_eq!(ray.direction.x, -0.3333333); //
        assert_eq!(ray.direction.y, -0.3333333); //
        assert_eq!(ray.direction.z, -0.5); //

        scene.camera.sensor.horizontal_res = 1;
        scene.camera.sensor.vertical_res = 1;
        ray = pixel_to_ray(0, 0, &scene);

        println!("direction: {:?}", ray.direction);
        assert_eq!(ray.direction.x, 0.0); //
        assert_eq!(ray.direction.y, 0.0); //
        assert_eq!(ray.direction.z, -0.5); //
    }
}
