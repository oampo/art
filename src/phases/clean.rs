use vm_inner::VmInner;

pub trait Clean {
    fn clean(&mut self);
}

impl Clean for VmInner {
    fn clean(&mut self) {
        debug!("Starting clean phase");
        self.graph.clear();
        self.bus_map.clear();
    }
}
