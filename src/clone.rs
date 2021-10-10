//! Provides the [`PureClone`] trait, which is a restrictive form of `Clone` that does not mutate
//! the containing [`Cell`](crate::cell::Cell).
//!
//! Conceptually, the relationship between [`Copy`], [`Clone`], and `PureClone` can be thought of as
//! follows:
//! ```text
//! Copy: PureClone: Clone
//! ```
//!
//! `PureClone` is `unsafe` because the `clone` implementation must not mutate the content of `Cell`
//! through the `&self` reference it gets with interior mutability. See this [Stack Overflow answer]
//! and this [Rust forum thread] for details.
//!
//! When this [`crate`] is built with the `"derive"` feature, the [`PureClone`](derive@PureClone)
//! proc macro can be used to derive `PureClone` for user types.
//!
//! [Rust forum thread]:
//! https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/3
//! [Stack Overflow answer]:
//! https://stackoverflow.com/questions/39667868/why-can-cell-in-rust-only-be-used-for-copy-and-not-clone-types

/// A derive macro that generates impls of the traits [`PureClone`] and [`Clone`].
///
/// See the [crate#soundness] doc on why this macro also generates a `Clone` impl.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
/// use clone_cell::{cell::Cell, clone::PureClone};
///
/// // Note: This also generates a `Clone` impl.
/// #[derive(PureClone)]
/// struct Foo<T> {
///     p: Rc<T>, // `Rc<T>` is always `PureClone`.
///     t: Option<T>, // `Option<T>` is `PureClone` if `T` is.
///     x: i32, // `i32` is `PureClone`.
/// }
///
/// let p = Rc::new(-42);
/// let f = Cell::new(Foo {
///     p: p.clone(),
///     t: Some(0),
///     x: 0,
/// });
///
/// f.set(Foo {
///     p,
///     t: Some(42),
///     x: 21,
/// });
///
/// assert_eq!(*f.get().p, -42);
/// assert_eq!(f.get().t, Some(42));
/// assert_eq!(f.get().x, 21);
/// ```
#[cfg(feature = "derive")]
pub use crate::derive::PureClone;

/// The `PureClone` trait, which is a subtrait of [`Clone`].
///
/// See the [module](self) documentation for more information.
pub unsafe trait PureClone: Clone {
    /// The `pure_clone` method.
    #[inline]
    fn pure_clone(&self) -> Self {
        Clone::clone(self)
    }
}

/// Implementations for types that are known to have compliant `clone` implementations.
mod impls {
    use alloc::{
        boxed::Box,
        rc::{Rc, Weak},
        vec::Vec,
    };

    use super::PureClone;

    macro_rules! impl_pure_clone {
        ($($t:ty)*) => {
            $(
                unsafe impl PureClone for $t {}
            )*
        }
    }

    macro_rules! impl_pure_clone_rc {
        ($($i:ident<T>)*) => {
            $(
                unsafe impl<T> PureClone for $i<T> where T: ?Sized {}
            )*
        }
    }

    macro_rules! impl_pure_clone_generic {
        ($($i:ident<$($j:ident),*>)*) => {
            $(
                unsafe impl<$($j),*> PureClone for $i<$($j),*> where $($j: PureClone),* {}
            )*
        }
    }

    macro_rules! impl_pure_clone_tuples {
        ($(($($i:ident),*))*) => {
            $(
                unsafe impl<$($i),*> PureClone for ($($i,)*) where $($i: PureClone),* {}
            )*
        }
    }

    unsafe impl<T> PureClone for &T where T: ?Sized {}

    impl_pure_clone! {
        usize u8 u16 u32 u64 u128
        isize i8 i16 i32 i64 i128
        f32 f64
        bool char
    }

    impl_pure_clone_rc! {
        Rc<T> Weak<T>
    }

    impl_pure_clone_generic! {
        Box<T>
        Option<T>
        Result<T, E>
        Vec<T>
    }

    impl_pure_clone_tuples! {
        ()
        (A)
        (A, B)
        (A, B, C)
        (A, B, C, D)
        (A, B, C, D, E)
        (A, B, C, D, E, F)
        (A, B, C, D, E, F, G)
        (A, B, C, D, E, F, G, H)
        (A, B, C, D, E, F, G, H, I)
        (A, B, C, D, E, F, G, H, I, J)
        (A, B, C, D, E, F, G, H, I, J, K)
        (A, B, C, D, E, F, G, H, I, J, K, L)
    }
}
