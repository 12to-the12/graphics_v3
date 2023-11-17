use std::ops::Mul;

use crate::primitives::{vector, vertex, vertex_from_array, Angle, Vector, Vertex};

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
            let transform = &self.matrix;
            let out_vertex = transform.dot(&vertex); // the resulting vertex

            out.push(vertex_from_array(out_vertex)); // output a vertex
        }
        return out;
    }
}

pub fn build_identity_matrix() -> Transform {
    let matrix = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}
pub fn build_translation_matrix(translation: Vector) -> Transform {
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
pub fn build_scale_matrix(scale: Vector) -> Transform {
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
// fn build_x_rotation_matrix(θ: f32) -> Array2<f32> {
//     return arr2(&[
//         [0.0, 0.0, 0.0, 0.0],
//         [0.0, cos(θ), sin(θ), 0.0],
//         [0.0, -sin(θ), cos(θ), 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ]);
// }
// fn build_y_rotation_matrix(θ: f32) -> Array2<f32> {
//     return arr2(&[
//         [cos(θ), 0.0, -sin(θ), 0.0],
//         [0.0, 1.0, 0.0, 0.0],
//         [sin(θ), 0.0, cos(θ), 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ]);
// }
// fn build_z_rotation_matrix(θ: f32) -> Array2<f32> {
//     return arr2(&[
//         [cos(θ), -sin(θ), 0.0, 0.0],
//         [sin(θ), cos(θ), 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ]);
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
    let x = *x;
    (x * 1e6).round() / 1e6
}

pub fn compile_transforms(transforms: &Vec<Transform>) -> Transform {
    let mut composure = build_identity_matrix();
    for transform in transforms.iter().rev() {
        // this needs to be reversed
        let matrix = composure.matrix.dot(&transform.matrix);

        composure.matrix = matrix;
    }
    composure
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use ndarray::linalg::Dot;

    use crate::primitives::vertex;

    use super::*;

    #[test]
    fn test_translate() {
        let myvertex = vertex(1.0, 2.0, 3.0);
        let myvertexlist = vec![myvertex];
        // let offset = vector(-1.0, 3.0, -7.3);
        // let mytransform = Transform::Translation(offset);
        // apply_transform(&mut myvertexlist, mytransform);
        let mytransform = build_translation_matrix(vector(-1.0, 3.0, -7.3));
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
    fn transform_matrix_integrity() {
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
}
