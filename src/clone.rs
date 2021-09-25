//! The `PureClone` trait, which is a subtrait of [`Clone`].

/// A derive macro that generates impls of the traits [`PureClone`] and [`Clone`].
///
/// See the [crate#soundness] doc on why this macro also generates a `Clone` impl.
#[cfg(feature = "derive")]
pub use crate::derive::PureClone;

/// A restrictive form of `Clone` that does not mutate the containing [`Cell`].
///
/// Conceptually, the relationship between [`Copy`], [`Clone`], and `PureClone` can be
/// thought of as follows:
/// ```text
/// Copy: PureClone: Clone
/// ```
///
/// `PureClone` is `unsafe` because the `clone` implementation must not mutate the
/// content of `Cell` through the `&self` reference it gets with interior
/// mutability. See this [Stack Overflow answer] and this [Rust forum thread] for
/// details.
///
/// [`Cell`]: crate::cell::Cell
/// [Rust forum thread]:
/// https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/3
/// [Stack Overflow answer]:
/// https://stackoverflow.com/questions/39667868/why-can-cell-in-rust-only-be-used-for-copy-and-not-clone-types
pub unsafe trait PureClone: Clone {
    #[inline]
    fn pure_clone(&self) -> Self {
        Clone::clone(self)
    }
}

/// Implementations for types that are known to have compliant `clone` implementations.
mod impls {
    use std::rc::{Rc, Weak};

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
