use types::ExpressionMap;

#[derive(Show)]
struct Edge {
    from: u32,
    to: u32
}

impl Edge {
    fn new(from: u32, to: u32) -> Edge {
        Edge {
            from: from,
            to: to
        }
    }
}

pub trait Node {
    fn get_edge_count(&self) -> u32;
    fn reset_edge_count(&mut self);
    fn increment_edge_count(&mut self);
    fn decrement_edge_count(&mut self);
}

pub trait NodeList {
    fn find_zero_order(&self, map: &ExpressionMap) -> Option<(usize, u32)>;
}

impl NodeList for [u32] {
    fn find_zero_order(&self, map: &ExpressionMap) -> Option<(usize, u32)> {
        self.iter().enumerate().find(|&: &(_, id) | {
            let node = map.get(id).unwrap();
            node.get_edge_count() == 0
        }).map(|(index, &id)| (index, id))
    }
}

pub struct Graph {
    // HashSet with retain would be nicer
    edges: Vec<Edge>
}

impl Graph {
    pub fn new(edge_capacity: u32) -> Graph {
        Graph {
            edges:Vec::with_capacity(edge_capacity as usize)
        }
    }

    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.push(Edge::new(from, to));
    }

    pub fn clear(&mut self) {
        self.edges.clear();
    }

    pub fn topological_sort(&mut self, map: &mut ExpressionMap,
                            nodes: &mut [u32]) {
        assert!(nodes.len() == map.len());
        self.update_edge_counts(map);

        let len = nodes.len();
        let mut start = 0;
        while start < len {
            let node_option = &nodes[start..].find_zero_order(
                map
            );

            if node_option.is_none() {
                return;
            }

            let (index, node) = node_option.unwrap();

            for edge in self.edges.iter() {
                if edge.from == node {
                    let node = map.get_mut(&edge.to).unwrap();
                    node.decrement_edge_count();
                }
            }

            nodes.swap(start + index, start);
            start += 1;
        }
    }

    fn update_edge_counts(&self, map: &mut ExpressionMap) {
        for edge in self.edges.iter() {
            let node = map.get_mut(&edge.to).unwrap();
            node.increment_edge_count();
        }
    }
}
