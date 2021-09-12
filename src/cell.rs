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

use core::{cell::UnsafeCell, mem, ptr};

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
pub struct Cell<T>
where
    T: ?Sized,
{
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

    /// Swaps the values of two `Cell`s. Unlike `std::mem::swap`, this does not
    /// require a `&mut` reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use clone_cell::cell::Cell;
    ///
    /// let c1 = Cell::new(Rc::new(21));
    /// let c2 = Cell::new(Rc::new(42));
    /// c1.swap(&c2);
    /// assert_eq!(42, *c1.get());
    /// assert_eq!(21, *c2.get());
    /// ```
    #[inline]
    pub fn swap(&self, other: &Self) {
        if ptr::eq(self, other) {
            return;
        }
        // SAFETY: Only safe because `Cell` is `!Sync`. Also, no pointers are
        // invalidated since `Cell` never returns references to its content.
        unsafe {
            ptr::swap(self.value.get(), other.value.get());
        }
    }

    /// Replaces the contained value with `value` and returns the old value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(Rc::new(42));
    /// assert_eq!(*c.get(), 42);
    /// assert_eq!(*c.replace(Rc::new(2)), 42);
    /// assert_eq!(*c.get(), 2);
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
    // TODO: Mark `const` once stabilized.
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
    /// let c = Cell::new(Rc::downgrade(&p));
    /// let p2 = c.get().upgrade().unwrap();
    /// assert_eq!(*p, *p2);
    /// assert_eq!(p, p2);
    /// assert_eq!(Rc::strong_count(&p), 2);
    /// assert_eq!(Rc::strong_count(&p2), 2);
    /// ```
    #[inline]
    pub fn get(&self) -> T
    where
        T: PureClone,
    {
        // SAFETY: Only safe because `Cell` is `!Sync`.
        unsafe { (*self.value.get()).clone() }
    }

    // TODO:
    // pub fn update

    /// Takes the value of the `Cell`, leaving a `Default::default()` in its place.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(42);
    /// let i = c.take();
    /// assert_eq!(i, 42);
    /// assert_eq!(c.into_inner(), 0);
    /// ```
    pub fn take(&self) -> T
    where
        T: Default,
    {
        self.replace(Default::default())
    }
}

impl<T> Cell<T>
where
    T: ?Sized,
{
    /// Returns a raw pointer to the underlying data in this `Cell`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let c = Cell::new(42);
    /// let p = c.as_ptr();
    /// ```
    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    /// Returns a mutable reference to the underlying data. This method requires
    /// `&mut self`, ensuring the caller has the only reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use clone_cell::cell::Cell;
    ///
    /// let mut c = Cell::new(42);
    /// *c.get_mut() += 1;
    /// assert_eq!(c.get(), 43);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    /// Returns a `&Cell<T>` from a `&mut T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use clone_cell::cell::Cell;
    ///
    /// let p = &mut Rc::new(42);
    /// let c = Cell::from_mut(p);
    /// assert_eq!(*c.get(), 42);
    /// ```
    #[inline]
    pub fn from_mut(t: &mut T) -> &Self {
        // SAFETY: `&mut` is unique.
        unsafe { &*(t as *mut T as *const Self) }
    }
}

impl<T> Cell<[T]> {
    /// Returns a `&[Cell<T>]` from a `&Cell<[T]>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use clone_cell::cell::Cell;
    ///
    /// let s: &mut [Rc<i32>] = &mut [Rc::new(0), Rc::new(1), Rc::new(2)];
    /// let cs: &Cell<[Rc<i32>]> = Cell::from_mut(s);
    /// let sc: &[Cell<Rc<i32>>] = cs.as_slice_of_cells();
    /// assert_eq!(sc.len(), 3);
    /// assert_eq!(*sc[0].get(), 0);
    /// assert_eq!(*sc[1].get(), 1);
    /// assert_eq!(*sc[2].get(), 2);
    /// ```
    pub fn as_slice_of_cells(&self) -> &[Cell<T>] {
        // SAFETY: `Cell<T>` has the same memory layout as `T`.
        unsafe { &*(self as *const Self as *const [Cell<T>]) }
    }
}

// TODO: Implement CoerceUnsized
