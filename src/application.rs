use crate::primitives::vector;
use crate::scene::Scene;
use crate::transformations::{
    build_scale_transform, build_translation_transform, build_x_rotation_transform,
    build_y_rotation_transform, build_z_rotation_transform,
};

pub fn application(scene: &mut Scene) -> &Scene {
    let tick = scene.tick as f32;
    let meshes = &mut scene.meshes;

    scene.lights[0].direction = vector(-1., 0., 0.);
    meshes[0].add_transform(build_y_rotation_transform(tick.to_radians() * 3.));
    meshes[1].add_transform(build_y_rotation_transform(tick.to_radians() * -3.));

    meshes[2].add_transform(build_y_rotation_transform(tick.to_radians() * 1.));

    // mesh.add_transform(build_x_rotation_transform(-30_f32.to_radians()));
    // mesh.add_transform(build_translation_transform(vector(0., -2., 2.)));

    return scene;
}
