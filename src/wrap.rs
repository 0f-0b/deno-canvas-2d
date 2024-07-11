use std::ops::Deref;

#[derive(Debug)]
pub struct Wrap<T> {
    inner: T,
}

impl<T> Wrap<T> {
    pub fn new(value: T) -> Self {
        Self { inner: value }
    }
}

impl<T> Deref for Wrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
