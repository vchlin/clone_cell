//! This crate provides a [`Cell`] implementation that works with types whose `Clone`
//! implementations are guaranteed not to mutate the `Cell` content through the
//! `self` reference the `clone` method gets. This is done with the provided
//! [`PureClone`] trait, which is a subtrait of [`Clone`] (and a logical supertrait
//! of [`Copy`]). It is only implemented for types with compliant `clone` methods.
//!
//! # Background
//!
//! This crate was largely inspired by the Swift programming language's
//! class properties (fields in Rust speak), which have value semantics. In Swift,
//! class types themselves have reference semantics and are shared. But methods on
//! class types are mutating. The Swift compiler is able to guarantee memory safety
//! in a single-threaded context because copy constructors are not defined by the
//! user. Intead, the compiler automatically generates ones that simply perform a
//! field-wise clone.
//!
//! In Rust, to enable interior-mutating methods on a `struct` stored in an `Rc`
//! without the overhead of a `RefCell`, we can wrap each of the fields in a
//! `Cell`. But `Cell::get` is only implemented for types that are `Copy`. This is
//! because if the `clone` method obtains a reference to the `Cell`'s interior, it
//! may be able to mutate its state. This can cause undefined behavior, as demonstrated
//! in this [Rust forum thread].
//!
//! By restricting outselves to a checked subset of `Clone` implementations that do
//! not exploit interior mutability to mutate the `Cell` content, it becomes possible
//! to provide a `Cell` with a `get` method that does not require `Copy` types.
//!
//! Note that just because there can be any number of readers and writers to a
//! [`Cell`] does not mean it is a good idea. In most cases, it may not make sense to
//! have more than one writer at a time. But the user can easily build abstractions
//! on top of a `Cell` to enforce this. I think this may be useful when implementing
//! the observer pattern.
//!
//! See the documentation for [`PureClone`] for a list of implemented types and more
//! details.
//!
//! [`Cell`]: cell/struct.Cell.html
//! [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [`PureClone`]: clone/trait.PureClone.html
//! [Rust forum thread]:
//! https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/3

// TODO: #![no_std]

pub mod cell;
pub mod clone;
//#[cfg(feature = "derive")]
//use clone_cell_derive as derive;
