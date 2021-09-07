use std::rc::{Rc, Weak};

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
fn pure_clone_tuple() {
    let i = Rc::new(0);
    let x = Cell::new((i.clone(), Some(Rc::downgrade(&i))));
    x.set((Rc::new(42), None));
    assert_eq!(*x.get().0, 42);
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
            this.observable
                .get()
                .observer
                .set(Some(Rc::downgrade(&this)));
            this
        }

        fn call_observable(&self) {
            self.observable.get().call_observer();
        }

        fn do_something(&self) {
            self.observable.set(Observable::new());
        }
    }

    struct Observable {
        observer: Cell<Option<Weak<Observer>>>,
    }

    impl Observable {
        fn new() -> Rc<Self> {
            Rc::new(Self {
                observer: Cell::new(None),
            })
        }

        fn call_observer(&self) {
            self.observer
                .get()
                .unwrap()
                .upgrade()
                .unwrap()
                .do_something();
        }
    }

    let observable = Observable::new();
    let observer = Observer::new(observable);
    observer.call_observable();
    // TODO: Assertions
}
