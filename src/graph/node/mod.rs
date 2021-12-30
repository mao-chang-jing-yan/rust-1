use crate::graph::edge;
use crate::graph::edge::Edge;

pub struct Node {
    pub(crate) name: String,
}

impl Node {
    pub fn print(&self) -> &str {
        print!("{}", self.name);
        return &self.name[..];
    }
}

fn te() {
    let e = Edge {
        p_node: Node { name: String::from("") },
        n_node: Node { name: String::from("") },
        weight: 0,
    };
}