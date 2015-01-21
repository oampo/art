use vm_inner::VMInner;

use graph::Node;

pub trait Sort {
    fn sort(&mut self);
}

impl Sort for VMInner {
    fn sort(&mut self) {
        self.expression_ids.clear();

        for (id, expression) in self.expressions.iter_mut() {
            self.expression_ids.push(*id);
            expression.reset_edge_count();
        }

        self.graph.topological_sort(&mut self.expressions,
                                    self.expression_ids.as_mut_slice());
    }
}
