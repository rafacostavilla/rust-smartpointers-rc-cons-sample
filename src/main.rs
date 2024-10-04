use std::rc::Rc;

use rust_smartpointers_rc_cons_sample::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
