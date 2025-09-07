// use image::{ImageBuffer, Rgb, RgbImage};
use image::{Rgb, RgbImage};
use ndarray::Array1;

use crate::{
    geometry::transformations::{compile_transforms, Transform},
    material::ShaderNode,
};
// use crate::ray_tracing::rendering_equation::BRDF;

/// geometry defining spatial surface
#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub position: Vector,
    pub uv_coord: (f32, f32),
    pub shader: ShaderNode,
    // pub x: f32,
    // pub y: f32,
    // pub z: f32,
}

pub const ORIGIN: Vector = Vector {
    x: 0.,
    y: 0.,
    z: 0.,
};

impl Vertex {
    // pub fn to_point(&self) -> Point {
    //     let factor: f32 = self.z;
    //     return Point {
    //         x: (self.x.round() / factor) as i32,
    //         y: (self.y.round() / factor) as i32,
    //     };
    // }
    pub fn _add(&mut self, other: &Vector) {
        self.position.x += other.x;
        self.position.y += other.y;
        self.position.z += other.z;
    }
    pub fn _as_array(&self) -> [f32; 3] {
        [self.position.x, self.position.y, self.position.z]
    }
    pub fn as_homogenous_array(&self) -> [f32; 4] {
        [self.position.x, self.position.y, self.position.z, 1.0]
    }
    pub fn _inv(&self) -> Vector {
        let x = self.position.x * -1.0;
        let y = self.position.y * -1.0;
        let z = self.position.z * -1.0;
        Vector { x, y, z }
    }
    pub fn as_vector(&self) -> Vector {
        let x = self.position.x;
        let y = self.position.y;
        let z = self.position.z;
        Vector { x, y, z }
    }
}

// impl std::ops::Mul<Vertex> for f32 {
//     type Output = Vertex;
//     fn mul(self, rhs: Vertex) -> Vertex {
//         Vertex {
//             x: rhs.x * self,
//             y: rhs.y * self,
//             z: rhs.z * self,
//         }
//     }
// }

// impl std::ops::Add<Vertex> for Vertex {
//     type Output = Vertex;
//     fn add(self, rhs: Vertex) -> Vertex {
//         Vertex {
//             x: rhs.x + self.x,
//             y: rhs.y + self.y,
//             z: rhs.z + self.z,
//         }
//     }
// }

const VERTEX: Vertex = Vertex {
    position: ORIGIN,
    uv_coord: (0., 0.),
    shader: ShaderNode::Void,
};

pub fn vertex(x: f32, y: f32, z: f32) -> Vertex {
    return Vertex {
        position: vector(x, y, z),
        ..VERTEX
    };
}

pub fn vertex_from_array(arr: Array1<f32>) -> Vertex {
    let w = arr[3];
    let position_vector = vector(arr[0] / w, arr[1] / w, arr[2] / w);
    return Vertex {
        position: position_vector,
        ..VERTEX
    };
}

