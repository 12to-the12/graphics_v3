use crate::camera::{Camera, Lens, Sensor};
use crate::orientation::RIGHT;
// use crate::coordinate_space::Polar;
use crate::primitives::{vector, vertex, Mesh};
// use crate::primitives::Object;
use crate::lighting::{black_body, black_spectra, const_spectra, norm_black_body, point_light, PointLight, Spectra};
use crate::load_object_file::load_obj;
use image::Rgb;

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
    pub lights: Vec<PointLight>,
    pub meshes: Vec<Mesh>,
    // pub unified_mesh: Vec<Polygon<'a>>,
    // pub unified_vertices: Vec<Vertex>, // pub materials: Vec<Material>,
    // pub image_assets: Vec<Image>,
    // pub time: Time, // timestamp to render at
    // pub settings: Settings,
    // geometry: <T,Mesh>,
    pub background: Rgb<u8>,
    pub tick: usize,
}

pub fn simple_scene() -> Scene {
    let lens = Lens {
        aperture: 50.0,
        focal_length: 120.0,
        focus_distance: 2.0,
    };
    let sensor = Sensor {
        width: 36.0,
        // height: 24.0,
        horizontal_res: 480,
        vertical_res: 320,
    };
    let camera = Camera {
        position: vertex(0.0, 0.0, 15.0),
        // orientation: Polar
        lens,
        sensor,
        near_clipping_plane: 1e-1,
        far_clipping_plane: 1e6,
        exposure_time: 1.,
    };
    let light = point_light(vertex(-100.0, 0.0, 0.0), RIGHT, norm_black_body(6000.));

    let lightb = point_light(vertex(100.0, 100.0, 100.0), RIGHT, norm_black_body(2000.));
    // println!("{:?}",light.radiant_flux.from_λ(700.));
    let lights = vec![light,lightb];
    let mut meshes = Vec::new();
    // let mesh = unit_cube(vector(0.0, 0.0, -5.0));
    // let mesh = sample_mesh(vector(0.0, 0.0, -3.0));
    let mut mesh = load_obj("models/cube.obj".to_string());
    mesh.position = vector(-3.0, 0.0, -10.0);
    meshes.push(mesh);

    let mut mesh = load_obj("models/sphere.obj".to_string());
    mesh.position = vector(0.0, 0.0, -10.0);
    meshes.push(mesh);

    let mut mesh = load_obj("models/sphere.obj".to_string());
    mesh.position = vector(3.0, 0.0, -10.0);
    meshes.push(mesh);

    let background = Rgb([0, 0, 0]);
    let scene = Scene {
        camera,
        lights,
        meshes,
        background,
        tick: 0, // unified_mesh: Vec::new(),
                 // unified_vertices: Vec::new(),
    };
    return scene;
}
