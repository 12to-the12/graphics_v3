use crate::geometry::primitives::{Ray, Vector};

pub fn ray_sphere_intersection(ray: &Ray, position: &Vector, radius: &f32) -> bool {
    let mut ray = ray.clone();
    ray.direction.norm();
    let a = 1.;
    let C = position;
    let R = radius;
    let O = ray.position;
    let D = ray.direction;
    let b = (2. * D).dot(&(O - *C));
    let c = &(O - *C).magnitude().powi(2) - R.powi(2);

    let L = position.clone() - ray.position;
    // println!("{:?}", L);
    let tca = L.dot(&ray.direction);
    if tca < 0. {
        return false;
    }
    let d2 = L.dot(&L) - tca * tca;
    if d2 > radius * radius {
        return false;
    }
    // let thc = f32::sqrt(radius*radius-d2);
    // let t0 = tca-thc;
    // let t1 = tca+thc;
    // the discriminant thing
    let Δ = b * b - 4. * a * c;
    if Δ >= 0. {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {

    use crate::geometry::{
        orientation::J,
        primitives::{vector, Ray, ORIGIN},
    };

    use super::ray_sphere_intersection;

    /// useful table: https://www.nikonians.org/reviews/fov-tables

    #[test]
    fn test_simple_intersection() {
        let ray = Ray {
            position: vector(0., 0., 0.),
            direction: vector(0., 0., 1.),
        };
        let position = vector(0., 0., 5.);
        let radius = 1.;
        let result = ray_sphere_intersection(&ray, &position, &radius);
        assert!(result);

        let ray = Ray {
            position: vector(0., 0., 0.),
            direction: vector(0., 1., 0.),
        };
        let position = vector(0., 0., 5.);
        let radius = 1.;
        let result = ray_sphere_intersection(&ray, &position, &radius);
        assert!(!result);

        let position = vector(0., 5., 0.);
        let radius = 4.;

        let ray = Ray {
            position: ORIGIN,
            direction: J,
        };
        assert!(ray_sphere_intersection(&ray, &position, &radius));

        let ray = Ray {
            position: ORIGIN,
            direction: vector(4., 2., 0.),
        };
        assert!(!ray_sphere_intersection(&ray, &position, &radius));

        let ray = Ray {
            position: ORIGIN,
            direction: vector(1., 1., 0.),
        };
        assert!(ray_sphere_intersection(&ray, &position, &radius));
    }
    #[test]
    fn test_intersection() {
        let ray = Ray {
            position: vector(0., 0., 200.),
            direction: vector(0., 0., 1.),
        };
        let position = vector(0., 0., 205.);
        let radius = 1.;
        assert!(ray_sphere_intersection(&ray, &position, &radius));

        let ray = Ray {
            position: vector(2., 0., 200.),
            direction: vector(0., 0., 1.),
        };
        let position = vector(0., 0., 205.);
        let radius = 1.;
        assert!(!ray_sphere_intersection(&ray, &position, &radius));
    }
}
