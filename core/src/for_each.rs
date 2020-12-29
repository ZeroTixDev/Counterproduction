pub trait ForEach<T> {
    fn for_each(&self, f: impl FnMut(T));
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
