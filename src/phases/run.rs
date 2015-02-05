use std::mem;

use vm_inner::VmInner;
use channel_stack::ChannelStack;

pub fn run(vm: &mut VmInner, busses: &mut ChannelStack)  {
    debug!("Starting run phase");

    let mut expression_ids = Vec::<u32>::with_capacity(0);
    mem::swap(&mut vm.expression_ids, &mut expression_ids);

    for id in expression_ids.iter() {
        let expression = vm.expressions.get(id).unwrap();
        let mut stack = ChannelStack::new(&mut vm.stack_data,
                                          vm.constants.block_size);
        let result = expression.tick(&vm.expression_store, &mut stack,
                                     busses, &mut vm.units, &mut vm.parameters,
                                     &mut vm.bus_map, &vm.constants);
        result.unwrap_or_else(|error| error!("{}", error));
    }

    mem::swap(&mut vm.expression_ids, &mut expression_ids);
}

