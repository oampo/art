#![feature(unsafe_destructor, unboxed_closures)]
#![allow(unstable)]

#[allow(unstable)]
#[macro_use] extern crate log;



extern crate portaudio;

pub mod errors;
pub mod types;
pub mod sizes;
pub mod rates;

pub mod vm;
pub mod vm_inner;

pub mod opcode;
pub mod opcode_reader;

pub mod device;
pub mod unit_factory;

pub mod expression;
pub mod unit;
pub mod parameter;

pub mod graph;
pub mod channel_stack;

pub mod util;

pub mod phases {
    pub mod process;
    pub mod link;
    pub mod sort;
    pub mod run;
    pub mod clean;
}

pub mod instructions {
    pub mod control {
        pub mod create_unit;
        pub mod set_parameter;
        pub mod add_expression;
    }
    pub mod dsp {
        pub mod unit;
        pub mod dac;
        pub mod parameter;
    }
}

pub mod dsp {
    pub mod oscillators {
        pub mod sine;
    }
}


