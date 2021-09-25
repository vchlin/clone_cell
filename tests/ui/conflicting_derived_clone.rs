use clone_cell::clone::PureClone;

// The order of `Clone` and `PureClone` shouldn't ever matter. But test both just in case.
#[derive(PureClone, Clone)]
struct Foo;

#[derive(Clone, PureClone)]
struct Bar;

fn main() {}
