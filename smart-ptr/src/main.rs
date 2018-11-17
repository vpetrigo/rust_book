use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    adjacent: RefCell<Vec<Weak<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            adjacent: RefCell::new(Vec::new()),
        }
    }
}

fn print_adjacent_nodes(node: &Node) {
    print!("Adjacent to node {}:", node.value);

    node.adjacent.borrow().iter().for_each(|n| {
        if let Some(exists) = n.upgrade() {
            print!(" node {}", exists.value);
        }
    });

    println!("");
}

fn main() {
    let node1 = Rc::new(Node::new(1));
    let node2 = Rc::new(Node::new(2));
    let node3 = Rc::new(Node::new(3));

    node1.adjacent.borrow_mut().push(Rc::downgrade(&node2));
    node1.adjacent.borrow_mut().push(Rc::downgrade(&node3));
    node2.adjacent.borrow_mut().push(Rc::downgrade(&node1));
    node3.adjacent.borrow_mut().push(Rc::downgrade(&node1));

    print_adjacent_nodes(&node1);
    print_adjacent_nodes(&node2);
    print_adjacent_nodes(&node3);
}