use std::{cell::RefCell, rc::Rc};

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    // tail: Option<Rc<RefCell<Node>>>,
    // length: u128,
}
impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: val,
            next: None,
        }))
    }
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, val: i32) {
        self.head = Some(Node::new(val));
    }

    // fn display(&self) {
    //     while let Some(_) == self.head{
    //         println!("ks");
    //     }
    // }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push(34);
    ll.push(56);
}
