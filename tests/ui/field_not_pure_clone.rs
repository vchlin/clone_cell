use clone_cell::{cell::Cell, clone::PureClone};

#[derive(PureClone)]
struct Foo {
    x: Cell<i32>,
}

fn main() {}
