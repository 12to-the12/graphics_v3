use crate::{
    entity::Entity,
    geometry::{
        orientation::{Orientation, UP},
        primitives::{Vector, ORIGIN},
    },
    scene::scene::EntityKey,
};

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
