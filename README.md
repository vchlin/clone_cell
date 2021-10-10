# clone_cell

clone_cell provides a `Cell` implementation that works with types whose `clone` methods are
guaranteed not to mutate the `Cell` content through the `&self` reference. This is enforced with the
provided `PureClone` trait, which is a subtrait of `Clone` (and a logical supertrait of `Copy`). It
is only implemented for types with a compliant `clone` method.

## Overview

The `Cell` implementation provided by this crate is intended to be a drop-in replacement of
`std::cell::Cell`. It can work with types that behave like *values*, such as `Rc<T>`, `Weak<T>`
(shared pointers themselves are like *values*; it is the pointees that behave like *references*),
`Option<T: PureClone>`, and more. Some motivating use cases include implementing the observer
pattern and combining `Cell` with clone-on-write or immutable collections to enable efficient
sharing of data structures.

`PureClone` is currently implemented for the following types:
- All primitives such as `i32`, `usize`, `f64`, etc;
- References: `&T`;
- `Rc<T>` and `Weak<T>`;
- `Option<T: PureClone>`; and
- Tuples: `(A: PureClone, ...)`.

See [`PureClone`] for a complete list.

[`PureClone`]: https://docs.rs/clone_cell/latest/clone_cell/clone/trait.PureClone.html

## Examples

In this example below, we store an `Rc<T>` in a `Cell` and later retrieve a copy of it.
```rust
use std::rc::Rc;
use clone_cell::cell::Cell;

let x = Cell::new(Rc::new(0));

x.set(Rc::new(42));

assert_eq!(*x.get(), 42);
```

See the documentation for [`Cell`] for more.

A proc macro is also provided to derive `PureClone` for user types safely.
```rust
use std::rc::Rc;
use clone_cell::{cell::Cell, clone::PureClone};

// Note: This also generates a `Clone` impl.
#[derive(PureClone)]
struct Foo<T> {
    p: Rc<T>, // `Rc<T>` is always `PureClone`.
    t: Option<T>, // `Option<T>` is `PureClone` if `T` is.
    x: i32, // `i32` is `PureClone`.
}

let p = Rc::new(-42);
let f = Cell::new(Foo {
    p: p.clone(),
    t: Some(0),
    x: 0,
});

f.set(Foo {
    p,
    t: Some(42),
    x: 21,
});

assert_eq!(*f.get().p, -42);
assert_eq!(f.get().t, Some(42));
assert_eq!(f.get().x, 21);
```

See the [`clone`] module documentation for more information.

[`Cell`]: https://docs.rs/clone_cell/latest/clone_cell/cell/struct.Cell.html
[`clone`]: https://docs.rs/clone_cell/latest/clone_cell/clone/index.html

## Limitations

- Similar to `std::cell::Cell`, this `Cell` is `!Sync`.
- Since a new trait `PureClone` is used, there is no out-of-the-box support for types from third-party crates.

## Safety

This is safe to use, because `PureClone` is an `unsafe` trait, and all `PureClone` implementations
are checked. This trait is implemented for:
- `Copy` types;
- Types that perform a shallow clone such as `Rc` and `Weak`; and
- Types whose `clone` methods are otherwise known to be safe, such as compound types that only
  contain `PureClone` types.

See the [documentation] for more information. Please let me know if you find any soundness issues!

[documentation]: https://docs.rs/clone_cell/

## Contributing

Pull requests are welcome and any feedback is appreciated!