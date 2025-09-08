use crate::camera::CAMERA;
use crate::camera::{Camera, Lens, Sensor};
use crate::geometry::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::geometry::primitives::{vector, Mesh};
use crate::material::{ShaderNode, PBR};
use crate::object::{Object, OBJECT};
// use crate::primitives::Object;
use crate::lighting::{black_spectra, norm_black_body, point_light, LightType, Spectra};
use crate::load_object_file::load_wavefront_obj;

#[derive(Clone)]
pub enum ShaderMode {
    _BVH,
    _Solid,
    Lit,
}

#[derive(Clone,PartialEq)]
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
#[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<LightType>,
    pub objects: Vec<Object>,
    pub _meshes: Vec<Mesh>,
    // pub unified_mesh: Vec<Polygon<'a>>,
    // pub unified_vertices: Vec<Vertex>, // pub materials: Vec<Material>,
    // pub image_assets: Vec<Image>,
    // pub time: Time, // timestamp to render at
    // pub settings: Settings,
    // geometry: <T,Mesh>,
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
        position: vector(0.0, 0.0, 10.0),
        // orientation: Polar
        lens,
        sensor,
        exposure_time:20_000_000.,
        ..CAMERA
    };
    let mut lights = vec![];

    let light = point_light(vector(-5.0, 3.0, -5.0), RIGHT, norm_black_body(1500.));
    // let light = point_light(vertex(-100.0, 0.0, 0.0), RIGHT, 1000*const_spectra(380.));
    // let lightb = point_light(vertex(100.0, 100.0, 100.0), RIGHT, monochroma_spectra(460.,5e-1));
    lights.push(LightType::PointLight(light));

    let lightb = point_light(vector(5.0, 1.0, -5.0), RIGHT, norm_black_body(3000.));
    lights.push(LightType::PointLight(lightb));

    let lightc = point_light(vector(-5.0, 5.0, -12.0), RIGHT, norm_black_body(6000.));
    lights.push(LightType::PointLight(lightc));

    let meshes = Vec::new();
    let mut objects = Vec::new();
    // let mesh = unit_cube(vector(0.0, 0.0, -5.0));
    // let mesh = sample_mesh(vector(0.0, 0.0, -3.0));
    let mesh = load_wavefront_obj("models/cube.obj".to_string());
    // meshes.push(mesh);
    let object = Object {
        position: vector(-3.0, 0.0, -10.0),
        meshes: vec![mesh],
        ..OBJECT
    };

    objects.push(object);

    let mesh = load_wavefront_obj("models/cube.obj".to_string());
    // meshes.push(mesh);
    let object = Object {
        position: vector(3.0, 0.0, -10.0),
        meshes: vec![mesh],
        ..OBJECT
    };
    objects.push(object);

    let mesh: Mesh = load_wavefront_obj("models/sphere.obj".to_string());
    // let mesh = sample_mesh();
    // mesh.position = vector(3.0, 0.0, -10.0);
    // meshes.push(mesh);

    let object = Object {
        position: vector(0., 0., -6.0),
        meshes: vec![mesh],
        material: ShaderNode::PBR(PBR::new(1.0,0.0)),
        ..OBJECT
    };
    objects.push(object);

    let mesh: Mesh = load_wavefront_obj("models/plane.obj".to_string());
    let object = Object {
        position: vector(0., -2., 0.0),
        meshes: vec![mesh],
        ..OBJECT
    };
    objects.push(object);

    // let background = Rgb([0, 0, 0]);
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
