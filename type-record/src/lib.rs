#![allow(incomplete_features)]
#![feature(generic_associated_types)]
/// A mapping from a type to another type, with a function for creation.
pub trait Mapping {
    type To<X>;
    type Arguments;
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
