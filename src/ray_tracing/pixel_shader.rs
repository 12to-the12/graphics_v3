use std::f32::consts::PI;

use crate::camera::Camera;
use crate::color::colorspace_conversion::{sRGB_to_display, spectra_to_display};
use crate::geometry::primitives::{even_over_hemisphere, Polygon, Ray, Vector};
use crate::lighting::{black_spectra, void_spectra, Radiance};
use crate::object::Object;
use crate::ray_tracing::ray_polygon_intersection::probe_ray_polygon_intersection;

use crate::geometry_pipeline::Tile;
use crate::scene::Scene;
use image::Rgb;
use rand::{prelude::ThreadRng, thread_rng};
use stopwatch::Stopwatch;

/// applies rendering mode to scene and yields a canvas
pub fn shade_pixels<F: Fn(u32, u32, &Scene, &mut ThreadRng) -> Rgb<u8>>(
    tile: &mut Tile,
    scene: &Scene,
    closure: F,
) {
    let mini_canvas = &mut tile.canvas;
    let x_start = tile.x_start;
    let x_end = tile.x_start + tile.width;
    let y_start = tile.y_start;
    let y_end = tile.y_start + tile.height;

    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let mut shading = Stopwatch::start_new();
    // let (width, height) = canvas.dimensions();
    let _width = y_end - y_start;
    let _height = x_end - x_start;
    for y in y_start..y_end {
        for x in x_start..x_end {
            let color = closure(x, y, scene, &mut rng);
            mini_canvas.put_pixel(x - x_start, y - y_start, color);
        }
    }

    shading.stop();
    if scene.logging > 1 {}
}

pub fn _color_shader(x: u32, y: u32, scene: &Scene, _rng: &mut ThreadRng) -> Rgb<u8> {
    let (hres, vres) = scene.active_camera.sensor.res();
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
        let r3 = f32::midpoint(r1, r2) as u8;
        let g3 = f32::midpoint(g1, g2) as u8;
        let b3 = f64::midpoint(b1, b2) as u8;
        return Rgb([r3, g3, b3]);
    }

    Rgb([r1 as u8, g1 as u8, b1 as u8])
}

/// shades all objects as solid
pub fn _solid_shader(x: u32, y: u32, scene: &Scene, rng: &mut ThreadRng) -> Rgb<u8> {
    let ray = Camera::pixel_to_ray(&scene.active_camera, x, y, rng);
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
        Rgb([255, 255, 255])
    } else {
        spectra_to_display(&scene.background)
    }
}

/// shows where bounding volume hierarchies are
pub fn bvh_shader(x: u32, y: u32, scene: &Scene, rng: &mut ThreadRng) -> Rgb<u8> {
    let ray = Camera::pixel_to_ray(&scene.active_camera, x, y, rng);
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
        Rgb([255, 255, 255])
    } else {
        spectra_to_display(&scene.background)
    }
}

/// render depth
pub fn z_shader(x: u32, y: u32, scene: &Scene, rng: &mut ThreadRng) -> Rgb<u8> {
    let ray = Camera::pixel_to_ray(&scene.active_camera, x, y, rng);
    let intersection = shoot_ray(ray, scene, scene.max_trace_depth);
    if intersection.is_none() {
        Rgb([0, 0, 0])
    } else {
        let (_, _, ω_o, _) = intersection.unwrap();
        let dist = ω_o.magnitude();
        let fractional_z = 1. - (dist / scene.max_render_dist);
        sRGB_to_display((fractional_z, fractional_z, fractional_z))
    }
}

/// fully lit shading mode
/// this lambda is executed once per pixel
pub fn lit_shader(x: u32, y: u32, scene: &Scene, rng: &mut ThreadRng) -> Rgb<u8> {
    let sample_average = integrate_pixel_radiance(x, y, scene, rng);
    // we should take a value parameterized in radiance, not radiant exitance!
    let joules = scene.active_camera.exposure_time
        * scene.active_camera.sensor._pixel_area()
        * scene.active_camera._pixel_solid_angle()
        * sample_average.0;

    spectra_to_display(&joules)
}
pub fn integrate_pixel_radiance(x: u32, y: u32, scene: &Scene, rng: &mut ThreadRng) -> Radiance {
    let mut radiance: Radiance = black_spectra().into();
    for _ in 0..scene.samples {
        let ray = Camera::pixel_to_ray(&scene.active_camera, x, y, rng);
        radiance.0 = radiance.0 + dispatch_light_ray(ray, scene, scene.max_trace_depth, rng).0;
    }

    (radiance.0 / scene.samples as f32).into()
}
pub fn integrate_indirect_surface_radiance(
    intersection_point: Vector,
    normal: Vector,
    scene: &Scene,
    trace_depth: u32,
    rng: &mut ThreadRng,
) -> Radiance {
    let mut radiance: Radiance = black_spectra().into();
    for _ in 0..1 {
        let ray = Ray::new(intersection_point, even_over_hemisphere(normal, rng));
        radiance.0 = radiance.0 + dispatch_light_ray(ray, scene, trace_depth, rng).0;
    }
    radiance
}

