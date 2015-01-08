#![feature(unsafe_destructor)]
#[macro_use] extern crate log;

extern crate portaudio;

pub mod errors;
pub mod types;
pub mod sizes;
pub mod rates;
pub mod vm;
pub mod vm_options;
pub mod opcode;
pub mod opcode_reader;
pub mod expression;
pub mod tickable;
pub mod unit_factory;
pub mod channel_layout;
pub mod util;
pub mod device;
pub mod channel_stack;

pub mod instructions {
    pub mod unit;
    pub mod dac;
}

pub mod dsp {
    pub mod oscillators {
        pub mod sine;
    }
}


