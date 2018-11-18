use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};
use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct Node {
    value: i32,
    adjacent: RefCell<Vec<Weak<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            adjacent: RefCell::new(Vec::new()),
        }
    }

    pub fn adjacent_ref(&self) -> Ref<Vec<Weak<Node>>> {
        self.adjacent.borrow()
    }

    pub fn add_adjacent(&self, node: Weak<Node>) {
        self.adjacent.borrow_mut().push(node);
    }
}

pub struct Graph {
    nodes: Vec<Rc<Node>>,
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            nodes: Vec::with_capacity(size),
        }
    }

    pub fn with_nodes(nodes_list: &[i32]) -> Graph {
        let mut nodes = vec![];

        nodes_list.iter().for_each(|elem| {
            nodes.push(Rc::new(Node::new(*elem)));
        });

        Graph {
            nodes
        }
    }

    pub fn add_node(&mut self, id: usize) {
        self.nodes.insert(id - 1, Rc::new(Node::new(id as i32)));
    }

    pub fn add_adjacent(&self, id: usize, adj: usize) {
        unsafe {
            self.nodes.get_unchecked(id - 1)
                .borrow_mut()
                .add_adjacent(Rc::downgrade(self.nodes.get_unchecked(adj - 1)));
            self.nodes.get_unchecked(adj - 1)
                .borrow_mut()
                .add_adjacent(Rc::downgrade(self.nodes.get_unchecked(id - 1)));
        }
    }

    pub fn get_nodes(&self) -> &Vec<Rc<Node>> {
        &self.nodes
    }

    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

fn print_adjacent_nodes(node: &Node) {
    print!("Adjacent to node {}:", node.value);

    node.adjacent_ref().iter().for_each(|n| {
        if let Some(exists) = n.upgrade() {
            print!(" node {}", exists.value);
        }
    });

    print!("\n");
}

fn main() {
    let mut graph = Graph::new(10);

    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_adjacent(1, 2);
    graph.add_adjacent(1, 3);

    graph.get_nodes().iter().for_each(|vertex| {
        println!("Node ID: {}", vertex.value);
        print_adjacent_nodes(vertex);
    });

    graph.get_nodes().iter().for_each(|vertex| {
        println!("Node ID: {}", vertex.value);
        print_adjacent_nodes(vertex);
    });
}

#[cfg(test)]
mod test_graph {
    use super::*;

    #[test]
    fn test_empty_graph_creation() {
        let graph = Graph::new(10);

        assert_eq!(10, graph.capacity());
        assert_eq!(0, graph.size());
    }

    #[test]
    fn test_graph_with_nodes_creation() {
        let graph = Graph::with_nodes(&vec![1, 2, 3]);

        println!("{}", graph.capacity());
        assert_eq!(3, graph.size());
    }
}