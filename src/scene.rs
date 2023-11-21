use crate::camera::{Camera, Lens, Sensor};
// use crate::coordinate_space::Polar;
use crate::primitives::{sample_mesh, unit_cube, vector, vertex, Mesh, Polygon, Vertex};
// use crate::primitives::Object;
use crate::load_object_file::load_obj;
use crate::lighting::{sun_light, Light};
use crate::transformations::{
    build_scale_transform, build_x_rotation_transform, build_y_rotation_transform,
    build_z_rotation_transform, Transform,
};

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
    pub lights: Vec<Light>,
    pub meshes: Vec<Mesh>,
    // pub unified_mesh: Vec<Polygon<'a>>,
    // pub unified_vertices: Vec<Vertex>, // pub materials: Vec<Material>,
    // pub image_assets: Vec<Image>,
    // pub time: Time, // timestamp to render at
    // pub settings: Settings,
    // geometry: <T,Mesh>,
}

pub fn simple_scene() -> Scene {
    let lens = Lens {
        aperture: 50.0,
        focal_length: 30.0,
        focus_distance: 2.0,
    };
    let sensor = Sensor {
        width: 36.0,
        // height: 24.0,
        horizontal_res: 400,
        vertical_res: 300,
    };
    let camera = Camera {
        position: vertex(0.0, 0.0, 0.0),
        // orientation: Polar
        lens,
        sensor,
        near_clipping_plane: 1e-1,
        far_clipping_plane: 1e6,
    };
    let light = sun_light(vertex(0.0, 0.0, 0.0), 1.0);
    let lights = vec![light];
    // let mesh = unit_cube(vector(0.0, 0.0, -5.0));
    // let mesh = sample_mesh(vector(0.0, 0.0, -3.0));
    let mut mesh = load_obj("models/cube.obj".to_string());
    mesh.position = vector(0.0, 0.0, -5.0);

    // println!("polygons: {:?}",mesh.polygons);
    let meshes = vec![mesh];
    let scene = Scene {
        camera,
        lights,
        meshes,
        // unified_mesh: Vec::new(),
        // unified_vertices: Vec::new(),
    };
    return scene;
}
