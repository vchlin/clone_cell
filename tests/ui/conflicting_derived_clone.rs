use clone_cell::clone::PureClone;

#[derive(PureClone, Clone)]
struct Foo {
    x: i32,
    y: f32,
}

fn main() {}
