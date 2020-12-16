Preexisting stuff:

```rust
trait Mapping {
    type To<X>;
    type Arguments;
    fn create<X>(arguments: &Self::Arguments) -> Self::To<X>;
}
```

Expected use:

```rust
trait VoxelType {
    fn color() -> (u8, u8, u8, u8);
}

struct Air;

impl VoxelType for Air {
    fn color() -> (u8, u8, u8, u8) {
        (0, 0, 0, 0)
    }
}

struct Thing;

impl VoxelType for Thing {
    fn color() -> (u8, u8, u8, u8) {
        (0, 0, 0, 255)
    }
}

#[contained(VoxelDataRecord)]
struct ThingData(u8);

#[contained]
struct VoxelDataRecord<X: Mapping>;
```

Becomes:

```rust
trait VoxelDataRecordType {
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self>;
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self>;
}

impl VoxelDataRecordType for ThingData {
    fn get<X: Mapping>(record: &VoxelDataRecord<X>) -> &X::To<Self> {
        &record.thing_data
    }
    fn get_mut<X: Mapping>(record: &mut VoxelDataRecord<X>) -> &mut X::To<Self> {
        &mut record.thing_data
    }
}

struct VoxelDataRecord<X: Mapping> {
    thing_data: X::To<ThingData>,
}

impl<X: Mapping, A> VoxelDataRecord<X> {
    pub fn new(arguments: X::Arguments) -> Self {
        VoxelDataRecord {
            thing_data: X::create<ThingData>(&arguments),
        }
    }
    pub fn get<T: VoxelDataRecordType>(&self) -> &X::To<T> {
        T::get(&self)
    }
    pub fn get_mut<T: VoxelDataRecordType>(&mut self) -> &mut X::To<T> {
        T::get_mut(&mut self)
    }
}
```

Then:

```rust
struct HashMapping;
impl Mapping for HashMapping {
    type To<X> = HashMap<(usize, IVec), X>;
    type Arguments = ()
    fn create<X>(arguments: &Self::Arguments) -> Self::To<X> {
        HashMap::new()
    }
}
let record = VoxelDataRecord<HashMapping>::new(());
record.get<ThingData>()[(0, IVec::zero())] = ThingData(16);
```

Also, there needs to be a way to turn things like the `color` function into an array for efficient access. That would probably be solved by the `#[enum_dispatch]` things though.