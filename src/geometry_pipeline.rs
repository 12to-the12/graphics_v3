use std::vec;

use crate::application::application;
use crate::primitives::{vector, Point, Polygon, Triangle, triangle, Vector, Vertex};
// use crate::primitives::LineCollection;
use crate::scene::Scene;
// use crate::primitives::PolygonCollection;
use crate::transformations::{
    build_projection_transform, build_scale_transform, build_translation_transform,
    compile_transforms, Transform,
};
use image::{ImageBuffer, Rgb, RgbImage};

use stopwatch::Stopwatch;

/// transforms from world space to camera space
fn build_camera_space_transform(scene: &Scene) -> Transform {
    let camera = &scene.camera;
    return build_translation_transform(camera.position.inv());

    // the camera rotation needs to be added
}

fn build_to_projection_transform(scene: &Scene) -> Transform {
    return build_projection_transform(&scene.camera);
}

// fn build_to_clip_transform(scene: &Scene) -> Transform {
//     return build_clip_transform(&scene.camera);
// }

fn build_to_display_transform(scene: &Scene) -> Transform {
    let camera = &scene.camera;
    let hres = camera.sensor.horizontal_res as f32;
    let vres = camera.sensor.vertical_res as f32;

    let mut display = Vec::new();
    // to de center the coordinates
    display.push(build_translation_transform(vector(1.0, 1.0, 0.0)));
    display.push(build_scale_transform(vector(0.5, 0.5, 1.0)));

    // to make them correct
    display.push(build_scale_transform(vector(hres, vres, 1.0)));
    compile_transforms(&display)
}

fn vertex_shader(scene: &mut Scene) {
    // calculate_global_space(scene);
    // view transform that goes from global space to clip space

    // world -> camera translation
    // world -> camera rotation
    // camera -> projection operation

    // --> clip coordinates
    let mut uniform_view_transforms: Vec<Transform> = Vec::new();

    uniform_view_transforms.push(build_camera_space_transform(scene));
    uniform_view_transforms.push(build_to_projection_transform(scene));
    uniform_view_transforms.push(build_to_display_transform(scene));

    // println!("{:?}",uniform_view_transforms);
    let uniform_view_transform = compile_transforms(&uniform_view_transforms);
    // println!("{:?}",uniform_view_transform);
    for mesh in &mut scene.meshes {
        let transform = build_translation_transform(mesh.position.clone());
        mesh.add_transform(transform);
        mesh.add_transform(uniform_view_transform.clone());
        // println!("transforms: {:?}\n\n", mesh.get_transforms());
    }
}

fn wire_frame(canvas: &mut RgbImage, scene: Scene) {
    let color = Rgb([0, 255, 0]);
    for mut mesh in scene.meshes {
        mesh.apply_transformations();
        for poly in mesh.polygons {
            let a = &mesh.output_vertices[poly[0]]; // currently vertexes;
            let b = &mesh.output_vertices[poly[1]];
            let c = &mesh.output_vertices[poly[2]];

            triangle(a, b, c).draw(canvas, color);
        }
    }
}

/// the function that actually paints the scene into an image
/// this takes a reference to a canvas and paints upon it
///
/// this is where shading happens
/// for now, it will draw lines, but in the future, solid triangles,  then phong shading and pixel shaders
///
/// data that needs to be made available to the rasterizer:
/// - the screen space coordinates of every mesh in the scene
/// - vertex information
/// - textures and shaders/materials associated with every mesh/vertex/pixel/face
///
/// pixel shaders need to know the following to work properly
/// the geometry id
/// the material associated with that geometry
/// the associated images maps and the corresponding coordinates
fn rasterize(canvas: &mut RgbImage, scene: Scene) {
    wire_frame(canvas, scene)
}

/// this serves as an abstraction away from rasterization, so that ray tracing can be dropped into the pipeline
/// it owns nothing, it just operates on a canvas
/// I am unsure of the best way to pass it information
///
fn render(canvas: &mut RgbImage, scene: Scene) {
    rasterize(canvas, scene);
}

/// Application
/// Geometry
/// Rasterization
/// the pipeline turns a scene into an image
///
/// can be used to make images meant for display or saving to disk,
/// animations, real time, whatever. It serves as an abstraction
///
/// Currently, this is a purely software implementation that runs on a single core
pub fn geometry_pipeline(mut scene: Scene, counter: f32) -> RgbImage {
    application(&mut scene, counter); // arrives at the geometry to render

    let mut vertex_shade = Stopwatch::start_new();

    vertex_shader(&mut scene); // projections
    vertex_shade.stop();
    println!("vertex_shade: {:?}", vertex_shade.elapsed());

    let horizontal_res = scene.camera.sensor.horizontal_res;
    let vertical_res = scene.camera.sensor.vertical_res;
    let mut canvas: RgbImage = ImageBuffer::new(horizontal_res, vertical_res);

    let mut render_time = Stopwatch::start_new();
    render(&mut canvas, scene);
    render_time.stop();
    println!("render: {:?}", render_time.elapsed());
    canvas
}
