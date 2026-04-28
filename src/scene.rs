use std::sync::Arc;

use crate::camera::{Camera, Lens, Sensor};
use crate::geometry::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::{Mesh, Vector};
use crate::material::Diffuse;
use crate::object::Object;
// use crate::primitives::Object;
use crate::lighting::{
    black_spectra, const_spectra, green_spectra, incandescent_spectra, norm_black_body, Light,
    PointLight, Spectra,
};
use crate::load_object_file::load_wavefront_obj;

#[derive(Clone)]
pub enum ShaderMode {
    _BVH,
    _Solid,
    Lit,
    _ZDepth,
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
    pub _recursive_raycasting: bool,
    pub threads: u32,
    pub samples: u32,
    pub max_trace_depth: u32,
    pub max_render_dist: f32,
}

pub fn simple_scene<'b>() -> Scene {
    let lens = Lens {
        _aperture: 50.0,
        focal_length: 50.0 / 1000., // 120.
        _focus_distance: 2.0,
    };
    let sensor = Sensor {
        width: 36.0 / 1000., // 36 mm
        // height: 24.0,
        horizontal_res: 210 * 2,
        vertical_res: 160 * 2,
    };
    let camera = Camera {
        position: Vector::new(0.0, 0.0, 10.0),
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
    let sphere: Mesh = load_wavefront_obj("models/sphere.obj".to_string());
    let plane: Mesh = load_wavefront_obj("models/plane.obj".to_string());
    let wall: Mesh = load_wavefront_obj("models/wall.obj".to_string());
    let cornell: Mesh = load_wavefront_obj("models/cornell.obj".to_string());

    let object = Object {
        position: Vector::new(0.0, 0.0, 5.34),
        meshes: vec![cube.clone()],
        ..Object::default()
    };
    objects.push(object);

    let background = black_spectra();
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
        _recursive_raycasting: true,
        threads: 42,
        samples: 8,
        max_trace_depth: 0,
        max_render_dist: 20.,
    };
    return scene;
}
