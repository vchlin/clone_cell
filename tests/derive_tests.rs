// use clone_cell::{cell::Cell, clone::PureClone};
// use std::rc::Rc;

// #[test]
// fn pure_clone_struct() {
//     #[derive(PureClone, Clone)]
//     struct Foo {
//         x: i32,
//         y: f32,
//     }

//     let c = Cell::new(Foo { x: 0, y: 0.0 });
//     c.set(Foo { x: 42, y: -42.0 });
//     assert_eq!(c.get().x, 42);
//     assert_eq!(c.get().y, -42.0);
// }

// #[test]
// fn pure_clone_bad() {
//     #[derive(PureClone)]
//     struct Foo {
//         data: i32,
//         ptr: Rc<Cell<Option<Foo>>>,
//     }

//     impl Clone for Foo {
//         // Bad clone implementation. Example from:
//         // https://users.rust-lang.org/t/why-does-cell-require-copy-instead-of-clone/5769/9
//         fn clone(&self) -> Self {
//             let data = &self.data;
//             // Clear out the cell we're contained in...
//             self.ptr.set(None);
//             Self { data: self.data, ptr: self.ptr.clone() }
//         }
//     }

//     let c = Rc::new(Cell::new(None));
//     c.set(Some(Foo {
//         data: 42,
//         ptr: c.clone()
//     }));
//     assert_eq!(c.get().unwrap().data, 42);
// }
