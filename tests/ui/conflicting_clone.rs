use clone_cell::clone::PureClone;

#[derive(PureClone)]
struct Foo;

impl Clone for Foo {
    // TODO: Could marking this as `default` make `PureClone` unsound?
    fn clone(&self) -> Self {
        Self {}
    }
}

fn main() {}
