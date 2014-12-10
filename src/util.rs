use std::num::Int;
use std::num::Float;
use std::os::getenv;

pub trait Ascii4 for Sized? {
    fn to_u32(&self) -> u32;
}

impl Ascii4 for [Ascii] {
    fn to_u32(&self) -> u32 {
        if self.len() != 4 {
            panic!("Cannot to convert Ascii array of length != 4 to u32");
        }
        Int::from_le((self[0].as_byte() as u32) << 0  |
                     (self[1].as_byte() as u32) << 8  |
                     (self[2].as_byte() as u32) << 16 |
                     (self[3].as_byte() as u32) << 24)
    }
}

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

pub fn get_int_env(name: &str) -> Option<int> {
    getenv(name).and_then(|s| from_str(s.as_slice()))
}

pub fn get_int_env_aliased(name: &str, alias: &str) -> Option<int> {
    getenv(name).or(getenv(alias)).and_then(|s| from_str(s.as_slice()))
}

