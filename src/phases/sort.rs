use vm_inner::VmInner;

use graph::Node;

pub trait Sort {
    fn sort(&mut self);
}

impl Sort for VmInner {
    fn sort(&mut self) {
        debug!("Starting sort phase");
        for (_, expression) in self.expressions.iter_mut() {
            expression.reset_edge_count();
        }

        self.graph.topological_sort(&mut self.expressions,
                                    self.expression_ids.as_mut_slice());
    }
}
