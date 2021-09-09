# clone_cell

clone_cell provides a `Cell` implementation that works with types whose `Clone`
implementations are guaranteed not to mutate the `Cell` content through the `self`
reference the `clone` method gets. This is done with the provided `PureClone` trait,
which is a subtrait of `Clone` (and a logical supertrait of `Copy`). It is only
implemented for types with compliant `clone` methods.

## Overview

The `Cell` implementation provided by this crate is intended to be a drop-in
replacement of `std::cell::Cell`. It can work with types that behave like "values",
such as `Rc<T>`, `Weak<T>` (shared pointers themselves are like values; it is the
pointees that behave like references), `Option<T: PureClone>`, and more. Some
motivating use cases include combining `Cell` with clone-on-write or immutable
collections to enable efficient mutable sharing of data structures.

`PureClone` is currently implemented for the following types:
- All primitives such as `i32`, `usize`, `f64`, etc;
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

See the documentation for [`Cell`] for more.

[`Cell`]: https://docs.rs/clone_cell/latest/clone_cell/cell/struct.Cell.html

## Limitations

- Similar to `std::cell::Cell`, this `Cell` is `!Sync`.
- `PureClone` is currently only implemented for some types from the standard library. I hope to support user types as well with a proc macro so that we can automatically derive `PureClone` for types when they only contain fields that are `PureClone`:
   ```rust
   // Not supported yet!
   #[derive(PureClone, Clone)]
   struct Foo {
       x: i32,
   }

   let f = Cell::new(Foo { x: 0 });
   f.set(Foo { x: 42 });

   assert_eq!(f.get().x, 42);
   ```

## Safety

I believe this is sound, because `PureClone` is unsafe to implement. This trait is implemented for:
- `Copy` types;
- Types that perform a shallow clone such as `Rc` and `Weak`; and
- Types whose `clone` methods are otherwise known to be safe, such as compound types that only contain `PureClone` types.

Please let me know if you find any soundness issues!

## Contributing

Pull requests are welcome and any feedback is appreciated!