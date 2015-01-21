use opcode::DspOpcode;
use graph::Node;

pub struct Expression {
    pub opcodes: Vec<DspOpcode>,
    incoming_edges: u32
}

impl Expression {
    pub fn new(opcodes: Vec<DspOpcode>) -> Expression {
        Expression {
            opcodes: opcodes,
            incoming_edges: 0
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
