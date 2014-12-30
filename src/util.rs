use std::num::Float;
use std::os::getenv;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

pub fn get_int_env(name: &str) -> Option<int> {
    getenv(name).and_then(|s| s.as_slice().parse::<int>())
}

pub fn get_int_env_aliased(name: &str, alias: &str) -> Option<int> {
    getenv(name).or(getenv(alias)).and_then(|s| s.as_slice().parse::<int>())
}

