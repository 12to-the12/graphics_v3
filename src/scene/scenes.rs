use std::sync::Arc;

use crate::camera::{Camera, Lens, Sensor};
use crate::geometry::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::{Mesh, Vector};
use crate::object::Object;
// use crate::primitives::Object;
use crate::lighting::{black_spectra, const_spectra, incandescent_spectra, Light, PointLight};
use crate::load_object_file::load_wavefront_obj;
use crate::scene::scene::{Rendermode, Scene, ShaderMode};
pub fn cornell_scene<'b>() -> Scene {
    let mut scene = Scene::default();
    scene.active_camera.lens.aperture = 50.;
    scene.active_camera.lens.focal_length = 80. / 1000.;
    scene.active_camera.lens.focus_distance = 2.;
    scene.active_camera.sensor.width = 36.0 / 1000.; // 36 mm
    scene.active_camera.sensor.horizontal_res = 240 * 2;
    scene.active_camera.sensor.vertical_res = 240 * 2;
    scene.active_camera.position = Vector::new(0., 2.74, 13.);
    scene.active_camera.exposure_time = 1e14;
    let light = PointLight::new(
        Vector::new(0.0, 5.0, -0.),
        // Vector::new(0.0, 3.0, -0.5),
        RIGHT,
        incandescent_spectra(2500., 1000.),
    );
    scene.simple_lights.push(Arc::new(light));

    let cornell = load_wavefront_obj("models/cornell.obj".to_string());

    let object = Object {
        position: Vector::new(2.8, 0., 0.),
        meshes: vec![cornell.clone()],
        ..Object::default()
    };
    scene.objects.push(object);

    scene.samples = 16;
    scene.max_trace_depth = 4;
    scene.max_render_dist = 20.;
    scene
}

pub fn simple_scene<'b>() -> Scene {
    let mut scene = Scene::default();
    scene.active_camera.exposure_time = 1e16;
    scene.active_camera.lens.aperture = 50.;
    scene.active_camera.lens.focal_length = 20.0 / 1000.; // 120.
    scene.active_camera.lens.focus_distance = 2.; // 120.

    scene.active_camera.lens.aperture = 50.;
    scene.active_camera.sensor.width = 36.0 / 1000.; // 36 mm
    scene.active_camera.sensor.horizontal_res = 240 * 4;
    scene.active_camera.sensor.vertical_res = 160 * 4;

    scene.active_camera.position = Vector::new(0.0, 0.0, 7.);
    let mut light = PointLight::default();
    light.position = Vector::new(-5.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(2000., 1000.);
    scene.simple_lights.push(Arc::new(light));

    let mut light = PointLight::default();
    light.position = Vector::new(0.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(3000., 1000.);
    scene.simple_lights.push(Arc::new(light));

    let mut light = PointLight::default();
    light.position = Vector::new(5.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(4000., 1000.);
    scene.simple_lights.push(Arc::new(light));

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
    scene.objects.push(object);
    let object = Object {
        position: Vector::new(-3.0, 0.0, 0.),
        meshes: vec![cube.clone()],
        ..Object::default()
    };
    scene.objects.push(object);
    let object = Object {
        position: Vector::new(0.0, 0.0, 0.),
        meshes: vec![sphere.clone()],
        ..Object::default()
    };
    scene.objects.push(object);
    let object = Object {
        position: Vector::new(0.0, -2.0, 0.),
        meshes: vec![plane.clone()],
        ..Object::default()
    };
    scene.objects.push(object);

    scene.rendermode = Rendermode::ThreadedRayTrace;
    scene.samples = 10;
    scene.max_trace_depth = 1;
    scene.max_render_dist = 20.;
    scene
}

pub fn calibration_scene<'b>() -> Scene {
    let lens = Lens {
        aperture: 50.0,
        focal_length: 1.0 / 1000., // 120.
        focus_distance: 2.0,
    };
    let sensor = Sensor {
        width: 4.0 / 1000., // 36 mm
        horizontal_res: 100,
        vertical_res: 100,
    };
    let camera = Camera {
        position: Vector::new(0.0, 0.0, 0.8),
        // orientation: Polar
        lens,
        sensor,
        exposure_time: 3e10,
        ..Camera::default()
    };
    let mut lights: Vec<Arc<dyn Light>> = vec![];

    let light = PointLight::new(
        Vector::new(0.0, 0.0, 10.0),
        RIGHT,
        const_spectra(1000.).into(),
    );
    lights.push(Arc::new(light));

    let meshes = Vec::new();
    let mut objects = Vec::new();
    let cube = load_wavefront_obj("models/cube.obj".to_string());
    let object = Object {
        position: Vector::new(0.0, 0.0, -1.),
        meshes: vec![cube.clone()],
        ..Object::default()
    };
    objects.push(object);

    let background = black_spectra();

    Scene {
        active_camera: camera,
        simple_lights: lights,
        _meshes: meshes,
        background,
        tick: 0,
        rendermode: Rendermode::ThreadedRayTrace,
        shadermode: ShaderMode::Lit,
        logging: 0,
        objects,
        spatial_acceleration_structures: true,
        recursive_raycasting: true,
        samples: 1,
        max_trace_depth: 0,
        max_render_dist: 20.,
        ..Default::default()
    }
}
