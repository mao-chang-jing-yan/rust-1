// mod graph;
// use graph::node::Node;
use std::fs;


fn main() {
    let text = fs::read_to_string("test.txt").unwrap();
    println!("{}", text);
}
