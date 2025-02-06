use std::sync::Arc;
use std::thread::{self};

use crate::application::application;
use crate::camera::Camera;
use crate::geometry::primitives::{triangle, vector};
// use crate::primitives::LineCollection;
use crate::scene::{Rendermode, Scene};
// use crate::primitives::PolygonCollection;
use crate::geometry::transformations::{
    build_projection_transform, build_scale_transform, build_translation_transform,
    compile_transforms, Transform,
};
use crate::rasterization::line_plotting::plot_triangle;
use crate::rasterization::rasterization::rasterize_triangle;
use crate::ray_tracing::pixel_shader::{lit_shader, shade_pixels};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use stopwatch::Stopwatch;

/// transforms from world space to camera space
fn build_camera_space_transform(camera: &Camera) -> Transform {
    return build_translation_transform(camera.position.inv());

    // the camera rotation needs to be added
}

fn build_to_projection_transform(scene: &Scene) -> Transform {
    return build_projection_transform(&scene.camera);
}

// fn build_to_clip_transform(scene: &Scene) -> Transform {
//  return build_clip_transform(&scene.camera);
// }

fn build_to_display_transform(scene: &Scene) -> Transform {
    let camera = &scene.camera;
    let aspect_ratio = camera.sensor.aspect_ratio();
    let hres = camera.sensor.horizontal_res as f32;
    let vres = camera.sensor.vertical_res as f32;

    let mut display = Vec::new();
    // to de center the coordinates
    display.push(build_scale_transform(vector(1.0, aspect_ratio, 1.0)));

    display.push(build_scale_transform(vector(-1.0, 1.0, 0.0)));

    display.push(build_translation_transform(vector(1.0, 1.0, 0.0)));
    display.push(build_scale_transform(vector(0.5, 0.5, 1.0)));

    // to make them pixel correct
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

    uniform_view_transforms.push(build_camera_space_transform(&scene.camera));
    uniform_view_transforms.push(build_to_projection_transform(scene));
    uniform_view_transforms.push(build_to_display_transform(scene));

    // println!("{:?}",uniform_view_transforms);
    let uniform_view_transform = compile_transforms(&uniform_view_transforms);
    // println!("{:?}",uniform_view_transform);
    for object in &mut scene.objects {
        for mesh in &mut object.meshes {
            let transform = build_translation_transform(object.position.clone());
            mesh.add_transform(transform);
            mesh.add_transform(uniform_view_transform.clone());
            println!("transforms: {:?}\n\n", mesh._get_transforms());
        }
    }
}

fn _wire_frame(canvas: &mut RgbImage, scene: Scene) {
    let color = Rgb([0, 255, 0]);
    for mut mesh in scene.meshes {
        mesh.apply_transformations();
        for poly in mesh.polygons {
            let a = &mesh.output_vertices[poly[0]]; // currently vertexes;
            let b = &mesh.output_vertices[poly[1]];
            let c = &mesh.output_vertices[poly[2]];

            plot_triangle(triangle(a, b, c), canvas, color)
        }
    }
}

fn solid(canvas: &mut RgbImage, scene: Scene) {
    for object in scene.objects {
        for mut mesh in object.meshes {
            mesh.apply_transformations();
            for poly in mesh.polygons {
                let a = &mesh.output_vertices[poly[0]]; // currently vertexes;
                let b = &mesh.output_vertices[poly[1]];
                let c = &mesh.output_vertices[poly[2]];

                rasterize_triangle(triangle(a, b, c), canvas);
            }
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
fn rasterize(canvas: &mut RgbImage, mut scene: Scene) {
    let mut raster_time = Stopwatch::start_new();

    vertex_shader(&mut scene); // projections
    raster_time.stop();
    println!("raster_time: {:?}", raster_time.elapsed());

    solid(canvas, scene);
}

fn apply_transforms(scene: &mut Scene) {
    for object in &mut scene.objects {
        for mesh in &mut object.meshes {
            // for mesh in scene.meshes.iter_mut() {
            // println!("{:?}",object.position);
            let to_world_space = build_translation_transform(object.position.clone());
            mesh.add_transform(to_world_space);
            let to_camera_space = build_camera_space_transform(&scene.camera);
            mesh.add_transform(to_camera_space);

            mesh.apply_transformations();
            // println!("transforms: {:?}\n\n", mesh._get_transforms());
        }
    }
}

fn ray_trace(canvas: &mut RgbImage, mut scene: Scene) {
    apply_transforms(&mut scene);

    shade_pixels(
        canvas,
        &scene,
        lit_shader,
        0,
        scene.camera.sensor.horizontal_res,
        0,
        scene.camera.sensor.vertical_res,
    ); // lit_shader solid_shader solid_shader
}
fn threaded_ray_trace(canvas: &mut RgbImage, mut scene: Scene) {
    apply_transforms(&mut scene);

    let data = Arc::new(scene.clone()); // necessary for borrowing in threads

    let mut threads = Stopwatch::start_new();
    let mut handles = Vec::new();
    let mut canvases: Vec<RgbImage> = Vec::new();

    let width = scene.camera.sensor.horizontal_res / scene.threads;
    let height = scene.camera.sensor.vertical_res;
    for i in 0..scene.threads {
        // sliced vertically

        let data_clone = Arc::clone(&data);

        let mut mini_canvas: RgbImage = ImageBuffer::new(width, height);

        let handle = thread::spawn(move || {
            shade_pixels(
                &mut mini_canvas,
                &data_clone,
                lit_shader,
                i * width,
                i * width + width,
                0,
                height,
            );
            mini_canvas
        });
        handles.push(handle);
    }

    threads.stop();
    if scene.logging > 0 {
        println!("threads: {:?}", threads.elapsed());
    }
    let mut reassembly = Stopwatch::start_new();

    // let painted_mini_canvas = handle.join().unwrap();
    for handle in handles {
        canvases.push(handle.join().unwrap());
    }
    for (i, mini_canvas) in canvases.iter().enumerate() {
        for (x, y, pixel) in mini_canvas.enumerate_pixels() {
            canvas.put_pixel(x + (i as u32) * width, y, *pixel);
        }
        // canvas.clone()
        // .save_with_format("rust-output.png", ImageFormat::Png)
        // .unwrap();
    }

    reassembly.stop();
    if scene.logging > 0 {
        println!("reassembly: {:?}", reassembly.elapsed());
    }

    // for i in 1..10 {
    //     println!("this is number {i} from main");
    //     thread::sleep(Duration::from_millis(1));

    // }
}

/// this serves as an abstraction away from rasterization, so that ray tracing can be dropped into the pipeline
/// it owns nothing, it just operates on a canvas
/// I am unsure of the best way to pass it information
///
fn render(canvas: &mut RgbImage, scene: Scene) {
    match &scene.rendermode {
        Rendermode::ThreadedRayTrace => threaded_ray_trace(canvas, scene),

        Rendermode::_RayTrace => ray_trace(canvas, scene),

        Rendermode::_Rasterize => rasterize(canvas, scene),
    }
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
pub fn geometry_pipeline(mut scene: Scene) -> RgbImage {
    application(&mut scene); // arrives at the geometry to render

    let horizontal_res = scene.camera.sensor.horizontal_res;
    let vertical_res = scene.camera.sensor.vertical_res;
    let mut canvas: RgbImage = ImageBuffer::new(horizontal_res, vertical_res);

    render(&mut canvas, scene);

    canvas
}
