#![feature(core, collections, io, env, path, unsafe_destructor,
           unboxed_closures)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate "rustc-serialize" as rustc_serialize;

extern crate portaudio;

pub mod errors;
pub mod types;
pub mod constants;

pub mod options;
pub mod vm;
pub mod vm_inner;

pub mod opcode;
pub mod opcode_reader;

pub mod device;
pub mod unit_factory;

pub mod validator;

pub mod expression;
pub mod unit;
pub mod parameter;

pub mod graph;
pub mod leap;
pub mod expression_store;
pub mod channel_stack;

pub mod operators;

pub mod util;

pub mod dsp {
    pub mod oscillators {
        pub mod sine;
    }
    pub mod parameter {
        pub mod parameter;
        pub mod parameter_writer;
    }
    pub mod bus {
        pub mod bus_in;
        pub mod bus_out;
    }
}


