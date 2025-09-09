use crate::geometry::primitives::{Polygon, Ray, Vector};
use stopwatch::Stopwatch;

#[allow(non_snake_case)]
pub fn _ray_polygon_intersection_test(ray: &Ray, polygon: &Polygon) -> bool {
    let mut probe = Stopwatch::start_new();
    let (b, _I, _dist) = probe_ray_polygon_intersection(ray, polygon);
    probe.stop();
    b
}

/// returns whether it intersects, the ray, and the distance
#[allow(non_snake_case)]
pub fn probe_ray_polygon_intersection(ray: &Ray, polygon: &Polygon) -> (bool, Vector, f32) {
    // first, if the ray is parallel to the plane the polygon lies in, they do not intersect
    // you can also discard backfacing normals

    // then, find the intersection point
    // convert it from cartesian coordinates to barycentric coordinates

    // simple culling from there

    // ''' this function determines intersection and returns the barycentric
    // coordinates of the intersection point if there is a hit
    let result = ray_plane_intersection(ray, polygon);
    if result.is_none() {
        return (false, Vector::new(0., 0., 0.), 0.);
    }

    let (I, dist) = result.unwrap();

    if I.is_origin() {
        return (false, I, dist);
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
    // a is the barycentric coordinate component, if 0, I is at A, if bigger than one, it's outside the triangle
    let a = 1. - (v.dot(&AI) / v.dot(&AB));

    // BC = C - B  # the mother line
    let BC = C.minus(&B);
    // AC = C - A
    let AC = C.minus(&A);

    // v = BC - project_vector(AC, BC)
    let v = BC.minus(&project_vector(&AC, &BC));

    // BI = I - B
    let BI = I.minus(&B);

    // barycentric coordinate for b
    // b = 1 - (dot(v, BI) / dot(v, BC))
    let b = 1. - (v.dot(&BI) / v.dot(&BC));

    // CA = A - C  # motherline
    let CA = A.minus(&C);

    // BA = A - B
    let BA = A.minus(&B);

    // finding the CB instead of the CA at the star of the expression took an hour
    // v = CA - project_vector(BA, CA)
    let v = CA.minus(&project_vector(&BA, &CA));

    // CI = I - C
    let CI = I.minus(&C);

    // barycentric coordinate for c
    // c = 1 - (dot(v, CI) / dot(v, CA))
    let c = 1. - (v.dot(&CI) / v.dot(&CA));

    //     return False
    if a < 0. || b < 0. || c < 0. {
        return (false, I, dist);
    }
    if a > 1. || b > 1. || c > 1. {
        return (false, I, dist);
    }

    (true, I, dist)
}

fn project_vector(a: &Vector, b: &Vector) -> Vector {
    a.times(a.dot(&b) / a.dot(&a))
}

#[allow(non_snake_case)]
fn ray_plane_intersection(ray: &Ray, polygon: &Polygon) -> Option<(Vector, f32)> {
    let ray_origin = &ray.position;
    let ray: &Vector = &ray.direction;

    let N = polygon.get_normal();

    if ray.dot(&N) > 0. {
        return None;
    }
    if ray.dot(&N).abs() <= 1e-4 {
        return None;
    }
    // if abs(dot(ray, N)) <= 1e-4:
    //     # print('the plane normal and ray are at right angles, no intersection is possible')

    // C is defined as a point that lies on the plane
    let C: Vector = polygon.a.as_vector();
    // V = ray
    let V = &ray;

    // W = C - ray_origin
    let W = C.minus(&ray_origin);
    let k: f32 = W.dot(&N) / V.dot(&N);
    // k is the multiplier with the ray to reach I from the ray_origin
    // if the ray is a unit vector it is the distance from ray_origin to the intersection

    // I = ray_origin + k * V
    let I = *ray_origin + V.times(k);
    // if k < 0  ray is facing away from plane
    if k < 0. {
        return None;
    }
    // if k == 0 ray is on plane I think
    if k == 0. {
        return None;
    }
    return Some((I, k));
    // returns the intersection point
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::primitives::{Polygon, Ray, Vector, Vertex},
        ray_tracing::ray_polygon_intersection::probe_ray_polygon_intersection,
    };

    /// useful table: https://www.nikonians.org/reviews/fov-tables

    #[test]
    fn ray_polygon_test_simple_polygon() {
        let polygon = Polygon::new(
            Vertex::new(2.0, 0.0, -2.0),
            Vertex::new(0.0, 2.0, -2.0),
            Vertex::new(0.0, 0.0, -2.0),
        );
        let ray = Ray::new(Vector::new(1.0, 1.0, 0.), Vector::new(0., 0.0, -1.0));
        println!("{:?}", polygon.get_normal());
        println!("{:?}", probe_ray_polygon_intersection(&ray, &polygon));
        assert!(probe_ray_polygon_intersection(&ray, &polygon).0);
    }
    #[test]
    fn ray_polygon_another_polygon() {
        let polygon = Polygon::new(
            Vertex::new(-4.0, -1.0, -0.0),
            Vertex::new(-4.0, 1.0, -2.0),
            Vertex::new(-4.0, 1.0, -0.0),
        );
        let ray = Ray::new(Vector::new(0.0, 0.5, -1.), Vector::new(-50., 0.0, 0.0));
        println!("{:?}", polygon.get_normal());
        println!("{:?}", probe_ray_polygon_intersection(&ray, &polygon));
        assert!(probe_ray_polygon_intersection(&ray, &polygon).0);
    }
    #[test]
    fn ray_polygon_test_polygon() {
        let polygon = Polygon::new(
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, 1.0, -3.0),
            Vertex::new(-1.0, 1.0, -1.0),
        );
        let ray = Ray::new(Vector::new(0.0, 0.5, -2.), Vector::new(-50., 0.0, 0.0));
        println!("ray position: {:?}", ray.position);
        println!("ray direction: {:?}", ray.direction);
        println!("normal: {:?}", polygon.get_normal());
        println!(
            "intersection result: {:?}",
            probe_ray_polygon_intersection(&ray, &polygon)
        );
        assert!(probe_ray_polygon_intersection(&ray, &polygon).0);
    }
}
