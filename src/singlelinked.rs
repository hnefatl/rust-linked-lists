use super::list;
use std::mem;

pub struct SingleLinked<T> {
    inner: Node<T>,
    length: usize
}
enum Node<T> {
    Null,
    Cons(T, Box<Node<T>>)
}
use Node::*;

impl<T> SingleLinked<T> {
    pub fn new() -> SingleLinked<T> {
        SingleLinked { inner: Null, length: 0 }
    }
}
impl<T> Node<T> {
    // Walk along the list, calling a function on the value/tail each non-null point
    fn traverse_with<F>(&mut self, f: F) where F: Fn(&mut T, &mut Node<T>) {
        let mut next = self;
        while let Cons(v, t) = next {
            f(v, t);
            next = t;
        }
    }
}
impl<T> list::List<T> for SingleLinked<T> {
    fn push(&mut self, item: T) {
        // mem::replace lets us replace the list without moving it out of self.
        let old_tail = mem::replace(&mut self.inner, Cons(item, Box::new(Null)));
        if let Cons(_, current_tail) = &mut self.inner {
            **current_tail = old_tail;
            self.length += 1;
        } else {
            panic!("Failed to update head node")
        }
    }
    fn pop(&mut self) {
        match mem::replace(&mut self.inner, Null) {
            Null => (),
            Cons(_, t) => {
                self.inner = *t;
                self.length -= 1;
            }
        }
    }
    fn front(&mut self) -> Option<&mut T> {
        match &mut self.inner {
            Null => None,
            Cons(v, _) => Some(v)
        }
    }
    //fn push_back(&mut self, item: T) {
    //    let (_, &mut end) = self.get_tail_mut();
    //    end = Box::new(Cons(item, Box::new(Null)));
    //}
    //fn pop_back(&mut self) {
    //    let (_, &mut tail) = self.get_tail_mut();
    //    tail = Box::new(Null);
    //}
    //fn back(&self) -> &mut T {
    //    let &mut Cons get_tail_mut();
    //}

    fn len(&self) -> usize {
        self.length
    }

    fn to_vec(&mut self) -> Vec<&mut T> {
        let mut result = Vec::new();
        let mut current = &mut self.inner;
        while let Cons(v, t) = current {
            result.push(v);
            current = &mut *t;
        }
        result
    }
}
//impl<T> Iterator for SingleLinked<T> {
//    type Item = T;
//    fn next(&mut self) -> Option<Self::Item> {
//        match &mut self.inner {
//            Null => None,
//            Cons(v, t) => 
//        }
//    }
//}