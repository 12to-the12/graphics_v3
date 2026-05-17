use crate::camera::{Camera, Lens, Sensor};
use crate::geometry::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::{Mesh, Vector};
use crate::object::Object;
// use crate::primitives::Object;
use crate::lighting::{incandescent_spectra, PointLight};
use crate::load_object_file::load_wavefront_obj;
use crate::scene::scene::{Rendermode, Scene};
pub fn cornell_scene<'b>() -> Scene {
    let mut scene = Scene::default();
    let lens = Lens {
        aperture: 50.,
        focal_length: 80. / 1000.,
        focus_distance: 2.,
        ..Lens::default()
    };
    let sensor = Sensor {
        width: 36.0 / 1.000,
        horizontal_res: 240 * 2,
        vertical_res: 240 * 2,
        ..Sensor::default()
    };
    let camera = Camera {
        position: Vector::new(0., 2.74, 13.),
        exposure_time: 1e14,
        lens,
        sensor,
        ..Camera::default()
    };
    let key = scene.insert(camera);
    scene.set_active_camera(key);
    let light = PointLight::new(
        Vector::new(0.0, 5.0, -0.),
        // Vector::new(0.0, 3.0, -0.5),
        RIGHT,
        incandescent_spectra(2500., 1000.),
    );
    scene.push_simple_light(light);

    let cornell = load_wavefront_obj("models/cornell.obj".to_string());

    let object = Object {
        position: Vector::new(2.8, 0., 0.),
        meshes: vec![cornell.clone()],
        ..Object::default()
    };
    scene.push_object(object);

    scene.samples = 16;
    scene.max_trace_depth = 4;
    scene.max_render_dist = 20.;
    scene
}

pub fn simple_scene<'b>() -> Scene {
    let mut scene = Scene::default();
    scene.active_camera_mut().exposure_time = 1e16;
    scene.active_camera_mut().lens.aperture = 50.;
    scene.active_camera_mut().lens.focal_length = 20.0 / 1000.; // 120.
    scene.active_camera_mut().lens.focus_distance = 2.; // 120.

    scene.active_camera_mut().lens.aperture = 50.;
    scene.active_camera_mut().sensor.width = 36.0 / 1000.; // 36 mm
    scene.active_camera_mut().sensor.horizontal_res = 240 * 4;
    scene.active_camera_mut().sensor.vertical_res = 160 * 4;

    scene.active_camera_mut().position = Vector::new(0.0, 0.0, 7.);
    let mut light = PointLight::default();
    light.position = Vector::new(-5.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(1000., 1000.);
    scene.push_simple_light(light);

    let mut light = PointLight::default();
    light.position = Vector::new(0.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(3000., 1000.);
    scene.push_simple_light(light);

    let mut light = PointLight::default();
    light.position = Vector::new(5.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(5000., 1000.);
    scene.push_simple_light(light);

    let cube = load_wavefront_obj("models/cube.obj".to_string());
    let sphere: Mesh = load_wavefront_obj("models/sphere.obj".to_string());
    let plane: Mesh = load_wavefront_obj("models/plane.obj".to_string());
    let _wall: Mesh = load_wavefront_obj("models/wall.obj".to_string());
    let _cornell: Mesh = load_wavefront_obj("models/cornell.obj".to_string());

    let object = Object {
        position: Vector::new(3.0, 0.0, 0.),
        meshes: vec![cube.clone()],
        ..Object::default()
    };
    scene.push_object(object);
    let object = Object {
        position: Vector::new(-3.0, 0.0, 0.),
        meshes: vec![cube.clone()],
        ..Object::default()
    };
    scene.push_object(object);
    let object = Object {
        position: Vector::new(0.0, 0.0, 0.),
        meshes: vec![sphere.clone()],
        ..Object::default()
    };
    scene.push_object(object);
    let object = Object {
        position: Vector::new(0.0, -2.0, 0.),
        meshes: vec![plane.clone()],
        ..Object::default()
    };
    scene.push_object(object);

    scene.rendermode = Rendermode::ThreadedRayTrace;
    scene.samples = 16;
    scene.max_trace_depth = 5;
    scene.max_render_dist = 20.;
    scene.tilesize = 10;
    // @8 samples  10: 13.0 20: 4.10 40: 3.1 80: 2.87
    // @32 10: 17.22 20: 12.9 40: 13.5 80: 13.8
    // @64 samples  10: 29.2 20: 27.6 40: 28.43

    // with batched disk writes
    // @8 samples  10: 3.2 20: 2.789 40: 2.83 80: 3.39
    // @16 10: 5.83 20: 5.91 40: 6.25 80: 6.13
    scene.hue_timer = false;
    scene
}

// pub fn calibration_scene<'b>() -> Scene {
//     let mut scene = Scene::default();
//     scene.active_camera.exposure_time = 1e12;
//     scene.active_camera.lens.focal_length = 1.0 / 1000.; // 120.
//     scene.active_camera.lens.focus_distance = 2.; // 120.
//     scene.active_camera.lens.aperture = 50.;
//     scene.active_camera.sensor.width = 4.0 / 1000.; // 36 mm
//     scene.active_camera.sensor.horizontal_res = 100;
//     scene.active_camera.sensor.vertical_res = 100;
//     scene.active_camera.position = Vector::new(0.0, 0.0, 1.);

//     let light = PointLight::new(
//         Vector::new(0.0, 0.0, 10.0),
//         RIGHT,
//         const_spectra(1000.).into(),
//     );
//     scene.push_simple_light(light);

//     let cube = load_wavefront_obj("models/cube.obj".to_string());
//     let object = Object {
//         position: Vector::new(0.0, 0.0, -1.),
//         meshes: vec![cube.clone()],
//         ..Object::default()
//     };
//     scene.push_object(object);

//     let _background = black_spectra();

//     scene.samples = 64;
//     scene.max_trace_depth = 0;
//     scene
// }
