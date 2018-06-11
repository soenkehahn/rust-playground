#![allow(dead_code)]

fn main() {
    let mut a: List<i32> = List::new();
    for i in 0..10 {
        a = a.prepend(i);
    }
    println!("{:?}", a);
    for i in a.into_iter() {
        println!("{:?}", i);
    }

    let mut b: List<i32> = List::new();
    for i in 0..10 {
        b = b.prepend(i);
    }
    println!("{:?}", b);
    for i in b.iter() {
        println!("{:?}", i);
        println!("{:?}", b.head());
    }
}

#[derive(Debug)]
struct List<T> {
    list: InnerList<T>,
}

#[derive(Debug)]
enum InnerList<T> {
    Empty,
    Cons(T, Box<InnerList<T>>),
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            list: InnerList::Empty,
        }
    }

    fn prepend(self, a: T) -> List<T> {
        List {
            list: InnerList::Cons(a, Box::new(self.list)),
        }
    }

    fn head(&self) -> Option<&T> {
        match self.list {
            InnerList::Empty => None,
            InnerList::Cons(ref a, _) => Some(a),
        }
    }
}

// into_iter

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { inner: self }
    }
}

struct IntoIter<T> {
    inner: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let list = &mut self.inner;
        let old = std::mem::replace(&mut list.list, InnerList::Empty);
        match old {
            InnerList::Empty => None,
            InnerList::Cons(head, tail) => {
                list.list = *tail;
                Some(head)
            }
        }
    }
}

// * iter

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {
        Iter { inner: &self.list }
    }
}

struct Iter<'a, T: 'a> {
    inner: &'a InnerList<T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.inner {
            InnerList::Empty => None,
            InnerList::Cons(head, tail) => {
                self.inner = tail;
                Some(&head)
            }
        }
    }
}
