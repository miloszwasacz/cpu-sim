use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Immutable<T>(T);

impl<T> Immutable<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Borrow<T> for Immutable<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for Immutable<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
