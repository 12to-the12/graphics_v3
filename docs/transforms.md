only one transform should exist for any given object
an object owns it's transform
transforms can be evaluated over a list of vertices,
ammended to include additional transforms



it should be created by revising it step by step to arrive at the final version, which is then applied to derive the correct coordinates of the final vertices on the screen

there is no such thing as a rotation matrix or translation matrix, these are forms that contribute to actual transform matrices

a new transform starts as an identity matrix,
and is modified from there

transforms:
- translation
- rotation
- scale