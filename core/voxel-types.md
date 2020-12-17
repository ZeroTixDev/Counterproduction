Preexisting stuff:

```rust
#![feature(generic_associated_types)]
trait Mapping {
    type To<X>;
    type Arguments;
    fn create<X>(arguments: &Self::Arguments) -> Self::To<X>;
}
```

Expected use:

```rust
// thing.rs
#[contained(VoxelDataRecord)]
struct ThingData(u8);
// other_thing.rs
#[contained(VoxelDataRecord)]
struct OtherThingData(i64);
// voxel_record.rs
use thing::*;
use other_thing::*;
#[contained]
struct VoxelDataRecord;
```

Becomes:

```rust
// thing.rs
struct ThingData(u8);

impl VoxelDataRecordType for ThingData {
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self> {
        &record.thing_data
    }
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self> {
        &mut record.thing_data
    }
}
// other_thing.rs
struct OtherThingData(i64);

impl VoxelDataRecordType for OtherThingData {
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self> {
        &record.other_thing_data
    }
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self> {
        &mut record.other_thing_data
    }
}
// voxel_record.rs
struct VoxelDataRecord<X: Mapping> {
    thing_data: X::To<ThingData>,
    other_thing_data: X::To<OtherThingData>,
}

trait VoxelDataRecordType where Self: Sized {
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self>;
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self>;
}

impl<X: Mapping> VoxelDataRecord<X> {
    pub fn new(arguments: X::Arguments) -> Self {
        VoxelDataRecord {
            thing_data: X::create::<ThingData>(&arguments),
            other_thing_data: X::create::<OtherThingData>(&arguments),
        }
    }
    pub fn get<T: VoxelDataRecordType>(&self) -> &X::To<T> {
        T::get(self)
    }
    pub fn get_mut<T: VoxelDataRecordType>(&mut self) -> &mut X::To<T> {
        T::get_mut(self)
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
}
```

Also, there needs to be a way to turn things like the `color` function into an array for efficient access. That would probably be solved by the `#[enum_dispatch]` things though.