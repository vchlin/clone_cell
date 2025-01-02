use std::mem::transmute;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use clone_cell::cell::Cell;

#[test]
fn copy_fields() {
    struct Foo {
        x: Cell<i32>,
        y: Cell<i32>,
    }

    let f = Rc::new(Foo {
        x: Cell::new(0),
        y: Cell::new(0),
    });
    f.x.set(42);
    f.y.set(f.x.get());
    assert_eq!(f.x.get(), 42);
    assert_eq!(f.y.get(), 42);
}

#[test]
fn pure_clone_fields() {
    struct Foo {
        x: Cell<Rc<i32>>,
        y: Cell<Option<Rc<i32>>>,
    }

    let f = Rc::new(Foo {
        x: Cell::new(Rc::new(0)),
        y: Cell::new(None),
    });
    let i = Rc::new(42);
    f.x.set(i.clone());
    f.y.set(Some(i));
    assert_eq!(*f.x.get(), 42);
    assert_eq!(*f.y.get().unwrap(), 42);
    f.x.set(Rc::new(0));
    assert_eq!(*f.x.get(), 0);
    assert_eq!(*f.y.get().unwrap(), 42);
}

#[test]
fn pure_clone_fields_arc() {
    struct Foo {
        x: Cell<Arc<i32>>,
        y: Cell<Option<Arc<i32>>>,
    }

    let f = Rc::new(Foo {
        x: Cell::new(Arc::new(0)),
        y: Cell::new(None),
    });
    let i = Arc::new(42);
    f.x.set(i.clone());
    f.y.set(Some(i));
    assert_eq!(*f.x.get(), 42);
    assert_eq!(*f.y.get().unwrap(), 42);
    f.x.set(Arc::new(0));
    assert_eq!(*f.x.get(), 0);
    assert_eq!(*f.y.get().unwrap(), 42);
}

#[test]
fn pure_clone_tuple() {
    let i = Rc::new(0);
    let x = Cell::new((i.clone(), Some(Rc::downgrade(&i))));
    x.set((Rc::new(42), None));
    assert_eq!(*x.get().0, 42);
}

#[test]
fn bad_drop() {
    struct Foo {
        ptr: Rc<Cell<Option<Foo>>>,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            // Triggers `drop` again...
            self.ptr.set(None);
        }
    }

    let c = Rc::new(Cell::new(None));
    c.set(Some(Foo { ptr: c.clone() }));
    c.set(None);
    assert_eq!(c.take().is_none(), true);
}

#[test]
fn cycle() {
    struct Observer {
        observable: Cell<Rc<Observable>>,
    }

    impl Observer {
        fn new(observable: Rc<Observable>) -> Rc<Self> {
            let this = Rc::new(Self {
                observable: Cell::new(observable),
            });
            this.observable.get().observer.set(Rc::downgrade(&this));
            this
        }

        fn poke_observable(&self) {
            self.observable.get().call_observer();
        }

        fn do_something(&self) {
            self.observable.set(Observable::new());
        }
    }

    struct Observable {
        observer: Cell<Weak<Observer>>,
    }

    impl Observable {
        fn new() -> Rc<Self> {
            Rc::new(Self {
                observer: Cell::new(Weak::new()),
            })
        }

        fn call_observer(&self) {
            self.observer.get().upgrade().unwrap().do_something();
        }
    }

    let observable = Observable::new();
    let weak_observable = Rc::downgrade(&observable);
    let observer = Observer::new(observable);
    observer.poke_observable();
    assert_eq!(weak_observable.upgrade().is_none(), true);
}

fn as_cell_of_array<T, const N: usize>(c: &[Cell<T>; N]) -> &Cell<[T; N]> {
    unsafe { transmute(c) }
}

#[test]
#[should_panic]
fn swap_overlap() {
    // Example from https://github.com/rust-lang/rust/issues/80778.
    let x = [Cell::new(vec![1]), Cell::new(vec![2]), Cell::new(vec![3])];
    let x1: &Cell<[_; 2]> = as_cell_of_array(x[0..2].try_into().unwrap());
    let x2: &Cell<[_; 2]> = as_cell_of_array(x[1..3].try_into().unwrap());
    // This should panic.
    x1.swap(x2);
}

#[test]
fn swap_nonoverlap() {
    let x = [
        Cell::new(vec![1]),
        Cell::new(vec![2]),
        Cell::new(vec![3]),
        Cell::new(vec![4]),
    ];
    let x1: &Cell<[_; 2]> = as_cell_of_array(x[0..2].try_into().unwrap());
    let x2: &Cell<[_; 2]> = as_cell_of_array(x[2..4].try_into().unwrap());
    x1.swap(x2);
}
