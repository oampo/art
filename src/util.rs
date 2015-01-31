use std::num::Float;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}


