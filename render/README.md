# Counterproduction-Render

This folder will contain a crate for rendering multiple voxel grids.

## Implementation

The current hypothesized rendering approach uses this collection of algorithms and data structures:

### Stored Data

**CPU and GPU data:**

This data is stored on both the CPU and the GPU, but is only mutated on the CPU. Mutating the data must involve only sending the data mutated to the GPU.

* A list of all positions (`f32x3`) and rotation (`f32x4`) of entities. (Relative to the camera?)
* A list of every renderable<sup>[1](#fn1)</sup> voxel's position relative to the entity (`i32x3`), entity index (`i32` or `i24`), and type (`u8`).
* A list of colors of voxels (`f32x3` or `u8x3`), indexed by type (`u8`).

**CPU data:**

* A mapping between the index of a renderable voxel (see `../storage.md`), and the position of the voxel within the buffer.

### Algorithms

When a voxel switches from being renderable to not being renderable, the voxel will be deleted from the buffer, by replacing it with the last voxel within the buffer (and updating the mapping due to the change). Similarly for when a voxel switches the other way.

During rendering, the GPU will compute the actual position, rotation, and color of each voxel based on the position within the entity, the entity's position and rotation, and the type of the voxel. Rendering will use the [fast ray-box intersection algorithm](http://www.jcgt.org/published/0007/03/04/paper-lowres.pdf), which takes in a set of voxels, each with a position, rotation, and color, and renders them onto the screen using a partial rasterization / ray-tracing approach.

### Things yet to be Completed

* How to do transparent voxels? (Don't write to depth buffer?)

<a name="fn1">1</a>: A voxel that is not surrounded on all 6 sides by solid voxels.
