use clone_cell::clone::PureClone;

#[derive(PureClone)]
struct Foo;

impl Clone for Foo {
    fn clone(&self) -> Self {
        Self {}
    }
}

fn main() {}
