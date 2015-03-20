use std::num::Float;
use std::env;
use std::path::PathBuf;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

#[cfg(target_os="linux")]
pub fn user_data_dir() -> Option<PathBuf> {

    env::home_dir().map(|dir| {
        let mut path = dir.to_path_buf();
        path.push(".config");
        path.push("art");
        path
    })
}

#[cfg(target_os="mac_os")]
pub fn user_data_dir() -> Option<PathBuf> {
    env::home_dir().map(|dir| dir.join_many(&["Library", "Application Support",
                                            "art"]))
}

#[cfg(target_os="windows")]
pub fn user_data_dir() -> Option<PathBuf> {
    env::home_dir().map(|dir| dir.join_many(&["AppData", "Local", "art",
                                            "User Data"]))
}


