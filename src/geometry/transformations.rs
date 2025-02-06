use crate::{
    camera::Camera,
    geometry::primitives::{vertex_from_array, Vector, Vertex},
};
use ndarray::{arr1, arr2, Array2};

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
    let _vfactor = hfactor / camera.sensor.aspect_ratio(); // for every meter away

    let matrix = arr2(&[
        //*x + *y + *z + *1
        [1.0, 0.0, 0.0, 0.0], // composes the x value
        [0.0, 1.0, 0.0, 0.0], // composes the y value
        [0.0, 0.0, 1.0, 0.0], // composes the z value
        [0.0, 0.0, hfactor, 0.0],
    ]);
    Transform { matrix }
}

// 2024-06-23 I switched which sin was negative because of what wikipedia used
pub fn _build_x_rotation_transform(θ: f32) -> Transform {
    let matrix = arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos(θ), -sin(θ), 0.0],
        [0.0, sin(θ), cos(θ), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    Transform { matrix }
}

// 2024-06-23 I switched which sin was negative because of what wikipedia used
pub fn build_y_rotation_transform(θ: f32) -> Transform {
    let matrix = arr2(&[
        [cos(θ), 0.0, sin(θ), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin(θ), 0.0, cos(θ), 0.0],
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

// quaternions! See my notes
// assumes radians
pub fn build_arbitrary_rotation_transform(θ: f32, mut axis: Vector) -> Transform {
    axis.norm();
    let q0 = cos(θ / 2.);
    let factor = sin(θ / 2.);
    let q1 = axis.x * factor;
    let q2 = axis.y * factor;
    let q3 = axis.z * factor;
    let matrix = arr2(&[
        [
            sq(q0) + sq(q1) - sq(q2) - sq(q3),
            2. * q1 * q2 - 2. * q0 * q3,
            2. * q1 * q3 + 2. * q0 * q2,
            0.,
        ],
        [
            2. * q1 * q2 + 2. * q0 * q3,
            sq(q0) - sq(q1) + sq(q2) - sq(q3),
            2. * q2 * q3 - 2. * q0 * q1,
            0.,
        ],
        [
            2. * q1 * q3 - 2. * q0 * q2,
            2. * q2 * q3 + 2. * q0 * q1,
            sq(q0) - sq(q1) - sq(q2) + sq(q3),
            0.,
        ],
        [0., 0., 0., 1.],
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

/// squares the number
fn sq(a: f32) -> f32 {
    // 2 << a
    f32::powi(a, 2)
}

fn _zero_tiny(x: &f32) -> f32 {
    if x.abs() < 1e-7 {
        return 0.0;
    }
    *x
}

fn _round_6(x: &f32) -> f32 {
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

    use crate::geometry::primitives::{vector, vertex};

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
        let myvertex = arr1(&myvertex._as_array());
        let myvertex = myvertex.map(_round_6);
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
        vertex = vertex.map(_round_6); // because of rounding errors
        vertex = vertex.map(_zero_tiny); // because of other errors
        assert_eq!(arr1(&[-2.0, 1.0, 3.0, 1.0]), vertex);
    }
    #[test]
    fn verify_z_rotation_implementation() {
        let transform = build_z_rotation_transform(90f32.to_radians());
        let myvertex = arr1(&[1.0, 2.0, 3.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        let myvertex = arr1(&myvertex._as_array());
        let myvertex = myvertex.map(_round_6);
        assert_eq!(arr1(&[-2.0, 1.0, 3.0]), myvertex);
    }
    #[test]
    fn verify_quaternions() {
        let vector = vector(1., 0., 0.);
        let angle = 90f32.to_radians();

        let transform = build_arbitrary_rotation_transform(angle, vector);

        let vertex = arr1(&[0., 1., 0., 1.]);
        let vertex_list = vec![vertex_from_array(vertex)];
        let vertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        let myvertex_p = arr1(&vertex._as_array());
        let myvertex_p = myvertex_p.map(_round_6);
        println!("{:?}", myvertex_p);
        assert_eq!(arr1(&[0., 0., 1.]), myvertex_p);
    }

    #[test]
    fn verify_projection_implementation() {
        let lens = Lens {
            _aperture: 30.0,
            focal_length: 18.0,
            _focus_distance: 2.0,
        };
        let sensor = Sensor {
            width: 36.0,
            horizontal_res: 400,
            vertical_res: 300,
        };
        let camera = Camera {
            position: vector(0.0, 0.0, 0.0),
            // orientation: Polar
            lens,
            sensor,
            _near_clipping_plane: 1e-1,
            _far_clipping_plane: 1e6,
            _exposure_time: 1.,
        };
        let transform = build_projection_transform(&camera);
        println!("{:?}", camera.horizontal_field_of_view());
        println!("{:?}", transform);
        let myvertex = arr1(&[10.0, 10.0, 5.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        println!("{:?}", myvertex);
        let myvertex = arr1(&myvertex._as_array());
        let myvertex = myvertex.map(_round_6);
        assert_eq!(arr1(&[2.0, 2.0, 1.0]), myvertex);

        let myvertex = arr1(&[015.0, 0.0, 3.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        println!("{:?}", myvertex);
        let myvertex = arr1(&myvertex._as_array());
        let myvertex = myvertex.map(_round_6);
        assert_eq!(arr1(&[05.0, 0.0, 1.0]), myvertex);

        let myvertex = arr1(&[10.0, 10.0, 1.0, 1.0]);
        let vertex_list = vec![vertex_from_array(myvertex)];
        let myvertex = transform.process(vertex_list).into_iter().nth(0).unwrap();
        println!("{:?}", myvertex);
        let myvertex = arr1(&myvertex._as_array());
        let myvertex = myvertex.map(_round_6);
        assert_eq!(arr1(&[10.0, 10.0, 1.0]), myvertex);
    }
}
