extern crate art;
use art::vm::VM;
use art::device::Device;

use art::dsp::oscillators::sine;
//use art::util::Ascii4;
//use art::opcode::Opcode;

fn main() {
    Device::init();
    Device::list();
    let (tx, rx) = channel();

    let mut vm: VM = VM::new(rx);
    let device = vm.run().unwrap();
    std::io::stdin().read_line();
    Device::uninit();
}
