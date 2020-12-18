<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment commands-show-output first-line-heading -->
Preexisting stuff:

```rust
#![feature(generic_associated_types)]
pub trait Mapping {
    type To<X>;
    type Arguments;
    fn create<X>(arguments: &Self::Arguments) -> Self::To<X>;
}
pub struct EmptyMapping;

impl Mapping for EmptyMapping {
    type To<X> = ();
    type Arguments = ();
    fn create<X>(_: &Self::Arguments) -> Self::To<X> { () }
}
pub trait RecordType<R> where Self: Sized {
    type Record<X: Mapping>;
    fn get<X: Mapping>(record: &Self::Record<X>) -> &X::To<Self>;
    fn get_mut<X: Mapping>(record: &mut Self::Record<X>) -> &mut X::To<Self>;
}
pub trait Record<X: Mapping, U, T: RecordType<U>> {
    fn _get(&self) -> &X::To<T>;
    fn _get_mut(&mut self) -> &mut X::To<T>;
}
```

Expected use:

```rust
// thing.rs
struct ThingData(u8);
// other_thing.rs
struct OtherThingData(i64);
// voxel_record.rs
use thing::*;
use other_thing::*;
type_record! {
    VoxelDataRecord {
        ThingData,
        OtherThingData,
    }
}
```

Becomes:

```rust
// thing.rs
struct ThingData(u8);
// other_thing.rs
struct OtherThingData(i64);
// voxel_record.rs
struct VoxelDataRecord<X: Mapping> {
    thing_data: X::To<ThingData>,
    other_thing_data: X::To<OtherThingData>,
}
impl RecordType<VoxelDataRecord<EmptyMapping>> for ThingData {
    type Record<X: Mapping> = VoxelDataRecord<X>;
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self> {
        &record.thing_data
    }
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self> {
        &mut record.thing_data
    }
}
impl RecordType<VoxelDataRecord<EmptyMapping>> for OtherThingData {
    type Record<X: Mapping> = VoxelDataRecord<X>;
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self> {
        &record.other_thing_data
    }
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self> {
        &mut record.other_thing_data
    }
}
impl<X: Mapping> VoxelDataRecord<X> {
    pub fn new(arguments: X::Arguments) -> Self {
        VoxelDataRecord {
            thing_data: X::create::<ThingData>(&arguments),
            other_thing_data: X::create::<OtherThingData>(&arguments),
        }
    }
    pub fn get<T: RecordType<VoxelDataRecord<EmptyMapping>>>(&self) -> &X::To<T> where Self: Record<X, VoxelDataRecord<EmptyMapping>, T> {
        <Self as Record<X, _, T>>::_get(self)
    }
    pub fn get_mut<T: RecordType<VoxelDataRecord<EmptyMapping>>>(&mut self) -> &mut X::To<T> where Self: Record<X, VoxelDataRecord<EmptyMapping>, T> {
        <Self as Record<X, _, T>>::_get_mut(self)
    }
}
```

Then:

```rust
use std::collections::HashMap;
struct HashMapping;
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct IVec(i32, i32, i32);
impl Mapping for HashMapping {
    type To<X> = HashMap<(usize, IVec), X>;
    type Arguments = ();
    fn create<X>(_: &Self::Arguments) -> Self::To<X> {
        HashMap::new()
    }
}

fn main() {
    let mut record = VoxelDataRecord::<HashMapping>::new(());
    record.get_mut().insert((0, IVec(0, 0, 0)), ThingData(16));
    println!("{:?}", record.get::<ThingData>()[&(0, IVec(0, 0, 0))].0);
    record.get_mut().insert((0, IVec(0, 0, 0)), OtherThingData(1024));
    println!("{:?}", record.get::<OtherThingData>()[&(0, IVec(0, 0, 0))].0);
}
```

Also, there needs to be a way to turn things like the `color` function into an array for efficient access. That would probably be solved by the `#[enum_dispatch]` things though.
