use vm_inner::VmInner;

use graph::Node;

pub fn sort(vm: &mut VmInner) {
    debug!("Starting sort phase");
    for (_, expression) in vm.expressions.iter_mut() {
        expression.reset_edge_count();
    }

    vm.graph.topological_sort(&mut vm.expressions,
                              vm.expression_ids.as_mut_slice());
}
