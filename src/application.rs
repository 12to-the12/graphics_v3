use std::ops::IndexMut;

use crate::geometry::primitives::vector;
use crate::geometry::transformations::{
    build_arbitrary_rotation_transform, build_translation_transform, build_y_rotation_transform,
    build_z_rotation_transform,
};
use crate::object::Object;
use crate::scene::Scene;

/// all of the stuff that should happen to run the scene,
/// note: this is recomputed every frame. Not a major performance bottleneck
pub fn application(scene: &mut Scene) -> &Scene {
    let tick = scene.tick as f32;
    let objects: &mut Vec<Object> = &mut scene.objects;
    // scene.lights[0].position.x += 2.*tick;

    // scene.lights[0].direction = vector(-1., 0., 0.);

    // meshes[0].add_transform(build_x_rotation_transform(tick.to_radians() * 3.));
    objects
        .index_mut(0)
        .meshes
        .index_mut(0)
        .add_transform(build_arbitrary_rotation_transform(
            tick.to_radians(),
            vector(0., 0., -1.),
        ));

    objects
        .index_mut(0)
        .meshes
        .index_mut(0)
        .add_transform(build_arbitrary_rotation_transform(
            tick.to_radians(),
            vector(1., 1., 1.),
        ));

    // meshes[0].add_transform(build_y_rotation_transform(tick.to_radians() * 3.));
    objects
        .index_mut(1)
        .meshes
        .index_mut(0)
        .add_transform(build_y_rotation_transform(tick.to_radians() * -3.));

    // objects
    //     .index_mut(2)
    //     .position
    //     .translate(vector(0.1 * scene.tick as f32, 0., 0.));
    objects
        .index_mut(2)
        .meshes
        .index_mut(0)
        .add_transform(build_y_rotation_transform(tick.to_radians() * 2.));

    // mesh.add_transform(build_x_rotation_transform(-30_f32.to_radians()));
    // mesh.add_transform(build_translation_transform(vector(0., -2., 2.)));

    return scene;
}
