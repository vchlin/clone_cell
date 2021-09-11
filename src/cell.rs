//! Shareable mutable containers.
//!
//! # When to use `Cell`
//!
//! [`Cell::get`] only requires `T` to be [`PureClone`]. This is useful when working
//! with with shared `struct`s whose fields are of types `Rc<T>`, `Weak<T>`,
//! `Option<T: PureClone>`, etc.
//!
//! Note that just because there can be any number of readers and writers to a
//! [`Cell`] does not mean it is always a good pattern. In most cases, it may not
//! make sense to have more than one writer at a time. But the user can easily build
//! zero-cost abstractions on top of a `Cell` to enforce this. For example, this may be
//! useful when implementing the observer pattern.
//!
//! [`PureClone`]: crate::clone::PureClone

use core::{cell::UnsafeCell, mem};

use crate::clone::PureClone;

/// A mutable memory location with a [`get`] method that works with [`PureClone`]
/// types.
///
/// [`PureClone`]: crate::clone::PureClone
/// [`get`]: Cell::get
///
/// # Examples
///
/// `Cell` works with some non-`Copy` types such as `Rc`:
/// ```
/// use std::rc::Rc;
/// use clone_cell::cell::Cell;
///
/// let x = Cell::new(Rc::new(0));
/// x.set(Rc::new(42));
/// assert_eq!(*x.get(), 42);
/// ```
#[repr(transparent)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    /// Creates a new `Cell` containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(42);
    /// ```
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Sets the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(42);
    /// c.set(0);
    /// ```
    #[inline]
    pub fn set(&self, value: T) {
        // It's important to move the old value out first and not drop it in
        // place. This prevents potential infinite recursion that can otherwise occur
        // if `T` contains a reference back to this `Cell`, and `T::drop` calls us
        // again.
        let old = self.replace(value);
        drop(old);
    }

    /// Replaces the contained value with `value` and returns the old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(42);
    /// assert_eq!(c.get(), 42);
    /// assert_eq!(c.replace(2), 42);
    /// assert_eq!(c.get(), 2);
    /// ```
    pub fn replace(&self, value: T) -> T {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        mem::replace(unsafe { &mut *self.value.get() }, value)
    }

    /// Unwraps the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(42);
    /// assert_eq!(c.into_inner(), 42);
    /// ```
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Returns a copy of the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use clone_cell::cell::Cell;
    ///
    /// let p = Rc::new(42);
    /// let c = Cell::new(p.clone());
    /// let p2 = c.get();
    /// assert_eq!(*p, *p2);
    /// assert_eq!(p, p2);
    /// assert_eq!(Rc::strong_count(&p), 3);
    /// assert_eq!(Rc::strong_count(&p2), 3);
    /// ```
    #[inline]
    pub fn get(&self) -> T
    where
        T: PureClone,
    {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        unsafe { (*self.value.get()).clone() }
    }
}
