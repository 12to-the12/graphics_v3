probably best to add the world, camera, screen, and display transformations to the long instead of performing the operations directly.

clipping and frustrum culling happen on a unified set of data, but we don't need that for a wireframe render