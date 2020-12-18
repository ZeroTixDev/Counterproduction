#![allow(incomplete_features)]
#![feature(generic_associated_types)]
//! **WARNING:** This crate requires a nightly compiler and
//! `generic_associated_types`.
//!
//! This crate provides a macro to create a type level record, in which values
//! can be fetched by their types. The types of the things within the record are
//! specified by an inputted `Mapping`.
//!
//! Example usage:
//!
//! ```rust
//! #![allow(incomplete_features)]
//! #![feature(generic_associated_types)]
//! use type_record::{record, Mapping};
//! struct Thing(u8);
//! struct OtherThing(i64);
//! // This creates a type level record, by the name of "Record",
//! // with component types `Thing` and `OtherThing`.
//! record! {
//!     Record {
//!         Thing,
//!         OtherThing,
//!     }
//! }
//!
//! use std::collections::HashMap;
//! struct HashMapping;
//! impl Mapping for HashMapping {
//!     type To<X> = HashMap<usize, X>;
//!     // This type is the arguments inputted into the create function.
//!     type Arguments = ();
//!     // This create function is called for each type within the record.
//!     fn create<X>(_: &Self::Arguments) -> Self::To<X> {
//!         HashMap::new()
//!     }
//! }
//!
//! #[test]
//! fn test() {
//!     // Creates a record, using the `HashMapping` mapping, and arguments `()`.
//!     let mut record = Record::<HashMapping>::new(());
//!     // Gets a mutable reference to the `HashMap<usize, Thing>` within the record.
//!     record.get_mut().insert(0, Thing(16));
//!     // Gets a `HashMap<usize, Thing>` and finds the inserted `Thing`.
//!     assert_eq!(16, record.get::<Thing>()[&0].0);
//!     record
//!         .get_mut()
//!         .insert(18, OtherThing(1024));
//!     assert_eq!(1024, record.get::<OtherThing>()[&18].0);
//! }
//! ```

/// A mapping from a type to another type, with a function for creation.
pub trait Mapping {
    /// This type function controls what the record types are.
    ///
    /// If this is `type To<X> = Vec<X>`, then the record will consist of
    /// vectors of the component types.
    type To<X>;
    /// The arguments provided to the `create` function.
    type Arguments;
    /// This function creates something of `To<X>` given the arguments.
    /// In a record, this would be called for each component type.
    fn create<X>(arguments: &Self::Arguments) -> Self::To<X>;
}

/// This type exists for macro expansion. Do not use.
pub struct _EmptyMapping;

impl Mapping for _EmptyMapping {
    type To<X> = ();
    type Arguments = ();
    fn create<X>(_: &Self::Arguments) -> Self::To<X> {}
}
/// This type exists for macro expansion. Do not use.
pub trait _RecordType<R>
where
    Self: Sized, {
    type Record<X: Mapping>;
    fn get<X: Mapping>(record: &Self::Record<X>) -> &X::To<Self>;
    fn get_mut<X: Mapping>(record: &mut Self::Record<X>) -> &mut X::To<Self>;
}
/// This type exists for macro expansion. Do not use.
pub trait _Record<X: Mapping, U, T: _RecordType<U>> {
    fn _get(&self) -> &X::To<T>;
    fn _get_mut(&mut self) -> &mut X::To<T>;
}
impl<X: Mapping, U, T: _RecordType<U>> _Record<X, U, T> for T::Record<X> {
    #[inline]
    fn _get(&self) -> &X::To<T> {
        T::get::<X>(self)
    }
    #[inline]
    fn _get_mut(&mut self) -> &mut X::To<T> {
        T::get_mut::<X>(self)
    }
}

#[macro_export]
/// The macro to create a type level record.
/// Always generates a `struct` with name `$record_name`, with a single generic
/// argument, which is the Mapping that controls what the record contains.
macro_rules! record {
    ($record_name:ident { $($record_contents:ident),+ $(,)? }) => {
        #[allow(non_snake_case)]
        pub struct $record_name<X: ::type_record::Mapping> {
            $($record_contents: X::To<$record_contents>,)+
        }
        $(impl ::type_record::_RecordType<$record_name<::type_record::_EmptyMapping>> for $record_contents {
            type Record<X: ::type_record::Mapping> = $record_name<X>;
            #[inline]
            fn get<X: ::type_record::Mapping>(record: &$record_name<X>) -> &X::To<Self> {
                &record.$record_contents
            }
            #[inline]
            fn get_mut<X: ::type_record::Mapping>(record: &mut $record_name<X>) -> &mut X::To<Self> {
                &mut record.$record_contents
            }
        })+
        impl<X: ::type_record::Mapping> $record_name<X> {
            #[inline]
            #[doc = "Creates a new record using the arguments provided."]
            pub fn new(arguments: X::Arguments) -> Self {
                $record_name {
                    $($record_contents: X::create::<$record_contents>(&arguments),)+
                }
            }
            #[inline]
            #[doc = "Gets an immutable reference to the value with key `T`."]
            pub fn get<
                T: ::type_record::_RecordType<$record_name<::type_record::_EmptyMapping>>
            >(&self) -> &X::To<T> where Self: ::type_record::_Record<X, $record_name<::type_record::_EmptyMapping>, T> {
                <Self as ::type_record::_Record<X, $record_name<::type_record::_EmptyMapping>, T>>::_get(self)
            }
            #[inline]
            #[doc = "Gets a mutable reference to the value with key `T`."]
            pub fn get_mut<
                T: ::type_record::_RecordType<$record_name<::type_record::_EmptyMapping>>
            >(&mut self) -> &mut X::To<T> where Self: ::type_record::_Record<X, $record_name<::type_record::_EmptyMapping>, T> {
                <Self as ::type_record::_Record<X, $record_name<::type_record::_EmptyMapping>, T>>::_get_mut(self)
            }
        }
    };
}
