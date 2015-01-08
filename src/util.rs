use std::num::Float;
use std::os::getenv;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

