use std::cmp::min;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::application::application;
use crate::camera::Camera;
use crate::geometry::primitives::{Triangle, Vector};
// use crate::primitives::LineCollection;
use crate::scene::scene::{Rendermode, Scene, ShaderMode};
// use crate::primitives::PolygonCollection;
use crate::geometry::transformations::{
    build_projection_transform, build_scale_transform, build_translation_transform,
    compile_transforms, Transform,
};
use crate::rasterization::rasterization::rasterize_triangle;
use crate::ray_tracing::pixel_shader::{
    _solid_shader, bvh_shader, lit_shader, shade_pixels, z_shader,
};
use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use stopwatch::Stopwatch;

/// transforms from world space to camera space
fn build_camera_space_transform(camera: &Camera) -> Transform {
    build_translation_transform(camera.position.inv())

    // the camera rotation needs to be added
}

fn build_to_projection_transform(scene: &Scene) -> Transform {
    build_projection_transform(&scene.active_camera)
}

fn build_to_display_transform(scene: &Scene) -> Transform {
    let camera = &scene.active_camera;
    let aspect_ratio = camera.sensor.aspect_ratio();
    let hres = camera.sensor.horizontal_res as f32;
    let vres = camera.sensor.vertical_res as f32;

    let mut display = Vec::new();
    // to de center the coordinates
    display.push(build_scale_transform(Vector::new(1.0, aspect_ratio, 1.0)));

    display.push(build_scale_transform(Vector::new(-1.0, 1.0, 0.0)));

    display.push(build_translation_transform(Vector::new(1.0, 1.0, 0.0)));
    display.push(build_scale_transform(Vector::new(0.5, 0.5, 1.0)));

    // to make them pixel correct
    display.push(build_scale_transform(Vector::new(hres, vres, 1.0)));
    compile_transforms(&display)
}

/// applies the transforms for the rasterizer
fn vertex_shader(scene: &mut Scene) {
    apply_transforms(scene);
}

fn apply_transforms(scene: &mut Scene) {
    // calculate_global_space(scene);
    // view transform that goes from global space to clip space

    // world -> camera translation
    // world -> camera rotation
    // camera -> projection operation

    // --> clip coordinates
    let mut uniform_view_transforms: Vec<Transform> = Vec::new();
    // let to_camera_space = build_camera_space_transform(&scene.camera);
    if scene.rendermode == Rendermode::Rasterize {
        uniform_view_transforms.push(build_camera_space_transform(&scene.active_camera));
        uniform_view_transforms.push(build_to_projection_transform(scene));
        uniform_view_transforms.push(build_to_display_transform(scene));
    }
    let uniform_view_transform = compile_transforms(&uniform_view_transforms);

    // scene.transform_children
    // scene.build_camera_space_mesh
    // this will recursively apply transforms to nodes, then apply them to meshes
    for object in &mut scene.objects {
        let to_world_space = build_translation_transform(object.position);
        for mesh in &mut object.meshes {
            // for mesh in scene.meshes.iter_mut() {
            mesh.add_transform(to_world_space.clone());

            // if scene.rendermode == Rendermode::Rasterize {
            //     // mesh.add_transform(to_camera_space.clone());
            // }
            mesh.add_transform(uniform_view_transform.clone());

            mesh.apply_transformations();
        }
    }
    // scene.root.cascade_transforms();
    // scene.apply_transformations();
}

fn wire_frame(canvas: &mut RgbImage, scene: Scene) {
    for object in scene.objects {
        for mesh in object.meshes {
            // mesh.apply_transformations();
            for poly in mesh.polygons {
                let a = &mesh.output_vertices[poly[0]]; // currently vertexes;
                let b = &mesh.output_vertices[poly[1]];
                let c = &mesh.output_vertices[poly[2]];
                let triangle = Triangle::new(a, b, c);
                if triangle.get_sign() {
                    rasterize_triangle(triangle, canvas, Rgb([0, 255, 0]));
                }
            }
        }
    }
}

