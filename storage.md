<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment commands-show-output first-line-heading -->

# An Efficient Voxel Storage for Arbitrary Sized Voxel Types

## Assumptions

For the purpose of this article, we assume that each voxel has a type, which is fixed-sized and no more than `2` bytes. Each voxel also has a separate piece of data, that depends on the type of the voxel. For example, an "air" voxel may have a type of `0`, while a "damaged" voxel may have a type of `1`, and a piece of data that is a float between zero and one that describes how damaged it is. This can be described as an enum in Rust:

```rust
enum Voxel {
    Air,
    Damaged(f32),
}
```

We also assume that there is a chunk-based Voxel Storage, generic around the type of voxel. A chunk is a solid array of voxels, with a metadata component. This storage must be sparse: the amount of space that the storage requires to contain `n` voxels, arbitrarily far away from each other, must be `O(n)`. It must also have `O(1)` indexing of voxels. Whenever a voxel is accessed, it must be `O(1)` to get the metadata associated with the chunk that the voxel is within, as well as `O(1)` to get the position of the voxel within the chunk. These requirements can be easily achieved by a hashmap of chunks.

Another assumption is that there will be multiple voxel storages. The hypothetical use case will be more like Teardown rather than Minecraft.

## Primary Issues with a Simple Voxel Storage

If a voxel is stored as an enum, the size of each individual voxel is equal to the size of the largest voxel. As such, the size of the voxel type described above (Example 1) is `5` bytes, assuming `4` bytes for the `f32` and `1` byte for the discriminant. With a larger list of voxels and more complex voxels, the total amount of space can be orders of magnitude greater than the average used size of a voxel.

## Solution

To solve this problem, I will separate the voxel type from the data that is dependent on the type. The most simple way to do this is to replace the part of the voxel stored within the storage with a discriminant paired with a unique identifier for that voxel. Separately, there can be a series of hashmaps, one for each discriminant, indexed by the unique identifier type, that store the separate information that is dependent on the type of the voxel. Unfortunately, this approach requires at least `5` bytes for each voxel (`1` byte for the discriminant and `4` bytes for the unique identifier), and usually at least `10` bytes. (`2` bytes for the discriminant and `8` bytes for the unique identifier - there'll probably be more than 4 billion voxels). This is too large to be used as the information stored within a voxel storage.

A better solution is to pair each chunk with a single unique identifier, and combine this with the position of the voxel within the chunk.

## Issues

The greatest issue is that the time of this is not guaranteed to be `O(1)`, as adding an element to a hash map can cause a large array copy. As such, this will have to use a custom hash map, that does gradual copying. This hash map will contain up to two separate hash maps with fixed sizes. One hash map will be designated as the "primary" hash map. If there is no secondary hash map, when an element is inserted, it is inserted into the primary hash map. If the primary hash map is above a certain threshold, another hash map, with a size of double the primary hash map, will be created, and set as the priamry hash map. The last primary hash map will be set as the secondary hash map. Whenever any element is inserted, the primary hash map will have that element inserted, and a number of elements will be transferred from the secondary hash map to the primary hash map. When an element is attempted to be fetched, while there are both a primary and a secondary hash map, the hash map with the greatest number of elements will be tested first, and if the element does not exist in that hash map, the other hash map will be tested.

While this is not truly `O(1)`, it will not have any large hangs caused by data copying.
