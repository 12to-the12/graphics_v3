use std::vec;

use crate::primitives::vector;
use crate::scene::Scene;
use crate::transformations::{
    build_scale_transform, build_translation_transform, build_x_rotation_transform,
    build_y_rotation_transform, build_z_rotation_transform,
};

pub fn application(scene: &mut Scene, counter: f32) -> &Scene {
    let mesh = &mut scene.meshes[0];

    mesh.add_transform(build_y_rotation_transform(counter.to_radians() * 3.));

    mesh.add_transform(build_x_rotation_transform(-30_f32.to_radians()));
    mesh.add_transform(build_translation_transform(vector(0., -2., 2.)));

    return scene;
}