/// this is what is recursed
/// the ray is given
pub fn dispatch_light_ray(
    ray: Ray,
    scene: &Scene,
    trace_depth: u32,
    rng: &mut ThreadRng,
) -> Radiance {
    let intersection = shoot_ray(ray, scene, scene.max_trace_depth);
    if intersection.is_none() {
        return void_spectra().into();
        // return black_spectra().into();
    }
    let (object, intersection_point, ω_o, normal) = intersection.unwrap();

    // direct illumination
    // this is basically integrating incoming light to our point
    // we know the area subtended by this light source already, so we don't need multiple samples
    let direct_illumination: Radiance = integrate_direct_surface_radiance(
        scene,
        object,
        intersection_point,
        ω_o,
        normal,
        trace_depth,
    );
    // direct_illumination

    if scene.recursive_raycasting && trace_depth > 0 {
        // return Some((
        //     closest_object.clone(),
        //     intersection_point,
        //     to_camera,
        //     surface_normal,
        // ));

        let indirect_illumination = integrate_indirect_surface_radiance(
            intersection_point,
            normal,
            scene,
            trace_depth - 1,
            rng,
        );
        (direct_illumination.0 + indirect_illumination.0).into()
    } else {
        direct_illumination.0.into()
    }
}

/// shoots a ray to every light from our point to compute illumination
/// the reason this returns radiant exitance is because we know the size of the light sources
/// not proper recursive ray tracing
pub fn integrate_direct_surface_radiance(
    scene: &Scene,
    object: Object,
    intersection_point: Vector,
    direction: Vector,
    normal: Vector,
    _trace_depth: u32,
) -> Radiance {
    let mut output: Radiance = void_spectra().into();
    // let mut output: RadiantExitance = black_spectra().into();
    'lights: for light in &scene.lights {
        // our job here is to find the amount of energy transmitted to the pixel from the light
        let to_light = &intersection_point.clone().to(light.get_position());

        let occlusion_ray = Ray::new(intersection_point, *to_light);

        let occlusion = shoot_ray(occlusion_ray.clone(), scene, _trace_depth);
        if occlusion.is_some() {
            // this is where a recursive ray would begin
            continue 'lights;
        }

        let _distance_to_surface: f32 = direction.magnitude();

        // if the angle between the surface and light is obtuse, it's facing away
        if to_light.dot(&normal) < 0. {
            continue;
        }

        let radiance = object.material.rendering_equation(
            &intersection_point,
            to_light,
            &direction,
            &normal,
            //  light.radiant_intensity(intersection_point),
            light.radiant_intensity(intersection_point),
        );
        // SHIT STILL NEEDS TO BE DIVIDED BY DISTANCE!!!
        let r_i = to_light.magnitude(); // dist to light source
        let _r_o = direction.magnitude(); // dist to observer

        // this does not change the units
        // it simply makes it so that the one sample that we have
        // is treated like it's a 1 meter sphere instead of the entire hemisphere
        let area_subtended = 1. / (2. * PI * r_i * r_i);
        let radiance: Radiance = (area_subtended * radiance.0).into();

        output.0 = output.0 + radiance.0;
    }
    output
}

/// given a ray in the scene, see what it hits if anything
pub fn shoot_ray(ray: Ray, scene: &Scene, _depth: u32) -> Option<(Object, Vector, Vector, Vector)> {
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
        let mut direction: Vector = ray.direction;
        direction.unitize();
        direction = closest_dist * direction; // explicitly not a unit vector
        let mut intersection_point: Vector = direction + ray.position;
        let to_camera = -1. * direction;
        // to prevent shader acne
        let mut offset = surface_normal;
        offset.unitize();
        offset = 1e-5 * offset;
        intersection_point = intersection_point + offset;

        Some((
            closest_object.clone(),
            intersection_point,
            to_camera,
            surface_normal,
        ))
    } else {
        None
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
        let mut rng = thread_rng();

        scene.active_camera.lens.focal_length = 18.0;
        scene.active_camera.sensor.width = 36.0;

        scene.active_camera.sensor.horizontal_res = 3;
        scene.active_camera.sensor.vertical_res = 3;
        ray = Camera::pixel_to_ray(&scene.active_camera, 0, 2, &mut rng);

        // println!("direction: {:?}", ray.direction);

        let mut foil = Vector::new(-0.3333333, -0.3333333, -0.5);
        foil.unitize();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //

        scene.active_camera.sensor.horizontal_res = 1;
        scene.active_camera.sensor.vertical_res = 1;
        ray = Camera::pixel_to_ray(&scene.active_camera, 0, 0, &mut rng);

        // println!("direction: {:?}", ray.direction);

        let mut foil = Vector::new(00.0, 0.0, -0.5);
        foil.unitize();
        assert_eq!(ray.direction.x, foil.x); //
        assert_eq!(ray.direction.y, foil.y); //
        assert_eq!(ray.direction.z, foil.z); //
    }
}
