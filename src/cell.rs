use core::{cell::UnsafeCell, mem};

use crate::clone::PureClone;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    /// Creates a new `Cell` containing the given value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Sets the contained value.
    #[inline]
    pub fn set(&self, value: T) {
        let old = self.replace(value);
        drop(old);
    }

    /// Replaces the contained value with `value` and returns the old value.
    pub fn replace(&self, value: T) -> T {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        mem::replace(unsafe { &mut *self.value.get() }, value)
    }

    /// Unwraps the value.
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Returns a copy of the contained value.
    #[inline]
    pub fn get(&self) -> T
    where
        T: PureClone,
    {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        unsafe { (*self.value.get()).clone() }
    }
}
