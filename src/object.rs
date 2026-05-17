use std::{fmt::Debug, sync::Arc};

use crate::{
    geometry::{
        orientation::{Orientation, UP},
        primitives::{Mesh, Ray, Vector, ORIGIN},
    },
    lighting::Light,
    material::{Diffuse, BRDF},
    ray_tracing::ray_sphere_intersection::ray_sphere_intersection,
    scene::scene::EntityKey,
};

pub trait Entity: Debug + Sync + Send {
    fn get_position(&self) -> Vector;
    fn get_orientation(&self) -> Orientation;
    fn get_scale(&self) -> Vector;
    fn set_parent(&mut self, parent: EntityKey) -> ();
    fn get_children(&mut self) -> &mut Vec<EntityKey>;
    fn add_child(&mut self, child: EntityKey) {
        self.get_children().push(child);
    }
    fn as_light(&self) -> Option<&dyn Light> {
        None
    }
    fn as_object(&self) -> Option<&Object> {
        None
    }
    fn as_object_mut(&mut self) -> Option<&mut Object> {
        None
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
    pub children: Vec<EntityKey>,
    pub material: Arc<dyn BRDF>,
    pub meshes: Vec<Mesh>,
    pub parent: Option<EntityKey>,
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

    pub fn _add_child(mut self, child: EntityKey) {
        self.children.push(child);
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
            parent: None,
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
    fn get_children(&mut self) -> &mut Vec<EntityKey> {
        &mut self.children
    }
    fn set_parent(&mut self, parent: EntityKey) {
        self.parent = Some(parent);
    }
    fn as_object(&self) -> Option<&Object> {
        Some(self)
    }
    fn as_object_mut(&mut self) -> Option<&mut Object> {
        Some(self)
    }
}

#[derive(Debug, Clone)]
pub struct Empty {
    pub position: Vector,
    pub orientation: Orientation,
    pub scale: Vector,
    pub children: Vec<EntityKey>,
    pub parent: Option<EntityKey>,
}

impl Default for Empty {
    fn default() -> Empty {
        Empty {
            position: ORIGIN,
            orientation: UP,
            scale: Vector::ones(),
            children: Vec::new(),
            parent: None,
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
    fn get_children(&mut self) -> &mut Vec<EntityKey> {
        &mut self.children
    }
    fn set_parent(&mut self, parent: EntityKey) {
        self.parent = Some(parent);
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
