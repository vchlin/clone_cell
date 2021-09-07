# clone_cell

clone_cell provides a `Cell` implementation that works with types whose `Clone` implementations are guaranteed not to mutate the `Cell` content through the `self` reference the `clone` method gets. This is done with a `PureClone` trait which is a subtrait of `Clone` (and a logical supertrait of `Copy`). It is only implemented for types with compliant `clone` methods.

## Overview

This crate was largely inspired by the Swift programming language's class properties (fields in Rust speak), which have value semantics. In Swift, class types themselves have reference semantics and are shared. But methods on class types are mutating. The Swift compiler is able to guarantee memory safety in a single-threaded context because copy constructors are not defined by the user. Intead, the compiler automatically generates ones that simply perform a field-wise clone.

In Rust, to enable interior-mutating methods on a `struct` stored in an `Rc` without the overhead of a `RefCell`, we can wrap each of the fields in a `Cell`. But `Cell::get` is only implemented for types that are `Copy`. This is because if the `clone` method obtains a reference to the `Cell`'s interior, it may be able to mutate its state, causing undefined behavior.

The `Cell` implementation provided by this crate is intended to be a drop-in replacement of `std::cell::Cell`. It can work with types that behave like "values", such as `Rc<T>`, `Weak<T>` (shared pointers themselves are like values; it is the pointees that behave like references), `Option<T: PureClone>`, and more. Some motivating use cases include combining `Cell` with clone-on-write or immutable collections to enable efficient mutable sharing of collections.

## Example

In this example below, we store an `Rc<T>` in a `Cell`.
```rust
let x = Cell::new(Rc::new(0));

x.set(Rc::new(42));

assert_eq!(*x.get(), 42);
```

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
- Types whose `clone` methods are known to be safe, such as those that only contain fields that are `PureClone`.

Please let me know if you find any soundness issues!

## Contributing

Pull requests are welcome and any feedback is appreciated!