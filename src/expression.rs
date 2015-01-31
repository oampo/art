use graph::Node;

#[derive(Copy)]
pub enum ExpressionState {
    Verify,
    Construct,
    Link,
    Run,
    Free
}

#[derive(Copy)]
pub struct Expression {
    pub id: u32,
    pub index: usize,
    incoming_edges: u32,
    pub state: ExpressionState
}

impl Expression {
    pub fn new(id: u32, index: usize) -> Expression {
        Expression {
            id: id,
            index: index,
            incoming_edges: 0,
            state: ExpressionState::Verify
        }
    }
}

impl Node for Expression {
    fn get_edge_count(&self) -> u32 {
        self.incoming_edges
    }

    fn reset_edge_count(&mut self) {
        self.incoming_edges = 0;
    }

    fn increment_edge_count(&mut self) {
        self.incoming_edges += 1;
    }

    fn decrement_edge_count(&mut self) {
        self.incoming_edges -= 1;
    }
}
