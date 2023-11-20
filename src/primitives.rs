// use image::{ImageBuffer, Rgb, RgbImage};
use image::{Rgb, RgbImage};
use ndarray::Array1;

use crate::{coordinate_space::Orientation, line_plotting::plot_line};

use crate::coordinate_space::Polar;
use crate::transformations::{build_translation_transform, compile_transforms, Transform};

/// point in 3D space
#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn to_point(&self) -> Point {
        let factor: f32 = self.z;
        return Point {
            x: (self.x.round() / factor) as i32,
            y: (self.y.round() / factor) as i32,
        };
    }
    pub fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn as_homogenous_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, 1.0]
    }
    pub fn inv(&self) -> Vector {
        let x = self.x * -1.0;
        let y = self.y * -1.0;
        let z = self.z * -1.0;
        Vector { x, y, z }
    }
}

pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    return Vertex { x, y, z };
}

pub fn vertex_from_array(arr: Array1<f32>) -> Vertex {
    let w = arr[3];
    return Vertex {
        x: arr[0] / w,
        y: arr[1] / w,
        z: arr[2] / w,
    };
}
/// direction and magnitude in 3D space
#[derive(Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// fn square(x:f32) -> f32 {
//     x**2
//     x.pow(2)
// }
impl Vector {
    /// magnitude of the vector
    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
    /// normalizes the vector
    pub fn norm(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }
    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn as_homogenous_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, 1.0]
    }
}

pub fn vector(x: f32, y: f32, z: f32) -> Vector {
    return Vector { x, y, z };
}

