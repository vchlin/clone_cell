use std::rc::Rc;

use clone_cell::{cell::Cell, clone::PureClone};

#[derive(PureClone)]
struct Foo {
    data: i32,
    ptr: Rc<Cell<Option<Foo>>>,
}

impl Clone for Foo {
    // Bad clone implementation. Example from:
    // https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/9
    fn clone(&self) -> Self {
        let data = &self.data;
        // Clear out the cell we're contained in...
        self.ptr.set(None);
        Self { data: self.data, ptr: self.ptr.clone() }
    }
}

fn main() {
    let c = Rc::new(Cell::new(None));
    c.set(Some(Foo {
        data: 42,
        ptr: c.clone()
    }));
    assert_eq!(c.get().unwrap().data, 42);
}
