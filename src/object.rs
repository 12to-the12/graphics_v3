use std::{fmt::Debug, sync::Arc};

use crate::{
    geometry::{
        orientation::{Orientation, _UP},
        primitives::{Mesh, Ray, Vector, ORIGIN},
    },
    material::{BRDF, PBR},
    ray_tracing::ray_sphere_intersection::ray_sphere_intersection,
};

pub trait Entity: Debug + Sync + Send {
    fn get_position(&self) -> Vector {
        ORIGIN
    }

    fn _get_parent(&self) -> Option<Arc<dyn Entity>> {
        None
    }
    // fn get_transforms(&self) -> &Vec<Transform>;
    // fn append_transforms(&self) -> &Vec<Transform>; // add position and scale and shit to log
}
/// physical object in space with associated data
#[derive(Debug, Clone)]
pub struct Object {
    pub _parent: Option<Arc<dyn Entity>>,
    pub position: Vector,
    pub _orientation: Orientation,
    pub _scale: f32,
    pub _children: Vec<Arc<dyn Entity>>,
    pub material: Arc<dyn BRDF>,
    pub meshes: Vec<Mesh>,
    // & links to textures associated with it
}
impl Object {
    pub fn get_radius(&self) -> f32 {
        let mut furthest = 0.;
        for mesh in &self.meshes {
            for vertex in &mesh.vertices {
                let dist = self._scale * vertex.position.magnitude();
                if dist > furthest {
                    furthest = dist;
                }
            }
        }
        return furthest;
    }
    pub fn ray_intercept(&self, ray: &Ray) -> bool {
        let position = self.position;
        let radius = self.get_radius();
        return ray_sphere_intersection(ray, &position, &radius);
    }

    pub fn _add_child(mut self, child: Arc<dyn Entity>) -> () {
        self._children.push(child.clone());
    }
}

impl Default for Object {
    fn default() -> Object {
        Object {
            _parent: None,
            position: ORIGIN,
            _orientation: _UP,
            _scale: 1.,
            _children: Vec::new(),
            material: Arc::new(PBR::default()),
            meshes: Vec::new(),
        }
    }
}

/// this will be modified in the future to accomodate a hierarchial parent child node system
impl Entity for Object {
    fn get_position(&self) -> Vector
    where
        Self: Sized,
    {
        self.position
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
