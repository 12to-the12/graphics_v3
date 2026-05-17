use crate::{
    camera::Camera,
    geometry::{orientation::Orientation, primitives::Vector},
    lighting::Light,
    object::Object,
    scene::scene::EntityKey,
};

pub trait Entity: Sync + Send {
    fn get_position(&self) -> Vector;
    fn get_orientation(&self) -> Orientation;
    fn get_scale(&self) -> Vector;
    fn set_parent(&mut self, parent: EntityKey) -> ();
    fn get_children(&self) -> Vec<EntityKey>;
    fn get_mut_children(&mut self) -> &mut Vec<EntityKey>;
    fn add_child(&mut self, child: EntityKey) {
        self.get_mut_children().push(child);
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
    fn as_camera(&self) -> Option<&Camera> {
        None
    }
    fn as_camera_mut(&mut self) -> Option<&mut Camera> {
        None
    }
    // fn get_transforms(&self) -> &Vec<Transform>;
    // fn append_transforms(&self) -> &Vec<Transform>; // add position and scale and shit to log
}
