//! This crate provides a [`Cell`](cell::Cell) implementation that works with types whose `clone`
//! methods are guaranteed not to mutate the `Cell` content using the `&self` reference. This is
//! enforced with the provided [`PureClone`] trait, which is a subtrait of [`Clone`] (and a logical
//! supertrait of [`Copy`]). It is only implemented for types with a compliant `clone` method.
//!
//! See the [`cell`](module@cell) module documentation for more information on how to use it.
//!
//! # Background
//!
//! To enable interiorly mutating methods on a type stored in an [`Rc`](alloc::rc::Rc) or
//! [`Arc`](alloc::sync::Arc) without the overhead of a [`RefCell`](core::cell::RefCell),
//! we can wrap each of its fields in a [`core::cell::Cell`].
//! But `Cell`'s [`get`](core::cell::Cell::get) method is only implemented for
//! types that are `Copy`. This is because if the `clone` method obtains a reference to the
//! containing `Cell`, it may be able to mutate its state. This can cause undefined behavior, as
//! demonstrated in this [example].
//!
//! By restricting ourselves to a checked subset of `Clone` implementations that do not exploit
//! interior mutability to mutate the `Cell` content, it becomes possible to provide a `Cell` with a
//! `get` method that does not require the type to be `Copy`.
//!
//! See the documentation for [`PureClone`] for a list of implemented types and the [`clone`] module
//! documentation for more details.
//!
//! # Soundness
//!
//! I believe this is sound, because `PureClone` is unsafe to implement. In user code, the only ways
//! to use `PureClone` without `unsafe` are:
//! 1. Use types that already implement `PureClone`.
//! 1. Use the provided [`PureClone`](derive@clone::PureClone) proc macro to derive this trait for
//! user types, which ensures:
//!     - Each field/variant of a given user `struct`/`enum` is also `PureClone`.
//!     - A `clone` method that does not call any `Cell` content accessors is implemented (such as
//!     one generated by `#[derive(Clone)]`).
//!
//! ## Interaction with specialization
//!
//! The [`PureClone`](derive@clone::PureClone) proc macro generates:
//! 1. A non-`default` `Clone` impl with trait bounds that ensure any fields with generic parameters
//! are also `Clone`; and
//! 1. A `PureClone` impl that only compiles if all fields are also `PureClone`.
//!
//! Item 1 is non-`default` and hence cannot be further specialized.
//!
//! The user may attempt to provide a `default` `Clone` impl (or one with a `default` `clone`
//! method). But this is fine, because item 2 ensures every field is `PureClone` (and therefore
//! `Clone`), so item 1 is always selected even if a user-provided `default` `Clone` impl with fewer
//! trait requirements is present.
//!
//! So I think even with [RFC1210] fully implemented, this is still sound.
//!
//! [`PureClone`]: clone::PureClone
//! [example]:
//! https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/3
//! [RFC1210]: https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md

#![no_std]

extern crate alloc;

pub mod cell;
pub mod clone;
#[cfg(feature = "derive")]
use clone_cell_derive as derive;
