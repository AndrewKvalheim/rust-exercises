pub mod graph_items;

use crate::utilities::CollectionUtilities;
use self::graph_items::edge::Edge;
use self::graph_items::node::Node;
use std::collections::HashMap;

#[derive(Default)]
pub struct Graph {
    pub attrs: HashMap<String, String>,
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            attrs: HashMap::new(),
            edges: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|&n| n.id == id)
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs.extend_from_strs(attrs);

        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges.extend_from_slice(edges);

        self
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes.extend_from_slice(nodes);

        self
    }
}
