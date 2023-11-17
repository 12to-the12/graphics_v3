use crate::line_plotting::plot_line;
use crate::primitives::{Point, Polygon, Triangle, Vertex, Vector, vector};
// use crate::primitives::LineCollection;
use crate::scene::Scene;
// use crate::primitives::PolygonCollection;
use image::{ImageBuffer, Rgb, RgbImage};
use crate::transformations::Transform;

fn application(scene: &Scene) -> &Scene {
    println!("The application step does nothing!");
    return scene;
}

// fn almalgamate_geometry(scene: &mut Scene) {
//     for mesh in  &mut scene.meshes{
//         mesh.apply_transformations();
//         scene.unified_vertices.append(&mut mesh.global_vertices.clone());

//         scene.unified_mesh.append(&mut mesh.polygons.clone())
//     }
//     println!("successfully went through amalgamate geometry");
//     println!("{:?}", scene.unified_vertices);
//     println!("{:?}", scene.unified_mesh);
// }

fn calculate_global_space(scene: &mut Scene) {


    let translation = Transform::Translation(vector(0.0, 0.0, -10.0));
    // let translation = Transform::Translation(vector(0.0, -5.0, -10.0));
    for mesh in &mut scene.meshes {
            mesh.transform_log.push(translation.clone());

    }

}
// fn calculate_screen_space(scene: &mut Scene) {}
// fn calculate_device_space(scene: &mut Scene) {
//     // scene.device_space();
// }

/// transform object local space to global space
/// camera to global position and orientation
/// everything then gets transformed to camera space
/// and to screen space after that
///
/// return a list of lines to draw for this mvp implementation
///
/// in the future everything will be constructed of references,
/// but for now, everything is going to be instantiated many times
fn geometry(scene: &mut Scene) {
    // almalgamate_geometry(scene);
    calculate_global_space(scene);
    // calculate_camera_space(scene);
    // calculate_screen_space(scene);
    // calculate_device_space(scene);

    // all objects -> global space
    // for mesh in scene.meshes{
    //     mesh.calculate_global();
    // }
    // for mesh in scene.meshes{
    //     mesh.calculate_camera
    // }
    // all objects -> camera space
    // all objects -> screen space
    //
}

/// this takes input data that has lines, and paints the canvas
/// all of the input geometry is already scaled to the canvas properly
/// all it does is paint the canvas
fn wire_frame(canvas: &mut RgbImage, scene: Scene) {
    let color = Rgb([0, 255, 0]);

    let camera = &scene.camera;
    // let hfov = camera.horizontal_field_of_view();
    let hfov = 90.0;

    let hfactor = hfov / 90.0; // for every meter away

    let vfactor = hfov / 90.0 / camera.sensor.aspect_ratio(); // for every meter away

    let hres = camera.sensor.horizontal_res as f32;
    let vres = camera.sensor.vertical_res as f32;
    // println!("hfactor: {hfactor}");
    // println!("vfactor: {vfactor}");

    for mut mesh in scene.meshes {
        // println!("{:?}", mesh.polygons);
        mesh.apply_transformations();
        for (i, vertex) in mesh.output_vertices.clone().iter().enumerate() {
            // println!("{:?}", vertex);
            let z = vertex.z;
            println!("{:?}\n", vertex);
            let x = vertex.x / -z / hfactor; // negative z on purpose
            let y = vertex.y / z / vfactor; // positive z on purpose
            let foreshortened = Vertex { x, y, z };

            mesh.output_vertices[i] = foreshortened;
        }
        // println!("{:?}", mesh.output_vertices);
        for poly in mesh.polygons {
            let a = &mesh.output_vertices[poly[0]]; // currently vertexes;
            let b = &mesh.output_vertices[poly[1]];
            let c = &mesh.output_vertices[poly[2]];
            println!("{:?}", a);
            println!("{:?}", b);
            println!("{:?}", c);
            println!("");

            let a = Point {
                x: (((a.x + 1.0) / 2.0) * hres) as i32,
                y: (((a.y + 1.0) / 2.0) * vres) as i32,
            };
            let b = Point {
                x: (((b.x + 1.0) / 2.0) * hres) as i32,
                y: (((b.y + 1.0) / 2.0) * vres) as i32,
            };
            let c = Point {
                x: (((c.x + 1.0) / 2.0) * hres) as i32,
                y: (((c.y + 1.0) / 2.0) * vres) as i32,
            };
            // println!("{:?}", a);
            // println!("{:?}", b);
            // println!("{:?}", c);
            Triangle { a, b, c }.draw(canvas, color);
        }
    }
    // println!("{}", scene.camera.horizontal_field_of_view());
    // for line in scene.unified_mesh {
    // plot_line(canvas, &line.a, &line.b, color);
    // }
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
pub fn geometry_pipeline(mut scene: Scene) -> RgbImage {
    application(&scene);
    geometry(&mut scene);
    let horizontal_res = scene.camera.sensor.horizontal_res;
    let vertical_res = scene.camera.sensor.vertical_res;
    let mut canvas: RgbImage = ImageBuffer::new(horizontal_res, vertical_res);
    render(&mut canvas, scene);
    canvas
}
