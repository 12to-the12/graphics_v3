use std::ops::Mul;

use crate::{
    camera::Camera,
    primitives::{vector, vertex, vertex_from_array, Angle, Vector, Vertex},
};
use ndarray::{arr1, arr2, Array1, Array2, Axis};

#[derive(Clone, Debug)]
pub struct Transform {
    pub matrix: Array2<f32>,
}
impl Transform {
    pub fn process(&self, vertices: Vec<Vertex>) -> Vec<Vertex> {
        let mut out = Vec::new();
        for vertex in vertices.into_iter() {
            // processes vertex by vertex
            let vertex = vertex.as_homogenous_array();
            let vertex = arr1(&vertex);
            // println!("in vertex: {:?}", vertex);
            let transform = &self.matrix;
            let out_vertex = transform.dot(&vertex); // the resulting vertex
                                                     // println!("out vertex: {:?}\n\n", out_vertex);
            out.push(vertex_from_array(out_vertex)); // output a vertex
        }
        return out;
    }
}

pub fn build_identity_transform() -> Transform {
    let matrix = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}
pub fn build_translation_transform(translation: Vector) -> Transform {
    let x = translation.x;
    let y = translation.y;
    let z = translation.z;
    let matrix = arr2(&[
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}
pub fn build_scale_transform(scale: Vector) -> Transform {
    let x = scale.x;
    let y = scale.y;
    let z = scale.z;
    let matrix = arr2(&[
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}
/// still trying to figure this one out
pub fn build_projection_transform(camera: &Camera) -> Transform {
    let hfov = camera.horizontal_field_of_view();
    let hfactor = hfov / 90.0; // for every meter away
    let vfactor = hfactor / camera.sensor.aspect_ratio(); // for every meter away

    let matrix = arr2(&[
        //*x + *y + *z + *1
        [1.0, 0.0, 0.0, 0.0], // composes the x value
        [0.0, 1.0, 0.0, 0.0], // composes the y value
        [0.0, 0.0, 1.0, 0.0], // composes the z value
        [0.0, 0.0, hfactor, 0.0],
    ]);
    Transform { matrix }
}
pub fn build_x_rotation_transform(θ: f32) -> Transform {
    let matrix = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos(θ), sin(θ), 0.0],
        [0.0, -sin(θ), cos(θ), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}
pub fn build_y_rotation_transform(θ: f32) -> Transform {
    let matrix = arr2(&[
        [cos(θ), 0.0, -sin(θ), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sin(θ), 0.0, cos(θ), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}
pub fn build_z_rotation_transform(θ: f32) -> Transform {
    let matrix = arr2(&[
        [cos(θ), -sin(θ), 0.0, 0.0],
        [sin(θ), cos(θ), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}

// /// the input vector is the shear coefficient along each axis,
// /// where a value of 0 does nothing and a value of one
// pub fn build_shear_matrix(θ: f32) -> Transform {
//     let matrix = arr2(&[
//         [cos(θ), -sin(θ), 0.0, 0.0],
//         [sin(θ), cos(θ), 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ]);
//     Transform { matrix }
// }

fn cos(θ: f32) -> f32 {
    θ.cos()
}

fn sin(θ: f32) -> f32 {
    θ.sin()
}

fn zero_tiny(x: &f32) -> f32 {
    if x.abs() < 1e-7 {
        return 0.0;
    }
    *x
}

fn round_6(x: &f32) -> f32 {
    // *x * 1e7 / 1e7
    let factor = 1e5;
    let x = *x;
    (x * factor).round() / factor
}

pub fn compile_transforms(transforms: &Vec<Transform>) -> Transform {
    let mut composure = build_identity_transform();
    for transform in transforms.iter().rev() {
        // this needs to be reversed
        let matrix = composure.matrix.dot(&transform.matrix);

        composure.matrix = matrix;
    }
    composure
}

#[cfg(test)]
mod tests {
    use crate::camera::{Camera, Lens, Sensor};

    use std::ops::Mul;

    use ndarray::linalg::Dot;

    use crate::{primitives::vertex, scene::simple_scene};

    use super::*;

    #[test]
    fn test_translate() {
        let myvertex = vertex(1.0, 2.0, 3.0);
        let myvertexlist = vec![myvertex];
        // let offset = vector(-1.0, 3.0, -7.3);
        // let mytransform = Transform::Translation(offset);
        // apply_transform(&mut myvertexlist, mytransform);
        let mytransform = build_translation_transform(vector(-1.0, 3.0, -7.3));
        let myvertexlist = mytransform.process(myvertexlist);
        assert_eq!(myvertexlist[0], vertex(0.0, 5.0, -4.3));
        assert_ne!(myvertexlist[0], vertex(1.0, 2.0, 3.0));
    }
    /// https://vitaminac.github.io/Matrices-in-Computer-Graphics/#Translation-Matrix
    #[test]
    fn identity_matrix_integrity() {
        let transform = arr2(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let mut vertex = arr1(&[-169.0, 0.0, 729.7, 1.0]);
        vertex = transform.dot(&vertex);
        assert_eq!(arr1(&[-169.0, 0.0, 729.7, 1.0]), vertex);
    }
    #[test]
    fn verify_identity_implementation() {
        let transform = build_identity_transform();
        let myvertex = arr1(&[-169.0, 0.0, 729.7, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        assert_eq!(vertex(-169.0, 0.0, 729.7), myvertex);
    }
    #[test]
    fn translation_matrix_integrity() {
        let transform = arr2(&[
            [1.0, 0.0, 0.0, 1.1],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, -7.6],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let mut vertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        vertex = transform.dot(&vertex);
        assert_eq!(arr1(&[2.1, 2.0, -4.6, 1.0]), vertex);
    }
    #[test]
    fn verify_translation_implementation() {
        let transform = build_translation_transform(vector(1.1, 0.0, -7.6));
        let myvertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        assert_eq!(vertex(2.1, 2.0, -4.6), myvertex);
    }
    #[test]
    fn scale_matrix_integrity() {
        let transform = arr2(&[
            [3.0, 0.0, 0.0, 0.0],
            [0.0, -2.0, 0.0, 0.0],
            [0.0, 0.0, 2.20, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let mut vertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        vertex = transform.dot(&vertex);
        vertex = vertex.map(|x| *x * 10.0 / 10.0); // because of rounding errors
        assert_eq!(arr1(&[3.0, -4.0, 6.6, 1.0]), vertex);
    }
    #[test]
    fn verify_scale_implementation() {
        let transform = build_scale_transform(vector(3.0, -2.0, 2.2));
        let myvertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        let myvertex = arr1(&myvertex.as_array());
        let myvertex = myvertex.map(round_6);
        assert_eq!(arr1(&[3.0, -4.0, 6.6]), myvertex);
    }

    #[test]
    fn z_rotation_matrix_integrity() {
        let mut θ: f32 = 90.0;
        θ = θ.to_radians();

        let transform = arr2(&[
            [cos(θ), -sin(θ), 0.0, 0.0],
            [sin(θ), cos(θ), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let mut vertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        vertex = transform.dot(&vertex);
        // vertex = vertex.map(|x| *x * 10.0 / 10.0); // because of rounding errors
        vertex = vertex.map(round_6); // because of rounding errors
        vertex = vertex.map(zero_tiny); // because of other errors
        assert_eq!(arr1(&[-2.0, 1.0, 3.0, 1.0]), vertex);
    }
    #[test]
    fn verify_rotation_implementation() {
        let transform = build_z_rotation_transform(90f32.to_radians());
        let myvertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        let myvertex = arr1(&myvertex.as_array());
        let myvertex = myvertex.map(round_6);
        assert_eq!(arr1(&[-2.0, 1.0, 3.0]), myvertex);
    }

    #[test]
    fn verify_projection_implementation() {
        let lens = Lens {
            aperture: 30.0,
            focal_length: 18.0,
            focus_distance: 2.0,
        };
        let sensor = Sensor {
            width: 36.0,
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
        let transform = build_projection_transform(&camera);
        println!("{:?}",camera.horizontal_field_of_view());
        println!("{:?}", transform);
        let myvertex = arr1(&[10.0, 10.0, 5.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        println!("{:?}", myvertex);
        let myvertex = arr1(&myvertex.as_array());
        let myvertex = myvertex.map(round_6);
        assert_eq!(arr1(&[2.0, 2.0, 1.0]), myvertex);

        let myvertex = arr1(&[015.0, 0.0, 3.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        println!("{:?}", myvertex);
        let myvertex = arr1(&myvertex.as_array());
        let myvertex = myvertex.map(round_6);
        assert_eq!(arr1(&[05.0, 0.0, 1.0]), myvertex);

        let myvertex = arr1(&[10.0, 10.0, 1.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        println!("{:?}", myvertex);
        let myvertex = arr1(&myvertex.as_array());
        let myvertex = myvertex.map(round_6);
        assert_eq!(arr1(&[10.0, 10.0, 1.0]), myvertex);
    }
}
