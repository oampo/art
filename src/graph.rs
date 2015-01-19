use std::hash::{Hash, Hasher, Writer};
use std::collections::HashSet;

use types::ExpressionMap;
use util::HashSetRetain;

#[derive(PartialEq, Eq)]
struct Edge {
    from: u32,
    to: u32
}

impl<H: Hasher + Writer> Hash<H> for Edge {
    fn hash(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

pub struct Graph {
    edges: HashSet<Edge>
}

impl Graph {
    pub fn new(node_capacity:u32, edge_capacity: u32) -> Graph {
        Graph {
            edges: HashSet::with_capacity(edge_capacity as usize)
        }
    }

    pub fn topological_sort(&mut self, expressions: &mut ExpressionMap, nodes: &mut [u32]) {
        assert(nodes.len() == expressions.len());

        self.remove_dangling_edges(expressions);
        self.update_edge_counts(expressions);
        self.set_node_ids(expressions, nodes);

        while nodes.len() > 0 {
            let node_option = self.find_zero_order_node(expressions, nodes);
            match node_option {
                Some((index, node)) => {
                    for edge in self.edges.iter() {
                        if edge.to == node {
                            expressions.get_mut(&edge.from).unwrap().incoming_edges -= 1;
                        }
                    }
                    // Move the node to the start
                    nodes.swap(index, 0);
                    position += 1;
                    nodes = nodes.slice_from_mut(1);
                },
                None => return
            }
        }
    }

    fn remove_dangling_edges(&mut self, expressions: &ExpressionMap) {
        self.edges.retain(|&: edge|
            expressions.contains_key(&edge.to) &&
            expressions.contains_key(&edge.from)
        );
    }

    fn zero_edge_counts(&self, expressions: &mut ExpressionMap) {
        for (_, expression) in expressions.iter_mut() {
            expression.incoming_edges = 0;
        }
    }

    fn update_edge_counts(&self, expressions: &mut ExpressionMap) {
        self.zero_edge_counts(expressions);
        for edge in self.edges.iter() {
            let expression = expressions.get_mut(&edge.to).unwrap();
            expression.incoming_edges += 1;
        }
    }

    fn set_node_ids(&self, expressions: &ExpressionMap, nodes: &mut[u32]) {
        for (id, _) in expressions.iter() {
            nodes[i] = *id;
        }
    }

    fn find_zero_order_node(&self, expressions: &ExpressionMap, nodes: &[u32])
            -> Option<usize, u32> {
        nodes.iter().enumerate().find(|&: (index, id) |
            expressions.get(*id).unwrap().incoming_edges == 0
        ).map(|(index, &id)| (index, id))
    }
}
