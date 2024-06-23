use std::vec;

use crate::primitives::{vector, Polygon, Ray, Vector};
use image::{Rgb, RgbImage};
use stopwatch::Stopwatch;

pub fn ray_polygon_intersection_test(ray: &Ray, polygon: &Polygon) -> bool {
    let mut probe = Stopwatch::start_new();
    let (b, I, dist) = probe_ray_polygon_intersection(ray, polygon);
    probe.stop();
    println!("  probe: {:?}", probe.elapsed());
    b
}

/// returns whether it intersects, the ray, and the distance
pub fn probe_ray_polygon_intersection(ray: &Ray, polygon: &Polygon) -> (bool, Vector, f32) {
    // println!("{:?}\n\n\n",polygon);
    // first, if the ray is parallel to the plane the polygon lies in, they do not intersect
    // you can also discard backfacing normals

    // then, find the intersection point
    // convert it from cartesian coordinates to barycentric coordinates

    // simple culling from there

    // ''' this function determines intersection and returns the barycentric
    // coordinates of the intersection point if there is a hit'''
    // # print(f'ray:{ray}')
    // # print(f'points:{points}')
    // # print()
    // I = ray_plane_intersection(ray, points)  # the point of intersection
    let result = ray_plane_intersection(ray, polygon);
    if result.is_none() {
        return (false, vector(0., 0., 0.), 0.);
    }

    let (I,dist) = result.unwrap();

    // # print(f'I:{I}')
    // if np.all(I == 0):
    //     return False
    if I.is_origin() {
        return (false, I, dist);
        // return Rgb([0,255,0]);
    }
    // A, B, C = points
    let A = polygon.a.as_vector();

    let B = polygon.b.as_vector();

    let C = polygon.c.as_vector();

    // AB = B - A
    let AB = B.minus(&A);
    // CB = B - C
    let CB = B.minus(&C);
    // v = AB - project_vector(CB, AB)
    let v = AB.minus(&project_vector(&CB, &AB));
    // AI = I - A
    let AI = I.minus(&A);
    // # print(f'AI:{AI}')
    // # a is the barycentric coordinate component, if 0, I is at A, if bigger than one, it's outside the triangle
    // # print( 1 - project_vector(v, AI) / project_vector(v, AB))
    // # print('v',v)
    // a = 1 - (dot(v, AI) / dot(v, AB))
    let a = 1. - (v.dot(&AI) / v.dot(&AB));
    // # print(f'a:{a}')

    // BC = C - B  # the mother line
    let BC = C.minus(&B);
    // AC = C - A
    let AC = C.minus(&A);

    // v = BC - project_vector(AC, BC)
    let v = BC.minus(&project_vector(&AC, &BC));

    // BI = I - B
    let BI = I.minus(&B);

    // # print(f'BI:{BI}')
    // # barycentric coordinate for b
    // b = 1 - (dot(v, BI) / dot(v, BC))
    let b = 1. - (v.dot(&BI) / v.dot(&BC));

    // # print(f'b:{b}')

    // CA = A - C  # motherline
    let CA = A.minus(&C);

    // BA = A - B
    let BA = A.minus(&B);

    // # finding the CB instead of the CA at the star of the expression took an hour
    // v = CA - project_vector(BA, CA)
    let v = CA.minus(&project_vector(&BA, &CA));

    // CI = I - C
    let CI = I.minus(&C);

    // # print(f'CI:{CI}')
    // # barycentric coordinate for c
    // # print('v',v)
    // c = 1 - (dot(v, CI) / dot(v, CA))
    let c = 1. - (v.dot(&CI) / v.dot(&CA));

    // # print(f'c:{c}')

    // barycentric_coordinates = np.array([a, b, c])
    // # print(f'barycentric_coordinates:\n{barycentric_coordinates}')
    // # print('\n')
    // if np.any(barycentric_coordinates < 0):
    //     return False
    if a < 0. || b < 0. || c < 0. {
        return (false, I, dist);
        // println!("{},{},{} for {:?}",a,b,c,ray.direction);
        // return Rgb([255,0,0]);
    }
    // if np.any(barycentric_coordinates > 1):
    //     return False
    if a > 1. || b > 1. || c > 1. {
        return (false, I, dist);
        // return Rgb([0,0,255]);
    }

    // return True
    (true, I, dist)
    // return Rgb([255,255,255]);
}

fn project_vector(a: &Vector, b: &Vector) -> Vector {
    // return (dot(a, b) / dot(a, a)) * a
    a.times(a.dot(&b) / a.dot(&a))
}

fn ray_plane_intersection(ray: &Ray, polygon: &Polygon) -> Option<(Vector,f32)> {
    let origin = vector(0., 0., 0.);
    let ray = &ray.direction;
    // ray_origin = np.array([0, 0, 0])
    let ray_origin = vector(0., 0., 0.);
    // N = normal_of_polygon(points)
    let N = polygon.get_normal();

    // print!("the normal of {:?} is ", polygon);
    // println!("{:?}\n\n", N);

    // # print(f'planes normal: {N}')
    // # print(f'ray:{ray}')
    // # print(dot(ray, N) )
    // if dot(ray, N) > 0:
    //     # print('the plane normal is facing away from the ray')
    //     return np.array([0., 0., 0.])
    if ray.dot(&N) > 0. {
        return None;
        // return origin;
    }
    if ray.dot(&N).abs() <= 1e-4 {
        return None;
        // return origin;
    }
    // if abs(dot(ray, N)) <= 1e-4:
    //     # print('the plane normal and ray are at right angles, no intersection is possible')
    //     return np.array([0., 0., 0.])
    // C = points[0]  # any point that lies on the shared plane
    let C: Vector = polygon.a.as_vector();
    // V = ray
    let V = &ray;

    // W = C - ray_origin
    let W = C.minus(&ray_origin);
    // k = dot(W, N) / dot(V, N)
    let k: f32 = W.dot(&N) / V.dot(&N);
    // # k is the multiplier with the ray to reach I from the ray_origin
    // # if the ray is a unit vector it is the distance from ray_origin to the intersection

    // I = ray_origin + k * V
    let I = V.times(k);
    // # print(f'k: {k}')
    // if k < 0:
    //     return np.array([0., 0., 0.])  # ray is facing away from plaen
    if k < 0. {
        return None;
        // return origin;
    }
    // if k == 0:
    //     return np.array([0., 0., 0.])  # ray is on plane I think
    if k == 0. {
        return None;
        // return origin;
    }
    // # print ('distance:',k)
    return Some((I,k));
    // return I  # returns the intersection point
}

#[cfg(test)]
mod tests {
    use crate::{
        primitives::{polygon, vertex},
        scene::simple_scene,
    };

    use super::*;

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn ray_math() {
        // let a = vertex(x, y, z);
        // let a = vertex(x, y, z);
        // let a = vertex(x, y, z);

        // polygon(a, b, c)
        assert_eq!(1, 1); //
    }
}
