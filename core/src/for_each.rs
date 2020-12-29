use std::marker::PhantomData;

pub trait ForEach<T> {
    fn for_each(&self, f: impl FnMut(T));
    fn for_each_map<S, F: Fn(T) -> S>(&self, mapper: F) -> ForEachMap<T, Self, S, F> {
        ForEachMap(self, mapper, PhantomData)
    }
}
pub trait ForEachMut<T> {
    fn for_each_mut(&mut self, f: impl FnMut(T));
}
impl<T, S> ForEachMut<T> for S
where
    S: ForEach<T>,
{
    fn for_each_mut(&mut self, f: impl FnMut(T)) {
        self.for_each(f)
    }
}

pub struct ForEachMap<'a, T, X: ForEach<T> + ?Sized, S, F: Fn(T) -> S>(
    &'a X,
    F,
    PhantomData<(*const T, *const S)>,
);
impl<T, X: ForEach<T>, S, F: Fn(T) -> S> ForEach<S> for ForEachMap<'_, T, X, S, F> {
    fn for_each(&self, mut f: impl FnMut(S)) {
        self.0.for_each(|t| f(self.1(t)))
    }
}
