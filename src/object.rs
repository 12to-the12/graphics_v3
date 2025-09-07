use crate::{
    geometry::{
        orientation::{Orientation, _UP},
        primitives::{Mesh, Ray, Vector, ORIGIN},
    },
    material::{PBR,ShaderNode},
    ray_tracing::ray_sphere_intersection::ray_sphere_intersection, scene::ShaderMode,
};

/// physical object in space with associated data
#[derive(Clone, Debug)]
pub struct Object {
    pub position: Vector,
    pub _orientation: Orientation,
    pub _scale: f32,
    pub _children: Vec<Object>,
    pub material: ShaderNode,
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
}

/// the defaults
pub const OBJECT: Object = Object {
    position: ORIGIN,
    _orientation: _UP,
    _scale: 1.,
    _children: Vec::new(),
    material: ShaderNode::PBR(PBR::new()),
    meshes: Vec::new(),
};

// struct MeshPool {
//     meshes: Vec
// }

#[cfg(test)]
mod tests {
    use crate::geometry::primitives::_unit_cube;

    use super::{Object, OBJECT};

    /// useful table: https://www.nikonians.org/reviews/fov-tables
    #[test]
    fn test_radius() {
        let mymesh = _unit_cube();
        let myobject: Object = Object {
            meshes: vec![mymesh],
            ..OBJECT
        };
        assert_eq!(myobject.get_radius(), f32::sqrt(3.));
    }
}
