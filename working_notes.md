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