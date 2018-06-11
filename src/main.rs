#![allow(dead_code)]

fn main() {
    let mut list: List<i32> = List::new();
    for i in 0..10 {
        list = list.prepend(i);
    }
    println!("{:?}", list);
    for i in list {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
struct List<T> {
    list: InnerList<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            list: InnerList::Empty,
        }
    }

    fn prepend(self, a: T) -> List<T> {
        List {
            list: InnerList::Cons(Inner {
                head: a,
                tail: Box::new(self.list),
            }),
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let old = std::mem::replace(&mut self.list, InnerList::Empty);
        match old {
            InnerList::Empty => None,
            InnerList::Cons(inner) => {
                self.list = *inner.tail;
                Some(inner.head)
            }
        }
    }
}

#[derive(Debug)]
enum InnerList<T> {
    Empty,
    Cons(Inner<T>),
}

#[derive(Debug)]
struct Inner<T> {
    head: T,
    tail: Box<InnerList<T>>,
}
