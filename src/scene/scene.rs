use std::sync::Arc;

use slotmap::new_key_type;

use crate::camera::Camera;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::Mesh;
use crate::material::BRDF;
use crate::object::{Empty, Entity, Object};
// use crate::primitives::Object;
use crate::lighting::{black_spectra, Light, Spectra};
use crate::slotmap::SlotMap;

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

pub type EntityType = Box<dyn Entity>;
new_key_type! {pub struct EntityKey;}
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
///
// #[derive(Clone)]
pub struct Scene {
    // pub struct Scene<T: Entity> {
    pub root: EntityKey,
    pub active_camera: Camera,
    pub entities: SlotMap<EntityKey, EntityType>,
    pub materials: Vec<Arc<dyn BRDF>>,
    // pub lights: Vec<&'static dyn Light>,
    pub simple_lights: Vec<EntityKey>,
    objects: Vec<EntityKey>,
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
        let mut entities: SlotMap<EntityKey, Box<dyn Entity>> = SlotMap::with_key();
        let root = entities.insert(Box::new(Empty::default()));
        let scene = Scene {
            root,
            active_camera: Camera::default(),
            entities,
            // entities: SlotMap::new<EntityKey,EntityType>(),
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
impl Scene {
    pub fn get_mut(&mut self, key: EntityKey) -> &mut EntityType {
        self.entities.get_mut(key).unwrap()
    }
    pub fn get(&self, key: EntityKey) -> &EntityType {
        self.entities.get(key).unwrap()
    }
    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        let objects = self.objects.clone();
        let iterable = objects.into_iter();
        let mapped = iterable.map(|key: EntityKey| self.get(key).as_object().unwrap());
        mapped
    }
    pub fn simple_lights(&self) -> impl Iterator<Item = &dyn Light> {
        let simple_lights = self.simple_lights.clone();
        let iterable = simple_lights.into_iter();
        let mapped = iterable.map(|key: EntityKey| self.get(key).as_light().unwrap());
        mapped
    }
    pub fn get_object_keys(&self) -> Vec<EntityKey> {
        self.objects.clone()
    }
    pub fn push_simple_light(&mut self, light: impl Light + 'static) -> EntityKey {
        let entity = Box::new(light);
        let key = self.entities.insert(entity);
        self.simple_lights.push(key);
        self.add_child(self.root, key);
        key
    }
    pub fn push_object(&mut self, object: Object) -> EntityKey {
        let entity: Box<dyn Entity> = Box::new(object);
        let key = self.entities.insert(entity);
        self.objects.push(key);
        self.add_child(self.root, key);
        key
    }
    pub fn add_child(&mut self, parent_key: EntityKey, child_key: EntityKey) {
        let parent = self.get_mut(parent_key);
        parent.add_child(child_key);
        let child = self.get_mut(child_key);
        child.set_parent(parent_key);
    }
    // pub fn crawl_scene_graph(&self){
    //     let children = self.get(self.root);
    //     for child in children{
    //         println!("{:?}",child)
    //     }
    // }

    // pub fn modify_transform_matrix_from_offsets_scales_and_rotations

    // pub fn get_objects(&mut self) -> impl Iterator<Item = &mut EntityType> {
    //     let objects = self.objects.clone();
    //     objects
    //         .into_iter()
    //         .filter_map(move |key| {self.entities.get_mut(key)})
    // }
}
