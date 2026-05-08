Solving this bvh transform chain problem.

I want the hierarchy to be dynamically climbable.

Like cameras should be switchable and shit, that's all I mean.

I need an API to access rendered out transforms at every level.

It's a terrible idea to apply transforms to meshes piecemeal, but might work best for like orientation stacking.

I'd like a simple way to find the relative transforms between any two entities.



christ okay. Working around Rust here, trying to solve problems.
I'm trying to find a way to generify an object so that I can implement it however I like, something that would be perfect for traits; the problem is that I'm storing these things on the stack in typed fields, which means they need to be of a fixed size.
Boxes might be the solution here. Maybe enums.

The material class needs to encapsulate any sort of material response, from perfect reflectance, to emission, to an image texture as albedo to a procedural texture as roughness




another task:
how do we track watts that land on our image sensor?
with an isotrophic light source directly shining into it, we can estimate the solid angle of the photosite with respect to the source to integrate away the steridians, but what about a bounce?


The polygons are in camera space!
That's why they're double offset!
And also why the functions are broken if not used from the same angle



lights need to be rotated!


lights are currently isotrophic
material calculations are done in watts/steridian with distance taken into account
if a camera ray hits a light, 

it needs to be triangulated!



an exhaustive exploration of the discrepancy between the rasterization and ray tracing projections shows that the raster projection is inaccurate


hard problems to solve:
pbrt
rotations


objects need their positions changed for bounding volume hierarchies to work
trying to sort out how transforms play with the scene graph


transform system:
the root has the camera transform applied to it to get stuff into camera space
everything needs to hold it's position in camera space as well as original values
camera space values are never propagated down!
view space stuff is not applied to the scene
the object/light buffer are rebuilt each time transforms are traversed


integration basically multiplies by whatever is the subject of integration
remember that the samples are used to approximate integration via monte carlo, the idea is that we treat *all* directions and points on surfaces as receiving whatever our rays receive