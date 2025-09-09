use crate::camera::Camera;
use crate::color::colorspace_conversion::spectra_to_display;
use crate::geometry::primitives::{Polygon, Ray, Vector};
use crate::lighting::{black_spectra, RadiometricUnit, Spectra};
use crate::object::Object;
use crate::ray_tracing::ray_polygon_intersection::probe_ray_polygon_intersection;

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
    // let (width, height) = canvas.dimensions();
    let _width = y_end - y_start;
    let _height = x_end - x_start;
    for y in y_start..y_end {
        for x in x_start..x_end {
            let color = closure(x, y, scene);
            mini_canvas.put_pixel(x - x_start as u32, y - y_start as u32, color);
        }
    }

    shading.stop();
    if scene.logging > 1 {}
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
    let ray = Camera::pixel_to_ray(&scene.camera, x, y);
    let mut hit = false;
    // here we're at once per pixel
    for object in &scene.objects {
        // this is once per object
        if scene.spatial_acceleration_structures && !object.ray_intercept(&ray) {
            continue;
        }
        for mesh in &object.meshes {
            // once per mesh
            for poly in mesh.polygons.clone() {
                let a = mesh.output_vertices[poly[0]].clone();
                let b = mesh.output_vertices[poly[1]].clone();
                let c = mesh.output_vertices[poly[2]].clone();
                let polygon = Polygon::new(a, b, c);
                let (b, _i, _dist) = probe_ray_polygon_intersection(&ray, &polygon);
                if b {
                    hit = true;
                    break;
                }
            }
        }
    }
    if hit {
        return Rgb([255, 255, 255]);
    } else {
        return spectra_to_display(&scene.background);
    }
}

pub fn bvh_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = Camera::pixel_to_ray(&scene.camera, x, y);
    let mut hit = false;
    // here we're at once per pixel
    for object in &scene.objects {
        // this is once per object
        if object.ray_intercept(&ray) {
            hit = true;
            break;
        }
    }
    if hit {
        return Rgb([255, 255, 255]);
    } else {
        return spectra_to_display(&scene.background);
    }
}
pub fn lit_shader(x: u32, y: u32, scene: &Scene) -> Rgb<u8> {
    let ray = Camera::pixel_to_ray(&scene.camera, x, y);
    let output: Spectra = match shoot_ray(ray, scene) {
        None => black_spectra(RadiometricUnit::Flux),
        Some((object, x, ω_o, normal)) => {
            let output = compute_light(scene, object, x, ω_o, normal);
            scene.camera.exposure_time * scene.camera.sensor._pixel_area() * output
        }
    };

    return spectra_to_display(&output);
}

pub fn compute_light(
    scene: &Scene,
    closest_object: Object,
    intersection_point: Vector,
    direction: Vector,
    normal: Vector,
) -> Spectra {
    let mut output = black_spectra(RadiometricUnit::Flux);
    'lights: for light in &scene.lights {
        // our job here is to find the amount of energy transmitted to the pixel from the light
        let to_light = &intersection_point.clone().to(light.get_position());

        let occlusion_ray = Ray::new(intersection_point, to_light.clone());

        for objects in scene.objects.iter() {
            if objects.ray_intercept(&occlusion_ray) {
                let occlusion = shoot_ray(occlusion_ray.clone(), &scene);
                if occlusion.is_some() {
                    output = output + black_spectra(RadiometricUnit::Flux);
                    continue 'lights;
                }
            }
        }

        let _distance_to_surface: f32 = direction.magnitude();

        // if the angle between the surface and light is obtuse, it's facing away
        if to_light.dot(&normal) < 0. {
            continue;
        }

        let radiance = closest_object.material.rendering_equation(
            &intersection_point,
            to_light,
            &direction,
            &normal,
            light.radiant_intensity(intersection_point),
        );

        output = output + radiance;
    }
    output
}

pub fn shoot_ray(ray: Ray, scene: &Scene) -> Option<(Object, Vector, Vector, Vector)> {
    let mut hit = false;
    let mut closest_dist: f32 = 1e6;
    let mut closest_object: &Object = &Object::default();
    let mut surface_normal: Vector;
    surface_normal = Vector::new(1., 1., 1.);
    // here we're at once per pixel
    for object in &scene.objects {
        // this is once per object
        if scene.spatial_acceleration_structures && !object.ray_intercept(&ray) {
            continue;
        }
        for mesh in &object.meshes {
            // once per mesh
            for poly in mesh.polygons.clone() {
                let a = mesh.output_vertices[poly[0]].clone();
                let b = mesh.output_vertices[poly[1]].clone();
                let c = mesh.output_vertices[poly[2]].clone();
                let polygon = Polygon::new(a, b, c);
                let (b, _i, dist) = probe_ray_polygon_intersection(&ray, &polygon);
                if b && dist < closest_dist {
                    closest_object = object;
                    closest_dist = dist;
                    hit = true;
                    surface_normal = polygon.get_normal();
                    if surface_normal.dot(&ray.direction) > 0. {
                        println!(
                            "polygon is facing away! This shouldn't register as an intersection!"
                        );
                        panic!();
                    }
                }
            }
        }
    }

    if hit {
        let mut direction: Vector = ray.direction.clone();
        direction.norm();
        direction = closest_dist * direction; // explicitly not a unit vector
        let mut intersection_point: Vector = (direction.clone()) + ray.position;
        let to_camera = -1. * direction;
        // to prevent shader acne
        let mut offset = surface_normal.clone();
        offset.norm();
        offset = 1e-5 * offset;
        intersection_point = intersection_point + offset;

        return Some((
            closest_object.clone(),
            intersection_point,
            to_camera,
            surface_normal,
        ));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::{camera::Camera, scene::simple_scene};

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
        ray = Camera::pixel_to_ray(&scene.camera, 0, 2);

        // println!("direction: {:?}", ray.direction);

        let mut foil = Vector::new(-0.3333333, -0.3333333, -0.5);
        foil.norm();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //

        scene.camera.sensor.horizontal_res = 1;
        scene.camera.sensor.vertical_res = 1;
        ray = Camera::pixel_to_ray(&scene.camera, 0, 0);

        // println!("direction: {:?}", ray.direction);

        let mut foil = Vector::new(00.0, 0.0, -0.5);
        foil.norm();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //
    }
}
