use clone_cell::clone::PureClone;
use clone_cell::cell::Cell;

#[derive(PureClone, Clone)]
struct Foo {
    x: Cell<i32>,
}

fn main() {}
