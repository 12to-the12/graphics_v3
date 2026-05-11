use std::sync::Arc;

use crate::camera::Camera;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::Mesh;
use crate::material::BRDF;
use crate::object::{Empty, Object};
// use crate::primitives::Object;
use crate::lighting::{black_spectra, Light, Spectra};

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

pub enum EntityType {
    Camera,
    Light,
    Object,
    Other,
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
    pub entities: Vec<EntityType>,
    pub materials: Vec<Arc<dyn BRDF>>,
    // pub lights: Vec<&'static dyn Light>,
    pub simple_lights: Vec<Arc<dyn Light>>,
    pub objects: Vec<Object>,
    pub _meshes: Vec<Mesh>,
    pub background: Spectra,
    pub tick: u32,
    pub rendermode: Rendermode,
    pub shadermode: ShaderMode,
    pub logging: u8,
    pub spatial_acceleration_structures: bool,
    pub recursive_raycasting: bool,
    pub hue_timer: bool,
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
            entities: Vec::new(),
            materials: Vec::new(),
            simple_lights: Vec::new(),
            objects: Vec::new(),
            _meshes: Vec::new(),
            background: black_spectra(),
            tick: 0,
            rendermode: Rendermode::ThreadedRayTrace,
            shadermode: ShaderMode::Lit,
            logging: 0,
            spatial_acceleration_structures: true,
            recursive_raycasting: true,
            hue_timer: false,
            samples: 1,
            max_trace_depth: 1,
            max_render_dist: 1e6,
            tilesize: 64,
        };
        // scene.build_light_vector();
        scene
    }
}
impl Scene {}
