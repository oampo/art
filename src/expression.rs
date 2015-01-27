use opcode::DspOpcode;
use graph::Node;

#[derive(Copy)]
pub enum ExpressionState {
    Verify,
    Construct,
    Link,
    Run,
    Free
}

pub struct Expression {
    pub id: u32,
    pub opcodes: Vec<DspOpcode>,
    incoming_edges: u32,
    pub state: ExpressionState
}

impl Expression {
    pub fn new(id: u32, opcodes: Vec<DspOpcode>) -> Expression {
        Expression {
            id: id,
            opcodes: opcodes,
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
