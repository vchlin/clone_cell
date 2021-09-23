#![cfg(feature = "derive")]

use std::rc::Rc;

use clone_cell::{cell::Cell, clone::PureClone};

#[test]
fn pure_clone_struct() {
    #[derive(PureClone)]
    struct Foo {
        x: i32,
        y: f32,
    }

    let c = Cell::new(Foo { x: 0, y: 0.0 });
    c.set(Foo { x: 42, y: -42.0 });
    assert_eq!(c.get().x, 42);
    assert_eq!(c.get().y, -42.0);
}

#[test]
fn inherent_clone_method() {
    #[derive(PureClone)]
    struct Foo {
        data: i32,
        ptr: Rc<Cell<Option<Bar>>>,
    }

    #[derive(PureClone)]
    struct Bar {
        f: Foo,
    }

    // Attempt to provide an "inherent" `clone` method. But the derived `clone`
    // method should never call this in its code path, so this can't cause any
    // damage.
    impl Foo {
        // Bad clone implementation. Example from:
        // https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/9
        #[allow(dead_code)]
        fn clone(&self) -> Self {
            // Clear out the cell we're contained in...
            self.ptr.set(None);
            Self {
                data: self.data,
                ptr: self.ptr.clone(),
            }
        }
    }

    let c: Rc<Cell<Option<Bar>>> = Rc::new(Cell::new(None));
    c.set(Some(Bar {
        f: Foo {
            data: 42,
            ptr: c.clone(),
        },
    }));
    assert_eq!(c.get().unwrap().f.data, 42);
}

#[test]
fn with_type_params() {
    #[derive(PureClone)]
    struct Foo<T> {
        value: T,
    }

    #[derive(PureClone)]
    struct Bar<T> {
        f: Foo<T>,
    }

    let f = Foo { value: 42 };
    let b = Bar { f };
    assert_eq!(b.clone().f.value, 42);
}

#[test]
fn with_lifetimes() {
    // TODO: Add another lifetime?
    #[derive(PureClone)]
    struct Foo<'a, T> {
        a: &'a T,
    }

    let i = 42;
    let f = Foo { a: &i };
    assert_eq!(f.clone().a, &i);
    assert_eq!(*f.clone().a, i);
}

#[test]
fn variant() {
    #[derive(Debug, PartialEq, PureClone)]
    struct Foo;

    #[derive(Debug, PartialEq, PureClone)]
    enum Bar<T> {
        _X,
        _Y(i32),
        Z { x: (usize,), y: Box<T>, z: Foo },
    }

    let b = Bar::Z { x: (42,), y: Box::new('y'), z: Foo };
    let b2 = b.clone();
    assert_eq!(b, b2);
}
