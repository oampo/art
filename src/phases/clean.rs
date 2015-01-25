use vm_inner::VMInner;

pub trait Clean {
    fn clean(&mut self);
}

impl Clean for VMInner {
    fn clean(&mut self) {
        debug!("Starting clean phase");
        self.graph.clear();
    }
}
