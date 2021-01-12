# Counterproduction-Physics

This folder will contain all the physics code for collisions between voxel grids.

## Implementation

The physics will use [`raiper3d`](https://docs.rs/rapier3d/0.4.2/rapier3d/), once it supports a custom collision implementation, with all collisions being offloaded to the GPU. Continous collision detection will be done with fast moving objects.

Octrees will be used to do efficient collisions, with spheres approximating the higher-level boundries (but not the individual voxels). The GPU will only store a octree that describes whether the voxel at that position is collidable, not what type it is.