/// direction and magnitude in 3D space
/// THIS IS EXPLICITLY A SPATIAL REPRESENTATION. DO NOT USE THIS FOR NON CARTESIAN DATA
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// fn square(x:f32) -> f32 {
//     x**2
//     x.pow(2)
// }
impl std::ops::Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vector {
    pub fn translate(&mut self, offset: Vector) {
        self.x += offset.x;
        self.y += offset.y;
        self.z += offset.z;
    }
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
    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn minus(&self, other: &Vector) -> Vector {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector { x, y, z }
    }
    pub fn times(&self, factor: f32) -> Vector {
        let x = self.x * factor;
        let y = self.y * factor;
        let z = self.z * factor;
        Vector { x, y, z }
    }
    pub fn is_origin(&self) -> bool {
        if self.x == 0. && self.y == 0. && self.z == 0. {
            return true;
        }
        false
    }
    pub fn to(&self, head: Vector) -> Vector {
        head.minus(self)
    }
    pub fn inv(&self) -> Vector {
        let x = self.x * -1.0;
        let y = self.y * -1.0;
        let z = self.z * -1.0;
        Vector { x, y, z }
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
    pub fn _draw(&self, _canvas: &mut RgbImage, _color: Rgb<u8>) {
        // note: replace the color type before using
        let _a: &Vector = &self.a.position;
        let _b: &Vector = &self.b.position;
        let _c: &Vector = &self.c.position;

        //     plot_line(canvas, a, b, color);
        //     plot_line(canvas, b, c, color);
        //     plot_line(canvas, c, a, color);
    }
    /// ONLY WORKS FOR TRIGON
    pub fn get_normal(&self) -> Vector {
        let a = &self.b.as_vector().minus(&self.a.as_vector());
        let b = &self.c.as_vector().minus(&self.a.as_vector());
        // let c = &self.c;
        let x = a.y * b.z - a.z * b.y;
        let y = a.z * b.x - a.x * b.z;
        let z = a.x * b.y - a.y * b.x;

        let mut out = vector(x, y, z);
        out.norm();
        out
    }
}

pub fn polygon(a: Vertex, b: Vertex, c: Vertex) -> Polygon {
    Polygon { a, b, c }
}
// pub fn polygon<'a>(a: &'a Vertex, b: &'a Vertex, c: &'a Vertex) -> Polygon {
//     Polygon { a, b, c }
// }

pub struct _Line3D {
    pub a: Vertex,
    pub b: Vertex,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    // pub position: Vector,
    // pub orientation: ,
    pub vertices: Vec<Vertex>, // a mesh owns it's vertex information
    // pub polygons: Vec<Polygon>, // it also owns it's polygon information
    pub polygons: Vec<Vec<usize>>,
    pub output_vertices: Vec<Vertex>,
    transform_log: Vec<Transform>,
}
impl Mesh {
    /// I need to learn matrix math for this one
    /// transforms are kept as a list of transforms to be done, which is much more efficient
    pub fn apply_transformations(&mut self) {
        let transform = compile_transforms(&self.transform_log);

        self.output_vertices = self.vertices.clone();
        self.output_vertices = transform.process(self.output_vertices.clone());
    }
    pub fn add_transform(&mut self, transform: Transform) -> () {
        self.transform_log.push(transform);
    }
    pub fn _get_transforms(&self) -> &Vec<Transform> {
        return &self.transform_log;
    }
}
pub fn mesh(vertices: Vec<Vertex>, polygons: Vec<Vec<usize>>) -> Mesh {
    Mesh {
        vertices,
        polygons,
        output_vertices: Vec::new(),
        transform_log: Vec::new(),
    }
}
pub fn _unit_cube() -> Mesh {
    let a: Vertex = vertex(-1.0, -1.0, -1.0); //0  left down far from above
    let b: Vertex = vertex(1.0, -1.0, -1.0); //1 right down far from above
    let c: Vertex = vertex(-1.0, 1.0, -1.0); //2  left   up far from above
    let d: Vertex = vertex(1.0, 1.0, -1.0); //3 right   up far from above
    let e: Vertex = vertex(-1.0, -1.0, 1.0); //4  left down    close from above
    let f: Vertex = vertex(1.0, -1.0, 1.0); //5 right down    close from above
    let g: Vertex = vertex(-1.0, 1.0, 1.0); //6  left   up    close from above
    let h: Vertex = vertex(1.0, 1.0, 1.0); //7 right   up    close from above

    let polygons = vec![
        vec![0, 2, 1], // bottom 0123
        vec![1, 2, 3], // bottom
        vec![4, 5, 6], // close 4567
        vec![6, 5, 7], // close
        vec![0, 1, 4], // down 01 45
        vec![4, 1, 5], // down
        vec![2, 6, 3], // up 23 67
        vec![3, 6, 7], // up
        vec![0, 4, 2], // right 0 2 4 6
        vec![2, 4, 6], // right
        vec![1, 3, 5], // left 1 3 5 7
        vec![5, 3, 7], // left
    ];
    let mesh = Mesh {
        vertices: vec![a, b, c, d, e, f, g, h],
        polygons,
        output_vertices: Vec::new(),
        transform_log: Vec::new(),
    };
    return mesh;
}

pub fn sample_mesh() -> Mesh {
    let a: Vertex = vertex(0., 0., 0.); //0  left down bottom from above
    let b: Vertex = vertex(1., 0., 0.); //1 right down bottom from above
    let c: Vertex = vertex(0., 1., 0.); //2  left   up bottom from above

    let polygons = vec![
        vec![0, 1, 2], // bottom 0123
    ];
    let mesh = Mesh {
        vertices: vec![a, b, c],
        polygons,
        output_vertices: Vec::new(),
        transform_log: Vec::new(),
    };
    return mesh;
}

/// 2D line
pub struct _Line {
    pub a: Point,
    pub b: Point,
}

/// 2D
#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct _BoundingBox2D {
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
    pub fn _get_bounding_box(&self) -> _BoundingBox2D {
        let x = *vec![self.a.x, self.b.x, self.c.x].iter().min().unwrap();
        let y = *vec![self.a.y, self.b.y, self.c.y].iter().min().unwrap();
        let min = Point { x, y };
        let x = *vec![self.a.x, self.b.x, self.c.x].iter().max().unwrap();
        let y = *vec![self.a.y, self.b.y, self.c.y].iter().max().unwrap();
        let max = Point { x, y };
        _BoundingBox2D { min, max }
    }
}
pub fn triangle(a: &Vertex, b: &Vertex, c: &Vertex) -> Triangle {
    let a = Point {
        x: a.position.x as i32,
        y: a.position.y as i32,
    };
    let b = Point {
        x: b.position.x as i32,
        y: b.position.y as i32,
    };
    let c = Point {
        x: c.position.x as i32,
        y: c.position.y as i32,
    };

    Triangle { a, b, c }
}

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::geometry::primitives::vector;

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn test_negative_vectors() {
        let myvec = vector(1., 2., 3.);
        let anothervec = -myvec;
        assert_eq!(anothervec, vector(-1., -2., -3.));
    }
}
