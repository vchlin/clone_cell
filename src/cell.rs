use core::{cell::UnsafeCell, mem};

use crate::clone::PureClone;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        let old = self.replace(value);
        drop(old);
    }

    pub fn replace(&self, value: T) -> T {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        mem::replace(unsafe { &mut *self.value.get() }, value)
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    #[inline]
    pub fn get(&self) -> T
    where
        T: PureClone,
    {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        unsafe { (*self.value.get()).clone() }
    }
}
