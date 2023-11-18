use crate::scene::Scene;
use crate::transformations::{
    build_scale_transform, build_x_rotation_transform, build_y_rotation_transform,
    build_z_rotation_transform,
};
use crate::primitives::vector;

pub fn application(scene: &mut Scene, counter: f32) -> &Scene {
    let mesh = &mut scene.meshes[0];
    mesh.add_transform(build_scale_transform(vector(2.0, 1.0, 1.0)));
    mesh.add_transform(build_y_rotation_transform(-counter.to_radians()));
    // mesh.add_transform(build_z_rotation_transform(-0.5 * counter.to_radians()));

    return scene;
}
