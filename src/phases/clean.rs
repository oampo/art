use vm_inner::VmInner;

pub fn clean(vm: &mut VmInner) {
    debug!("Starting clean phase");
    vm.graph.clear();
    vm.bus_map.clear();
}
