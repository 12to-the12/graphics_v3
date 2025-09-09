use std::sync::Arc;

use crate::camera::{Camera, Lens, Sensor};
use crate::geometry::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::{Mesh, Vector};
use crate::material::PBR;
use crate::object::Object;
// use crate::primitives::Object;
use crate::lighting::{black_spectra, norm_black_body, Light, PointLight, Spectra};
use crate::load_object_file::load_wavefront_obj;

#[derive(Clone)]
pub enum ShaderMode {
    _BVH,
    _Solid,
    Lit,
}

#[derive(Clone, PartialEq)]
pub enum Rendermode {
    _RayTrace,
    ThreadedRayTrace,
    _Rasterize,
}

/// I am not sure what the responsibilities of this construction should be
/// should it be concerned with intermediate rendering data?
/// like transformed coordinates?
///
/// or should it be read only?
/// obviously the enclosed information needs to persist for anything beyond a single frame render
/// I want to avoid duplicating the information if necessary
///
/// I have settled on unified
/// To implement a unified mesh, references need to stay valid
// #[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Arc<dyn Light>>,
    pub objects: Vec<Object>,
    pub _meshes: Vec<Mesh>,
    pub background: Spectra,
    pub tick: usize,
    pub rendermode: Rendermode,
    pub shadermode: ShaderMode,
    pub logging: u8,
    pub spatial_acceleration_structures: bool,
    pub threads: u32,
}

pub fn simple_scene<'b>() -> Scene {
    let lens = Lens {
        _aperture: 50.0,
        focal_length: 50.0, // 120.
        _focus_distance: 2.0,
    };
    let sensor = Sensor {
        width: 36.0, // 36 mm
        // height: 24.0,
        horizontal_res: 420,
        vertical_res: 320,
    };
    let camera = Camera {
        position: Vector::new(0.0, 0.0, 10.0),
        // orientation: Polar
        lens,
        sensor,
        exposure_time: 20_000_000.,
        ..Camera::default()
    };
    let mut lights: Vec<Arc<dyn Light>> = vec![];

    let light = PointLight::new(Vector::new(-5.0, 3.0, -5.0), RIGHT, norm_black_body(1500.));
    lights.push(Arc::new(light));

    let lightb = PointLight::new(Vector::new(5.0, 1.0, -5.0), RIGHT, norm_black_body(3000.));
    lights.push(Arc::new(lightb));

    let lightc = PointLight::new(Vector::new(-5.0, 5.0, -12.0), RIGHT, norm_black_body(6000.));
    lights.push(Arc::new(lightc));

    let meshes = Vec::new();
    let mut objects = Vec::new();
    let mesh = load_wavefront_obj("models/cube.obj".to_string());
    let object = Object {
        position: Vector::new(-3.0, 0.0, -10.0),
        meshes: vec![mesh],
        ..Object::default()
    };

    objects.push(object);

    let mesh = load_wavefront_obj("models/cube.obj".to_string());
    let object = Object {
        position: Vector::new(3.0, 0.0, -10.0),
        meshes: vec![mesh],
        ..Object::default()
    };
    objects.push(object);

    let mesh: Mesh = load_wavefront_obj("models/sphere.obj".to_string());

    let object = Object {
        position: Vector::new(0., 0., -6.0),
        meshes: vec![mesh],
        material: Arc::new(PBR::new(1.0, 0.0)),
        ..Object::default()
    };
    objects.push(object);

    let mesh: Mesh = load_wavefront_obj("models/plane.obj".to_string());
    let object = Object {
        position: Vector::new(0., -2., 0.0),
        meshes: vec![mesh],
        ..Object::default()
    };
    objects.push(object);

    let background = black_spectra(crate::lighting::RadiometricUnit::Flux);
    let scene = Scene {
        camera,
        lights,
        _meshes: meshes,
        background,
        tick: 0,
        rendermode: Rendermode::ThreadedRayTrace,
        shadermode: ShaderMode::Lit,
        logging: 0,
        objects,
        spatial_acceleration_structures: true,
        threads: 60,
    };
    return scene;
}
