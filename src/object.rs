use std::{fmt::Debug, sync::Arc, sync::Weak};

use crate::{
    geometry::{
        orientation::{Orientation, UP},
        primitives::{Mesh, Ray, Vector, ORIGIN},
    },
    material::{Diffuse, BRDF},
    ray_tracing::ray_sphere_intersection::ray_sphere_intersection,
    scene::scene::Scene,
};

pub trait Entity: Debug + Sync + Send {
    fn get_position(&self) -> Vector;
    fn get_orientation(&self) -> Orientation;
    fn get_scale(&self) -> Vector;
    fn get_children(&mut self) -> &mut Vec<Arc<dyn Entity>>;
    fn add_child(&mut self, child: Arc<dyn Entity>) {
        self.get_children().push(child);
    }
    // fn get_transforms(&self) -> &Vec<Transform>;
    // fn append_transforms(&self) -> &Vec<Transform>; // add position and scale and shit to log
}
/// physical object in space with associated data
#[derive(Debug, Clone)]
pub struct Object {
    pub position: Vector, // this is relative to it's parent
    // pub camera_space_position: Vector, // this exists in camera space
    pub orientation: Orientation,
    pub scale: Vector,
    pub children: Vec<Arc<dyn Entity>>,
    pub material: Arc<dyn BRDF>,
    pub meshes: Vec<Mesh>,
    // & links to textures associated with it
}
impl Object {
    pub fn get_radius(&self) -> f32 {
        let mut furthest = 0.;
        for mesh in &self.meshes {
            for vertex in &mesh.vertices {
                let dist = self.scale.max() * vertex.position.magnitude();
                if dist > furthest {
                    furthest = dist;
                }
            }
        }
        furthest
    }
    pub fn ray_intercept(&self, ray: &Ray) -> bool {
        let position = self.position;
        let radius = self.get_radius();
        ray_sphere_intersection(ray, &position, &radius)
    }

    pub fn _add_child(mut self, child: Arc<dyn Entity>) {
        self.children.push(child.clone());
    }
}
impl Default for Object {
    fn default() -> Object {
        Object {
            position: ORIGIN,
            orientation: UP,
            scale: Vector::ones(),
            children: Vec::new(),
            material: Arc::new(Diffuse::default()),
            meshes: Vec::new(),
        }
    }
}

/// this will be modified in the future to accomodate a hierarchial parent child node system
impl Entity for Object {
    fn get_position(&self) -> Vector {
        self.position
    }
    fn get_orientation(&self) -> Orientation {
        self.orientation
    }
    fn get_scale(&self) -> Vector {
        self.scale
    }
    fn get_children(&mut self) -> &mut Vec<Arc<dyn Entity>> {
        &mut self.children
    }
}

#[derive(Debug, Clone)]
pub struct Empty {
    pub position: Vector,
    pub orientation: Orientation,
    pub scale: Vector,
    pub children: Vec<Arc<dyn Entity>>,
}

impl Default for Empty {
    fn default() -> Empty {
        Empty {
            position: ORIGIN,
            orientation: UP,
            scale: Vector::ones(),
            children: Vec::new(),
        }
    }
}
impl Entity for Empty {
    fn get_position(&self) -> Vector {
        self.position
    }
    fn get_orientation(&self) -> Orientation {
        self.orientation
    }
    fn get_scale(&self) -> Vector {
        self.scale
    }
    fn get_children(&mut self) -> &mut Vec<Arc<dyn Entity>> {
        &mut self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::primitives::Mesh;

    use super::Object;

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn test_radius() {
        let mymesh = Mesh::_unit_cube();
        let myobject: Object = Object {
            meshes: vec![mymesh],
            ..Object::default()
        };
        assert_eq!(myobject.get_radius(), f32::sqrt(3.));
    }
}
