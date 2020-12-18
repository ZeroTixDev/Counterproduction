#![allow(incomplete_features)]
#![feature(generic_associated_types)]
use type_record::*;

struct ThingData(u8);
struct OtherThingData(i64);
record! {
    VoxelDataRecord {
        ThingData,
        OtherThingData,
    }
}

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

#[test]
fn test() {
    let mut record = VoxelDataRecord::<HashMapping>::new(());
    record.get_mut().insert((0, IVec(0, 0, 0)), ThingData(16));
    assert_eq!(16, record.get::<ThingData>()[&(0, IVec(0, 0, 0))].0);
    record
        .get_mut()
        .insert((0, IVec(0, 0, 0)), OtherThingData(1024));
    assert_eq!(1024, record.get::<OtherThingData>()[&(0, IVec(0, 0, 0))].0);
}
