Solving this bvh transform chain problem.

I want the hierarchy to be dynamically climbable.

Like cameras should be switchable and shit, that's all I mean.

I need an API to access rendered out transforms at every level.

It's a terrible idea to apply transforms to meshes piecemeal, but might work best for like orientation stacking.

I'd like a simple way to find the relative transforms between any two entities.