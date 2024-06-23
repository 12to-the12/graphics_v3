use crate::camera::{Camera, Lens, Sensor};
use crate::line_plotting::plot_triangle;
use crate::path_tracing::{probe_ray_polygon_intersection, ray_polygon_intersection_test};
use crate::primitives::{polygon, ray, vector, vertex, Point, Polygon, Ray, Triangle, Vector};
use crate::scene::Scene;
use image::{ImageFormat, Rgb, RgbImage};
use ndarray::{arr1, arr2, Array1, Array2, Axis};
use rand::distributions::{Distribution, Uniform}; // 0.6.5
use stopwatch::Stopwatch;

pub fn shade_pixels<F: Fn(u32, u32, &Scene) -> Rgb<u8>>(
    canvas: &mut RgbImage,
    scene: &Scene,
    closure: F,
) {

    let mut shading = Stopwatch::start_new();
    // println!("{}", scene.camera.sensor.horizontal_res);
    let (width, height) = canvas.dimensions();
    for y in 0..height {
        for x in 0..width {
            let color = closure(x, y, scene);
            canvas.put_pixel(x as u32, y as u32, color);
            // canvas
            // .save_with_format("rust-output.bmp", ImageFormat::Bmp)
            // .unwrap();
            // println!("px");
        }
    }
    shading.stop();
    println!("  shading: {:?}", shading.elapsed());
    
}

pub fn color_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let (hres, vres) = scene.camera.sensor.res();
    let x = x as f32 / (hres as f32);
    let y = y as f32 / (vres as f32);
    let r1 = y*255.;
    let g1 = 0.;
    let b1 = 0.;

    let r2 = 0.;
    let g2 = y*255.;
    let b2 = 200.;
    if x < 0.3 {
        return Rgb([r1 as u8, g1 as u8, b1 as u8]);
    }
    if x > 0.6 {
        return Rgb([r2 as u8, g2 as u8, b2 as u8]);
    }
    if y < 0.5 {
        let r3 = ((r1+r2)/2.) as u8;
        let g3 = ((g1+g2)/2.) as u8;
        let b3 = ((b1+b2)/2.) as u8;
        return Rgb([r3,g3,b3]);
    }

    return Rgb([r1 as u8, g1 as u8, b1 as u8]);
}

pub fn solid_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = pixel_to_ray(x, y, scene);
    // let mut hit = false;
    'mesh: for mesh in scene.meshes.iter() {
        'poly: for poly in &mesh.polygons {
            // println!("{:?}\n\n\n", mesh.output_vertices);
            let a = mesh.output_vertices[poly[0]].clone();
            let b = mesh.output_vertices[poly[1]].clone();
            let c = mesh.output_vertices[poly[2]].clone();
            let polygon = polygon(a, b, c);
            if ray_polygon_intersection_test(&ray, &polygon) {
                // hit = true;
                // break 'mesh;
                return Rgb([255,255,255]);
            }
            // } else {
            //     return Rgb([0, 0, 0]);
            // }
            // return ray_polygon_intersection_test(&ray, &polygon);
        }
    }
    return Rgb([0,0,0]);
    // if hit {
    //     return Rgb([255, 255, 255]);
    // } else {
    //     return Rgb([0, 0, 0]);
    // } // Rgb([x, y, y])
}

pub fn lit_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = pixel_to_ray(x, y, scene);
    let mut hit = false;
    let mut closest = 1e6;
    let mut surface_normal: Vector;
    surface_normal = vector(1., 1., 1.);
    for mesh in scene.meshes.clone() {
        for poly in mesh.polygons {
            // println!("{:?}\n\n\n", mesh.output_vertices);
            let a = mesh.output_vertices[poly[0]].clone();
            let b = mesh.output_vertices[poly[1]].clone();
            let c = mesh.output_vertices[poly[2]].clone();
            let polygon = polygon(a, b, c);
            let (b, I, dist) = probe_ray_polygon_intersection(&ray, &polygon);
            if b && dist < closest {
                closest = dist;
                hit = true;
                surface_normal = polygon.get_normal();
            }
            // } else {
            //     return Rgb([0, 0, 0]);
            // }
            // return ray_polygon_intersection_test(&ray, &polygon);
        }
    }

    if hit {
        let mut r = 0.;
        let mut g = 0.;
        let mut b = 0.;

        for light in scene.lights.clone() {
            // let light_angle = vector(0., -1., 0.);
            // let θ = light_angle.dot(&surface_normal).acos().cos().to_degrees();
            let θ = light.direction.dot(&surface_normal);

            // if θ < -1. || θ > 1. {
            //     println!("{θ}");

            // }

            let mut mag = θ; // [0 -> 180]
            mag *= -1.;
            if mag < 0. {
                return Rgb([0, 0, 0]);
            }
            // [-1 -> 1] becomes 0 -> -1, where -1 is strong
            if mag > 0.9 {
                return Rgb([255, 0, 0]);
            }

            if mag < 0.1 {
                return Rgb([0, 0, 100]);
            }

            // mag /= 90.; // [0 -> 1]

            mag *= 255.;
            r += mag;
            g += mag;
            b += mag;
        }
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        return Rgb([r, g, b]);
    } else {
        return Rgb([0, 0, 0]);
        // return scene.background;
    } // Rgb([x, y, y])
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

        let mut foil = vector(-0.3333333, -0.3333333, -0.5);
        foil.norm();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //

        scene.camera.sensor.horizontal_res = 1;
        scene.camera.sensor.vertical_res = 1;
        ray = pixel_to_ray(0, 0, &scene);

        println!("direction: {:?}", ray.direction);

        let mut foil = vector(00.0, 0.0, -0.5);
        foil.norm();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //
    }
}
