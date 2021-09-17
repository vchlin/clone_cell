use clone_cell::clone::PureClone;

#[derive(PureClone)]
struct Foo {
    x: i32,
    y: f32,
}

impl Clone for Foo {
    // TODO: Could marking this as `default` make `PureClone` unsound?
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {}
