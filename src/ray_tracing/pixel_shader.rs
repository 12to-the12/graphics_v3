use crate::color::colorspace_conversion::spectra_to_display;
use crate::geometry::primitives::{polygon, ray, vector, Ray, Vector};
use crate::lighting::{black_spectra, Spectra};
use crate::ray_tracing::path_tracing::{
    _ray_polygon_intersection_test, probe_ray_polygon_intersection,
};
use crate::ray_tracing::rendering_equation::white_matte_equation;

use crate::scene::Scene;
use image::{Rgb, RgbImage};
use stopwatch::Stopwatch;

pub fn shade_pixels<F: Fn(u32, u32, &Scene) -> Rgb<u8>>(
    mini_canvas: &mut RgbImage,
    scene: &Scene,
    closure: F,
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
) {
    let mut shading = Stopwatch::start_new();
    // println!("{}", scene.camera.sensor.horizontal_res);
    // let (width, height) = canvas.dimensions();
    let _width = y_end - y_start;
    let _height = x_end - x_start;
    for y in y_start..y_end {
        for x in x_start..x_end {
            let color = closure(x, y, scene);
            mini_canvas.put_pixel(x - x_start as u32, y - y_start as u32, color);
            // for personal canvas
            // canvas.put_pixel(x as u32, y as u32, color);
            // canvas
            // .save_with_format("rust-output.bmp", ImageFormat::Bmp)
            // .unwrap();
            // println!("px");
        }
    }

    // for y in 0..height {
    //     for x in 0..width {
    //         let color = closure(x, y, scene);
    //         canvas.put_pixel(x as u32, y as u32, color);
    //         // canvas
    //         // .save_with_format("rust-output.bmp", ImageFormat::Bmp)
    //         // .unwrap();
    //         // println!("px");
    //     }
    // }
    shading.stop();
    // println!("  shading: {:?}", shading.elapsed());
}

pub fn _color_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let (hres, vres) = scene.camera.sensor.res();
    let x = x as f32 / (hres as f32);
    let y = y as f32 / (vres as f32);
    let r1 = y * 255.;
    let g1 = 0.;
    let b1 = 0.;

    let r2 = 0.;
    let g2 = y * 255.;
    let b2 = 200.;
    if x < 0.3 {
        return Rgb([r1 as u8, g1 as u8, b1 as u8]);
    }
    if x > 0.6 {
        return Rgb([r2 as u8, g2 as u8, b2 as u8]);
    }
    if y < 0.5 {
        let r3 = ((r1 + r2) / 2.) as u8;
        let g3 = ((g1 + g2) / 2.) as u8;
        let b3 = ((b1 + b2) / 2.) as u8;
        return Rgb([r3, g3, b3]);
    }

    return Rgb([r1 as u8, g1 as u8, b1 as u8]);
}

pub fn _solid_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = pixel_to_ray(x, y, scene);
    // let mut hit = false;
    '_mesh: for mesh in scene.meshes.iter() {
        '_poly: for poly in &mesh.polygons {
            // println!("{:?}\n\n\n", mesh.output_vertices);
            let a = mesh.output_vertices[poly[0]].clone();
            let b = mesh.output_vertices[poly[1]].clone();
            let c = mesh.output_vertices[poly[2]].clone();
            let polygon = polygon(a, b, c);
            if _ray_polygon_intersection_test(&ray, &polygon) {
                // hit = true;
                // break 'mesh;
                return Rgb([255, 255, 255]);
            }
            // } else {
            //     return Rgb([0, 0, 0]);
            // }
            // return ray_polygon_intersection_test(&ray, &polygon);
        }
    }
    return Rgb([0, 0, 0]);
    // if hit {
    //     return Rgb([255, 255, 255]);
    // } else {
    //     return Rgb([0, 0, 0]);
    // } // Rgb([x, y, y])
}

pub fn lit_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = pixel_to_ray(x, y, scene);
    let mut hit = false;
    let mut closest: f32 = 1e6;
    let mut surface_normal: Vector;
    surface_normal = vector(1., 1., 1.);
    for mesh in scene.meshes.clone() {
        for poly in mesh.polygons {
            // println!("{:?}\n\n\n", mesh.output_vertices);
            let a = mesh.output_vertices[poly[0]].clone();
            let b = mesh.output_vertices[poly[1]].clone();
            let c = mesh.output_vertices[poly[2]].clone();
            let polygon = polygon(a, b, c);
            let (b, _i, dist) = probe_ray_polygon_intersection(&ray, &polygon);
            if b && dist < closest {
                closest = dist;
                hit = true;
                surface_normal = polygon.get_normal();
                if surface_normal.dot(&ray.direction) > 0. {
                    println!("polygon is facing away! This shouldn't register as an intersection!");
                    panic!();
                }
            }
            // } else {
            //     return Rgb([0, 0, 0]);
            // }
            // return ray_polygon_intersection_test(&ray, &polygon);
        }
    }

    if hit {
        let mut direction: Vector = ray.direction.clone();
        direction.norm();
        let intersection_point: Vector = (closest * direction.clone()) + ray.position;

        let mut output: Spectra = black_spectra();
        for light in scene.lights.clone() {
            // our job here is to find the amount of energy transmitted to the pixel from the light

            let to_light = &intersection_point.clone().to(light.position.as_vector());

            let _distance_to_surface: f32 = closest;
            // let _distance_to_light: f32 = to_light.magnitude();

            // let radiance = rendering_equation(
            //     &intersection_point,
            //     to_light,
            //     &direction,
            //     &surface_normal,
            //     light.radiant_flux,
            //     diffuse_white,
            //     no_emission,
            //     normal_incoming_spectral_radiance,
            // );

            let radiance = white_matte_equation(
                &intersection_point,
                to_light,
                &direction,
                &surface_normal,
                light.radiant_flux,
            );

            // let radiance = white_emission_equation(
            //     &intersection_point,
            //     to_light,
            //     &direction,
            //     &surface_normal,
            //     light.radiant_flux,
            // );

            // let θ = (irradiance).acos().to_degrees();

            // let mut brightness = θ; // [0 -> 180]
            // brightness -= 90.;

            // if brightness < 0. {
            //     brightness = 0.;
            // }
            // brightness /= 90.;
            // // 1 should map to 180° and 0 should be anything below 90°
            // // brightness *= 3.;

            // let mut brightness = radiance.from_λ(555.);
            // let mut brightness = spectra_to_sRGB(&radiance);
            // brightness *= 255.;

            // r += brightness;
            // g += brightness;
            // b += brightness;

            // println!("{:?}\n\n", radiance.luminance());
            if radiance.total() > 0. {
                // println!("{:?}", radiance.total());
                // println!("{:?}", spectra_to_sRGB(&radiance));
                output = output + radiance;
            }
        }
        return spectra_to_display(&output);
        // let r = r as u8;
        // let g = g as u8;
        // let b = b as u8;
        // return Rgb([r, g, b]);
    } else {
        // return Rgb([0, 0, 0]);
        return scene.background;
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
