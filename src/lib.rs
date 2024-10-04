use std::rc::Rc;

#[derive(Debug)]
pub enum List<T>{
    Cons(T, Rc<List<T>>),
    Nil,
}
