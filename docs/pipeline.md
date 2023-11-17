probably best to add the world, camera, screen, and display transformations to the long instead of performing the operations directly.

object ->
world ->
camera ->
screen ->
display

every step includes a translation, rotation, and scaling segment, in that order


still not sure why there is a distinction between screen and display



clipping and frustrum culling happen on a unified set of data, but we don't need that for a wireframe render