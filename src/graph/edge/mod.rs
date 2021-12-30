use crate::graph::node;

pub struct Edge {
    pub(crate) p_node: node::Node,
    pub(crate) n_node: node::Node,
    pub(crate) weight: u8,
}

impl Edge {
    fn print(&self) {
        print!("{}->{} {}", self.p_node.print(), self.n_node.print(), self.weight);
    }
}