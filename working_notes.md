Solving this bvh transform chain problem.

I want the hierarchy to be dynamically climbable.

Like cameras should be switchable and shit, that's all I mean.

I need an API to access rendered out transforms at every level.

It's a terrible idea to apply transforms to meshes piecemeal, but might work best for like orientation stacking.

I'd like a simple way to find the relative transforms between any two entities.



christ okay. Working around Rust here, trying to solve problems.
I'm trying to find a way to generify an object so that I can implement it however I like, something that would be perfect for traits; the problem is that I'm storing these things on the stack in typed fields, which means they need to be of a fixed size.
Boxes might be the solution here. Maybe enums.