use crate::scene::Scene;
use crate::transformations::{build_x_rotation_transform,build_y_rotation_transform,build_z_rotation_transform};

pub fn application(scene: &mut Scene, counter: f32) -> &Scene {
    let mesh = &mut scene.meshes[0];
    mesh.add_transform(build_y_rotation_transform(-counter.to_radians()));
    mesh.add_transform(build_z_rotation_transform(-0.5 * counter.to_radians()));

    return scene;
}
