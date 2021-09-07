use clone_cell::{cell::Cell, clone::PureClone};

#[test]
fn pure_clone_struct() {
    #[derive(PureClone, Clone)]
    struct Foo {
        x: i32,
        y: f32,
    }

    let c = Cell::new(Foo { x: 0, y: 0.0 });
    c.set(Foo { x: 42, y: -42.0 });
    assert_eq!(c.get().x, 42);
    assert_eq!(c.get().y, -42.0);
}
