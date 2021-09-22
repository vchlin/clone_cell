use clone_cell::clone::PureClone;

#[derive(Clone)]
struct Foo;

#[derive(PureClone)]
struct Bar {
    f: Foo,
}

fn main() {}
