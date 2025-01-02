//! Shareable mutable containers.
//!
//! # When to use `Cell`
//!
//! [`Cell::get`] only requires `T` to be [`PureClone`](crate::clone::PureClone). This is useful
//! when working with with shared `struct`s whose fields are of types `Rc<T>`, `Weak<T>`, `Option<T:
//! PureClone>`, etc.
//!
//! Note that just because there can be any number of readers and writers to a [`Cell`] does not
//! mean it is always a good pattern. In most cases, it may not make sense to have more than one
//! writer at a time. But the user can easily build zero-cost abstractions on top of a `Cell` to
//! enforce this. For example, this may be useful when implementing the observer pattern.

use core::{
    cell::UnsafeCell,
    cmp::Ordering,
    fmt,
    fmt::{Debug, Formatter},
    mem, ptr,
};

use crate::clone::PureClone;

/// A mutable memory location with a [`get`](Cell::get) method that works with
/// [`PureClone`](crate::clone::PureClone) types.
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
        self.replace(value);
    }

    /// Swaps the values of two `Cell`s. Unlike `std::mem::swap`, this does not require a `&mut`
    /// reference.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `other` are different `Cell`s that partially overlap.
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

        // Check if the two overlap.
        let src_usize = self as *const Self as usize;
        let dst_usize = other as *const Self as usize;
        let diff = src_usize.abs_diff(dst_usize);
        if diff < size_of::<Self>() {
            // See https://github.com/rust-lang/rust/issues/80778 for more information.
            panic!("`Cell::swap` on overlapping non-identical `Cell`s");
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
        unsafe { (*self.value.get()).pure_clone() }
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

    /// Returns a mutable reference to the underlying data. This method requires `&mut self`,
    /// ensuring the caller has the only reference to it.
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

impl<T> Clone for Cell<T>
where
    T: PureClone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}

impl<T> Debug for Cell<T>
where
    T: Debug + PureClone,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Cell").field("value", &self.get()).finish()
    }
}

impl<T> Default for Cell<T>
where
    T: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for Cell<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> PartialEq for Cell<T>
where
    T: PartialEq + PureClone,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T> Eq for Cell<T> where T: Eq + PureClone {}

impl<T> PartialOrd for Cell<T>
where
    T: PartialOrd + PureClone,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.get() < other.get()
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.get() <= other.get()
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.get() > other.get()
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.get() >= other.get()
    }
}

impl<T> Ord for Cell<T>
where
    T: Ord + PureClone,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}
