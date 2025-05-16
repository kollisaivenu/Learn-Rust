use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let e = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let f = Cons(3, Rc::clone(&e));
    let g = Cons(4, Rc::clone(&e));
    let x = RefCell::new(5);
    let mut y = x.borrow_mut();
    *y = 7;
    println!("y = {}", *y);
    //let mut z = x.borrow_mut();

}
