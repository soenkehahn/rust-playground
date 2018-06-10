#![allow(dead_code)]

use std::mem;

fn main() {
    let mut list: List<i32> = List::new();
    for i in 0..10 {
        list.push(i);
    }
    println!("{:?}", list);
    for i in list.into_iter() {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Link<T> {
    link: MyOption<Box<Node<T>>>,
}

#[derive(Debug)]
pub enum MyOption<T> {
    MySome(T),
    MyNone,
}

impl<T> MyOption<T> {
    fn take(&mut self) -> MyOption<T> {
        mem::replace(self, MyOption::MyNone)
    }

    fn map<U, F>(self, f: F) -> MyOption<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyOption::MyNone => MyOption::MyNone,
            MyOption::MySome(x) => MyOption::MySome(f(x)),
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            MyOption::MyNone => None,
            MyOption::MySome(x) => Some(x),
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: Link {
                link: MyOption::MyNone,
            },
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: Link {
                link: self.head.link.take(),
            },
        });

        self.head = Link {
            link: MyOption::MySome(new_node),
        };
    }

    pub fn pop(&mut self) -> MyOption<T> {
        self.head.link.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.link.take();
        while let MyOption::MySome(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.link.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop().to_option()
    }
}
