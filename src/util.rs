use std::num::Float;
use std::os;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

#[cfg(target_os="linux")]
pub fn user_data_dir() -> Option<Path> {
    os::homedir().map(|dir| dir.join_many(&[".config", "art"]))
}

#[cfg(target_os="mac_os")]
pub fn user_data_dir() -> Path {
    os::homedir().map(|dir| dir.join_many(&["Library", "Application Support",
                                            "art"]))
}

#[cfg(target_os="windows")]
pub fn user_data_dir() -> Path {
    os::homedir().map(|dir| dir.join_many(&["AppData", "Local", "art",
                                            "User Data"]))
}


