use std::old_io::BufReader;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VmInner;
use opcode::{Opcode, ControlOpcode};
use opcode_reader::OpcodeReader;

pub fn process(vm: &mut VmInner) {
    debug!("Starting process phase");
    let result = vm.input_channel.try_recv();
    if let Ok(byte_code) = result {
        let result = process_byte_code(
            vm, &byte_code.data[..byte_code.size]
        );
        result.unwrap_or_else(|error| error!("{}", error));
    }
}

fn process_byte_code(vm: &mut VmInner, byte_code: &[u8]) -> ArtResult<()> {
    let mut reader = BufReader::new(byte_code);
    while !reader.eof() {
        try!(process_opcode(vm, &mut reader));
    }
    Ok(())
}

fn process_opcode(vm: &mut VmInner, reader: &mut BufReader) -> ArtResult<()> {
    let opcode = try!(
        reader.read_control_opcode()
    );

    match opcode {
        ControlOpcode::AddExpression { expression_id, num_opcodes } => {
            let start = try!(
                vm.expression_store.push_start(num_opcodes as usize)
            );

            let result = process_expression(vm, reader, num_opcodes);

            if result.is_err() {
                try!(vm.expression_store.remove(start));
                return result;
            }

            vm.add_expression(expression_id, start)
        },
        ControlOpcode::SetParameter { expression_id, unit_id,
                                      parameter_id, value } => {
            vm.set_parameter((expression_id, unit_id, parameter_id), value)
        },
        _ => Err(ArtError::UnimplementedOpcode {
            opcode: Opcode::Control(opcode)
        })
    }
}

fn process_expression(vm: &mut VmInner, reader: &mut BufReader,
                      num_opcodes: u32) -> ArtResult<()> {
    for _ in range(0, num_opcodes) {
        let opcode = try!(reader.read_dsp_opcode());
        try!(vm.expression_store.push(opcode));
    }
    Ok(())
}
