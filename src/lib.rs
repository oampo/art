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
pub mod vm_options;

pub mod opcode;
pub mod opcode_reader;

pub mod device;
pub mod device_id;
pub mod unit_factory;

pub mod expression;
pub mod unit;
pub mod parameter;

pub mod channel_layout;
pub mod channel_stack;
pub mod util;

pub mod instructions {
    pub mod unit;
    pub mod dac;
}

pub mod dsp {
    pub mod oscillators {
        pub mod sine;
    }
}


