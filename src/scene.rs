use std::sync::Arc;

use crate::camera::{Camera, Lens, Sensor};
use crate::geometry::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::{Mesh, Vector};
use crate::object::{Empty, Object};
// use crate::primitives::Object;
use crate::lighting::{
    black_spectra, const_spectra, incandescent_spectra, Light, PointLight, Spectra,
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
    Rasterize,
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
    // pub struct Scene<T: Entity> {
    pub scene_root: Empty,
    pub active_camera: Camera,
    pub lights: Vec<Arc<dyn Light>>,
    pub objects: Vec<Object>,
    pub _meshes: Vec<Mesh>,
    pub background: Spectra,
    pub tick: u32,
    pub rendermode: Rendermode,
    pub shadermode: ShaderMode,
    pub logging: u8,
    pub spatial_acceleration_structures: bool,
    pub recursive_raycasting: bool,
    pub threads: u32,
    pub samples: u32,
    pub max_trace_depth: u32,
    pub max_render_dist: f32,
    pub tilesize: u32,
}
impl Default for Scene {
    fn default() -> Self {
        let scene = Scene {
            scene_root: Empty::default(),
            active_camera: Camera::default(),
            lights: Vec::new(),
            objects: Vec::new(),
            _meshes: Vec::new(),
            background: black_spectra(),
            tick: 0,
            rendermode: Rendermode::ThreadedRayTrace,
            shadermode: ShaderMode::Lit,
            logging: 0,
            spatial_acceleration_structures: true,
            recursive_raycasting: true,
            threads: 1,
            samples: 1,
            max_trace_depth: 1,
            max_render_dist: 1e6,
            tilesize: 128,
        };
        // scene.build_light_vector();
        scene
    }
}
impl Scene {
    // fn build_light_vector(&mut self) -> () {

    // }
    // fn add_light<T: Light + 'static>(&mut self, light: T) -> () {
    //     self.scene_root.add_child(Arc::new(light));
    // }
}

pub fn calibration_scene<'b>() -> Scene {
    let lens = Lens {
        _aperture: 50.0,
        focal_length: 1.0 / 1000., // 120.
        _focus_distance: 2.0,
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
        lights,
        _meshes: meshes,
        background,
        tick: 0,
        rendermode: Rendermode::ThreadedRayTrace,
        shadermode: ShaderMode::Lit,
        logging: 0,
        objects,
        spatial_acceleration_structures: true,
        recursive_raycasting: true,
        threads: 1,
        samples: 1,
        max_trace_depth: 0,
        max_render_dist: 20.,
        ..Default::default()
    }
}

pub fn simple_scene<'b>() -> Scene {
    let mut scene = Scene::default();
    scene.active_camera.exposure_time = 1e16;
    scene.active_camera.lens._aperture = 50.;
    scene.active_camera.lens.focal_length = 20.0 / 1000.; // 120.
    scene.active_camera.lens._focus_distance = 2.; // 120.

    scene.active_camera.lens._aperture = 50.;
    scene.active_camera.sensor.width = 36.0 / 1000.; // 36 mm
    scene.active_camera.sensor.horizontal_res = 240 * 4;
    scene.active_camera.sensor.vertical_res = 160 * 4;

    scene.active_camera.position = Vector::new(0.0, 0.0, 7.);
    let mut light = PointLight::default();
    light.position = Vector::new(-5.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(2000., 1000.);
    scene.lights.push(Arc::new(light));

    let mut light = PointLight::default();
    light.position = Vector::new(0.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(3000., 1000.);
    scene.lights.push(Arc::new(light));

    let mut light = PointLight::default();
    light.position = Vector::new(5.0, 5.0, 3.0);
    light.radiant_flux = incandescent_spectra(4000., 1000.);
    scene.lights.push(Arc::new(light));

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

    scene.rendermode = Rendermode::Rasterize;
    scene.threads = 48;
    scene.samples = 32;
    scene.max_trace_depth = 1;
    scene.max_render_dist = 20.;
    scene
}

pub fn cornell_scene<'b>() -> Scene {
    let lens = Lens {
        _aperture: 50.0,
        focal_length: 80.0 / 1000., // 120.
        _focus_distance: 2.0,
    };
    let sensor = Sensor {
        width: 36.0 / 1000., // 36 mm
        horizontal_res: 240 * 2,
        vertical_res: 240 * 2,
    };
    let camera = Camera {
        position: Vector::new(0., 2.74, 13.),
        // orientation: Polar
        lens,
        sensor,
        exposure_time: 1e14,
        ..Camera::default()
    };
    let mut lights: Vec<Arc<dyn Light>> = vec![];

    let light = PointLight::new(
        Vector::new(0.0, 5.0, -0.),
        // Vector::new(0.0, 3.0, -0.5),
        RIGHT,
        incandescent_spectra(2500., 1000.),
    );
    lights.push(Arc::new(light));

    let meshes = Vec::new();
    let mut objects = Vec::new();
    let cornell = load_wavefront_obj("models/cornell.obj".to_string());

    let object = Object {
        position: Vector::new(2.8, 0., 0.),
        meshes: vec![cornell.clone()],
        ..Object::default()
    };
    objects.push(object);

    // let object = Object {
    //     position: Vector::new(0.0, 4.5, -3.),
    //     meshes: vec![cube.clone()],
    //     ..Object::default()
    // };
    // objects.push(object);

    let background = black_spectra();

    Scene {
        active_camera: camera,
        lights,
        _meshes: meshes,
        background,
        tick: 0,
        rendermode: Rendermode::ThreadedRayTrace,
        shadermode: ShaderMode::Lit,
        logging: 1,
        objects,
        spatial_acceleration_structures: true,
        recursive_raycasting: true,
        threads: 48,
        // samples: 1,
        samples: 16,
        max_trace_depth: 4,
        max_render_dist: 20.,
        tilesize: 80,
        ..Default::default()
    }
}
