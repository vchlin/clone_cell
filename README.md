# clone_cell

clone_cell provides a `Cell` implementation that works with types whose `Clone`
implementations are guaranteed not to mutate the `Cell` content through the `&self`
reference. This is enforced with the provided `PureClone` trait, which is a subtrait
of `Clone` (and a logical supertrait of `Copy`). It is only implemented for types
with compliant `clone` methods.

## Overview

The `Cell` implementation provided by this crate is intended to be a drop-in
replacement of `std::cell::Cell`. It can work with types that behave like *values*,
such as `Rc<T>`, `Weak<T>` (shared pointers themselves are like *values*; it is the
pointees that behave like *references*), `Option<T: PureClone>`, and more. Some
motivating use cases include implementing the observer pattern and combining `Cell`
with clone-on-write or immutable collections to enable efficient sharing of data
structures.

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
let x = Cell::new(Rc::new(0));

x.set(Rc::new(42));

assert_eq!(*x.get(), 42);
```

A proc macro is also provided to derive `PureClone` for user types safely.
```
#[derive(PureClone)]
struct Foo {
    x: i32,
}

let f = Cell::new(Foo { x: 0 });
f.set(Foo { x: 42 });

assert_eq!(f.get().x, 42);
```

See the documentation for [`Cell`] for more.

[`Cell`]: https://docs.rs/clone_cell/latest/clone_cell/cell/struct.Cell.html

## Limitations

- Similar to `std::cell::Cell`, this `Cell` is `!Sync`.
- Since a new trait `PureClone` is used, there is no out-of-box support for types from third-party crates.

## Soundness

I believe this is sound, because `PureClone` is unsafe to implement. This trait is implemented for:
- `Copy` types;
- Types that perform a shallow clone such as `Rc` and `Weak`; and
- Types whose `clone` methods are otherwise known to be safe, such as compound types that only contain `PureClone` types.

See the [documentation] for more information. Please let me know if you find any soundness issues!

[documentation]: https://docs.rs/clone_cell/

## Contributing

Pull requests are welcome and any feedback is appreciated!