use clone_cell::clone::PureClone;

struct Foo;

#[derive(PureClone)]
struct Bar {
    f: Foo,
}

fn main() {}