fn _solid(canvas: &mut RgbImage, scene: Scene) {
    for object in scene.objects {
        for mut mesh in object.meshes {
            mesh.apply_transformations();
            for poly in mesh.polygons {
                let a = &mesh.output_vertices[poly[0]]; // currently vertexes;
                let b = &mesh.output_vertices[poly[1]];
                let c = &mesh.output_vertices[poly[2]];

                rasterize_triangle(Triangle::new(a, b, c), canvas, Rgb([0, 255, 0]));
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
    if scene.logging > 0 {
        println!("raster_time: {:?}", raster_time.elapsed());
    }

    wire_frame(canvas, scene);
}

pub struct Tile {
    pub canvas: RgbImage,
    pub x_start: u32,
    pub y_start: u32,
    pub width: u32,
    pub height: u32,
}
impl Tile {
    pub fn new(x_start: u32, y_start: u32, width: u32, height: u32) -> Tile {
        Tile {
            canvas: ImageBuffer::new(width, height),
            x_start,
            y_start,
            width,
            height,
        }
    }
}
fn ray_trace(canvas: &mut RgbImage, mut scene: Scene) {
    apply_transforms(&mut scene);
    let shadermode = match scene.shadermode {
        ShaderMode::Lit => lit_shader,
        ShaderMode::_BVH => bvh_shader,
        ShaderMode::_Solid => _solid_shader,
        ShaderMode::_ZDepth => z_shader,
    };
    let mut tile: Tile = Tile::new(
        0,
        0,
        scene.active_camera.sensor.horizontal_res,
        scene.active_camera.sensor.vertical_res,
    );

    shade_pixels(&mut tile, &scene, shadermode);

    let _ = canvas.copy_from(&tile.canvas, 0, 0);
}
fn threaded_ray_trace(canvas: &mut RgbImage, mut scene: Scene) {
    apply_transforms(&mut scene);

    // let width = scene.active_camera.sensor.horizontal_res / scene.threads;
    let width = scene.active_camera.sensor.horizontal_res;
    let height = scene.active_camera.sensor.vertical_res;

    let x_tiles =
        (scene.active_camera.sensor.horizontal_res as f32 / scene.tilesize as f32).ceil() as u32;
    let y_tiles =
        (scene.active_camera.sensor.vertical_res as f32 / scene.tilesize as f32).ceil() as u32;
    let shadermode = match scene.shadermode {
        ShaderMode::Lit => lit_shader,
        ShaderMode::_BVH => bvh_shader,
        ShaderMode::_Solid => _solid_shader,
        ShaderMode::_ZDepth => z_shader,
    };
    let data = Arc::new(scene); // necessary for borrowing in threads
    let (sender, receiver) = mpsc::channel::<Tile>();

    let mut render_timer = Stopwatch::start_new();
    // let mut handles = Vec::new();

    let mut tiles: Vec<Tile> = Vec::new();

    // for i in 0..data.threads {
    //     tiles.push(Tile::new(
    //         i * (width / data.threads),
    //         0,
    //         width / data.threads,
    //         height,
    //     ));
    //     pixel_checker += (width / data.threads) * height;
    // }
    // println!("x_tiles: {}",x_tiles);
    // println!("y_tiles: {}",y_tiles);
    // println!("width: {}",width);
    // println!("height: {}",height);
    // println!("tilesize: {}",data.tilesize);
    for i in 0..x_tiles {
        for j in 0..y_tiles {
            let x_start = i * data.tilesize;
            let y_start = j * data.tilesize;
            tiles.push(Tile::new(
                x_start,
                y_start,
                min(data.tilesize, width - x_start),
                min(data.tilesize, height - y_start),
            ));
        }
    }
    tiles.shuffle(&mut thread_rng());
    let tilecount = tiles.len();

    let handle = thread::spawn(move || {
        tiles.into_par_iter().for_each(move |mut tile| {
            let mut thread_timer = Stopwatch::start_new();
            shade_pixels(&mut tile, &Arc::clone(&data), shadermode);
            thread_timer.stop();
            let hue = thread_timer.elapsed_ms();
            if data.hue_timer {
                print!("thread timer: {thread_timer} - ");
                for (x, y, _) in tile.canvas.clone().enumerate_pixels().clone() {
                    tile.canvas.put_pixel(
                        x,
                        y,
                        Rgb([
                            min(hue, 255) as u8,
                            min(hue / 8, 255) as u8,
                            min(hue / 64, 255) as u8,
                        ]),
                    );
                }
            }
            sender.clone().send(tile).unwrap();
        });
    });

    // for i in 0..threadcount {
    //     // sliced vertically

    //     let data_clone = Arc::clone(&data);

    //     // let mut mini_canvas: RgbImage = ImageBuffer::new(width, height);
    //     let mut tile: Tile = Tile::new(i * width, 0, width, height);
    //     // let mut tile: Tile = Tile::new(i * width, 0, data.tilesize, data.tilesize);

    //     let handle = thread::spawn(move || {
    //         shade_pixels(&mut tile, &data_clone, shadermode);
    //         tile
    //     });
    //     handles.push(handle);
    // }

    // let painted_mini_canvas = handle.join().unwrap();
    // for handle in handles {
    //     tiles.push(handle.join().unwrap());
    // }

    // tiles.into_iter().for_each(|tile| {
    //     let _ = canvas.copy_from(&tile.canvas, tile.x_start, tile.y_start);
    // });
    let mut i = 0;
    for tile in receiver {
        println!("{}/{}", i + 1, tilecount);
        i += 1;
        canvas
            .copy_from(&tile.canvas, tile.x_start, tile.y_start)
            .unwrap();
        canvas.save("partial.png").unwrap();
    }
    handle.join().unwrap();

    render_timer.stop();
    println!("parallel ray tracing: {:?}", render_timer.elapsed());
}

/// this serves as an abstraction away from rasterization, so that ray tracing can be dropped into the pipeline
/// it owns nothing, it just operates on a canvas
/// I am unsure of the best way to pass it information
///
fn render(canvas: &mut RgbImage, scene: Scene) {
    match &scene.rendermode {
        Rendermode::ThreadedRayTrace => threaded_ray_trace(canvas, scene),

        Rendermode::_RayTrace => ray_trace(canvas, scene),

        Rendermode::Rasterize => rasterize(canvas, scene),
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

    let horizontal_res = scene.active_camera.sensor.horizontal_res;
    let vertical_res = scene.active_camera.sensor.vertical_res;
    let mut canvas: RgbImage = ImageBuffer::new(horizontal_res, vertical_res);

    render(&mut canvas, scene);

    canvas
}