#[derive(Clone, Debug)]
pub struct Polygon {
    // <'a>
    // pub a: &'a Vertex,
    // pub b: &'a Vertex,
    // pub c: &'a Vertex,
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Polygon {
    pub fn draw(&self, canvas: &mut RgbImage, color: Rgb<u8>) {
        // note: replace the color type before using
        let a: &Point = &self.a.to_point();
        let b: &Point = &self.b.to_point();
        let c: &Point = &self.c.to_point();

        plot_line(canvas, a, b, color);
        plot_line(canvas, b, c, color);
        plot_line(canvas, c, a, color);
    }
}

pub fn polygon(a: Vertex, b: Vertex, c: Vertex) -> Polygon {
    Polygon { a, b, c }
}
// pub fn polygon<'a>(a: &'a Vertex, b: &'a Vertex, c: &'a Vertex) -> Polygon {
//     Polygon { a, b, c }
// }

pub struct Line3D {
    pub a: Vertex,
    pub b: Vertex,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub position: Vector,
    // pub orientation: ,
    pub vertices: Vec<Vertex>, // a mesh owns it's vertex information
    // pub polygons: Vec<Polygon>, // it also owns it's polygon information
    pub polygons: Vec<Vec<usize>>,
    pub output_vertices: Vec<Vertex>,
    transform_log: Vec<Transform>,
}
impl Mesh {
    /// I need to learn matrix math for this one
    /// for now we'll keep it to simple translations
    /// transforms are kept as a list of transforms to be done, which is much more efficient
    pub fn apply_transformations(&mut self) {
        let transform = compile_transforms(&self.transform_log);
        // println!("{:?}", transform.matrix);
        // println!("compiled transform:{:?}", transform);

        self.output_vertices = self.vertices.clone();
        self.output_vertices = transform.process(self.output_vertices.clone());
    }
    pub fn add_transform(&mut self, transform: Transform) -> () {
        self.transform_log.push(transform);
    }
    pub fn get_transforms(&self) -> &Vec<Transform> {
        return &self.transform_log;
    }
}

pub fn unit_cube(position: Vector) -> Mesh {
    // let a: Vertex = vertex(-1.0, -1.0, -1.0); //  left down bottom from above
    // let b: Vertex = vertex(1.0, -1.0, -1.0); // right down bottom from above
    // let c: Vertex = vertex(-1.0, 1.0, -1.0); //  left   up bottom from above
    // let d: Vertex = vertex(1.0, 1.0, -1.0); // right   up bottom from above
    // let e: Vertex = vertex(-1.0, -1.0, 1.0); //  left down    top from above
    // let f: Vertex = vertex(1.0, -1.0, 1.0); // right down    top from above
    // let g: Vertex = vertex(-1.0, 1.0, 1.0); //  left   up    top from above
    // let h: Vertex = vertex(1.0, 1.0, 1.0); // right   up    top from above

    let a: Vertex = vertex(-1.0, -1.0, -1.0); //0  left down bottom from above
    let b: Vertex = vertex(1.0, -1.0, -1.0); //1 right down bottom from above
    let c: Vertex = vertex(-1.0, 1.0, -1.0); //2  left   up bottom from above
    let d: Vertex = vertex(1.0, 1.0, -1.0); //3 right   up bottom from above
    let e: Vertex = vertex(-1.0, -1.0, 1.0); //4  left down    top from above
    let f: Vertex = vertex(1.0, -1.0, 1.0); //5 right down    top from above
    let g: Vertex = vertex(-1.0, 1.0, 1.0); //6  left   up    top from above
    let h: Vertex = vertex(1.0, 1.0, 1.0); //7 right   up    top from above

    let polygons = vec![
        vec![0, 1, 2], // bottom 0123
        vec![1, 2, 3], // bottom
        vec![4, 5, 6], // top 4567
        vec![5, 6, 7], // top
        vec![0, 1, 4], // down 01 45
        vec![1, 4, 5], // down
        vec![2, 3, 6], // up 23 67
        vec![3, 6, 7], // up
        vec![0, 2, 4], // right 0 2 4 6
        vec![2, 4, 6], // right
        vec![1, 3, 5], // left 1 3 5 7
        vec![3, 5, 7], // left
    ];
    let mesh = Mesh {
        position,
        vertices: vec![a, b, c, d, e, f, g, h],
        polygons,
        output_vertices: Vec::new(),
        transform_log: Vec::new(),
    };
    // mesh.polygons.push(polygon(&mesh.vertices[0], &mesh.vertices[0], &mesh.vertices[0]));
    // mesh.polygons.push(polygon(&e, &f, &g)); // top
    // mesh.polygons.push(polygon(&f, &g, &h)); // top
    // mesh.polygons.push(polygon(&a, &b, &c)); // bottom
    // mesh.polygons.push(polygon(&b, &c, &d)); // bottom
    // mesh.polygons.push(polygon(&c, &d, &g)); // up (+y) cdgh
    // mesh.polygons.push(polygon(&d, &g, &h)); // up (+y)
    // mesh.polygons.push(polygon(&a, &b, &e)); // down (-y) abef
    // mesh.polygons.push(polygon(&b, &e, &f)); // down (-y)
    // mesh.polygons.push(polygon(&a, &c, &e)); // left aceg
    // mesh.polygons.push(polygon(&c, &e, &g)); // left
    // mesh.polygons.push(polygon(&b, &d, &f)); // right bdfh
    // mesh.polygons.push(polygon(&d, &f, &h)); // right
    return mesh;
}

pub fn sample_mesh(position: Vector) -> Mesh {
    let a: Vertex = vertex(0.0, 0.0, 0.0); //0  left down bottom from above
    let b: Vertex = vertex(1.0, 0.0, 0.0); //1 right down bottom from above
    let c: Vertex = vertex(0.0, 1.0, 0.0); //2  left   up bottom from above

    let polygons = vec![
        vec![0, 1, 2], // bottom 0123
    ];
    let mesh = Mesh {
        position,
        vertices: vec![a, b, c],
        polygons,
        output_vertices: Vec::new(),
        transform_log: Vec::new(),
    };
    return mesh;
}

/// 2D line
pub struct Line {
    pub a: Point,
    pub b: Point,
}

/// 2D
#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct BoundingBox2D {
    pub min: Point,
    pub max: Point,
}
/// 2D
#[derive(Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    // fn points(&self) ->
    pub fn get_bounding_box(&self) -> BoundingBox2D {
        let x = *vec![self.a.x, self.b.x, self.c.x].iter().min().unwrap();
        let y = *vec![self.a.y, self.b.y, self.c.y].iter().min().unwrap();
        let min = Point { x, y };
        let x = *vec![self.a.x, self.b.x, self.c.x].iter().max().unwrap();
        let y = *vec![self.a.y, self.b.y, self.c.y].iter().max().unwrap();
        let max = Point { x, y };
        BoundingBox2D { min, max }
    }
}
pub fn triangle(a: &Vertex, b: &Vertex, c: &Vertex) -> Triangle {
    let a = Point {
        x: a.x as i32,
        y: a.y as i32,
    };
    let b = Point {
        x: b.x as i32,
        y: b.y as i32,
    };
    let c = Point {
        x: c.x as i32,
        y: c.y as i32,
    };

    Triangle { a, b, c }
}

#[derive(Clone, Debug)]
pub struct Angle(f32); // simple shit

pub struct Ray {
    pub position: Vector,
    pub direction: Vector,
}

pub fn ray(position: Vector, direction: Vector) -> Ray {
    direction.clone().norm();
    Ray {
        position,
        direction,
    }
}
