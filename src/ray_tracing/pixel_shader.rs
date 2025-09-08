use crate::color::colorspace_conversion::spectra_to_display;
use crate::geometry::primitives::{polygon, ray, vector, Ray, Vector};
use crate::lighting::{black_spectra, Light, LightType, RadiometricUnit, Spectra};
use crate::material::ShaderNode;
use crate::object::{Object, OBJECT};
use crate::ray_tracing::ray_polygon_intersection::probe_ray_polygon_intersection;

use crate::ray_tracing::rendering_equation::BRDF;
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
            // for personal canvas
            // canvas.put_pixel(x as u32, y as u32, color);
            // canvas
            // .save_with_format("rust-output.bmp", ImageFormat::Bmp)
            // .unwrap();
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
    if scene.logging > 1 {
        // println!("  shading: {:?}", shading.elapsed());
    }
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
                let polygon = polygon(a, b, c);
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
    let ray = pixel_to_ray(x, y, scene);
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
    let ray = pixel_to_ray(x, y, scene);
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
    'lights: for light in scene.lights.clone() {
        // our job here is to find the amount of energy transmitted to the pixel from the light
        let light = match light {
            LightType::PointLight(light) => light,
        };
        let to_light = &intersection_point.clone().to(light.position);

        let occlusion_ray = ray(intersection_point, to_light.clone());
        // if (occlusion_ray.position.x < 3.) {
        //     println!("{:?}", occlusion_ray);
        // }

        for objects in scene.objects.iter() {
            if objects.ray_intercept(&occlusion_ray) {
                let occlusion = shoot_ray(occlusion_ray.clone(), &scene);
                if occlusion.is_some() {
                    // continue
                    output = output + black_spectra(RadiometricUnit::Flux);
                    continue 'lights;
                    // return monochroma_spectra(700., 100., RadiometricUnit::Flux);
                }
                // else{
                //     println!("{:?}",occlusion_ray);
                // }
            }
        }

        let _distance_to_surface: f32 = direction.magnitude();

        // if the angle between the surface and light is obtuse, it's facing away
        if to_light.dot(&normal) < 0. {
            // return monochroma_spectra(550., 1., RadiometricUnit::Flux)
            continue;
        }

        let closest_material: &ShaderNode = &closest_object.material;
        let radiance: Spectra = match closest_material {
            ShaderNode::Void => black_spectra(crate::lighting::RadiometricUnit::Radiance),
            ShaderNode::PBR(pbr) => pbr.rendering_equation(
                &intersection_point,
                to_light,
                &direction,
                &normal,
                light.radiant_intensity(intersection_point),
            ),
            ShaderNode::_Literal(spectra) => spectra.clone(),
        };

        output = output + radiance;
    }
    output
}

pub fn shoot_ray(ray: Ray, scene: &Scene) -> Option<(Object, Vector, Vector, Vector)> {
    let mut hit = false;
    let mut closest_dist: f32 = 1e6;
    let mut closest_object: &Object = &OBJECT;
    let mut surface_normal: Vector;
    surface_normal = vector(1., 1., 1.);
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
                let polygon = polygon(a, b, c);
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
        let to_camera = -1.*direction;
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
    let position = scene.camera.position;
    let mut ray = ray(position, direction);
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

        // println!("direction: {:?}", ray.direction);

        let mut foil = vector(-0.3333333, -0.3333333, -0.5);
        foil.norm();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //

        scene.camera.sensor.horizontal_res = 1;
        scene.camera.sensor.vertical_res = 1;
        ray = pixel_to_ray(0, 0, &scene);

        // println!("direction: {:?}", ray.direction);

        let mut foil = vector(00.0, 0.0, -0.5);
        foil.norm();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //
    }
}
