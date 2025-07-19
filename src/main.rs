#![windows_subsystem = "windows"]
mod hook;
use crate::hook::{init_worker, start_hook};
mod key;
use std::env;
use winreg::{
    enums::{
        HKEY_CURRENT_USER,
        KEY_WRITE,
    },
    RegKey,
};

fn add_to_startup(name: &str) {
    let exe_path = env::current_exe().unwrap();
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE).unwrap();

    let existing: Result<String, _> = run_key.get_value(name);
    if existing.is_err() {
        run_key.set_value(name, &exe_path.to_str().unwrap()).unwrap();
    }
}

fn main() {
    add_to_startup("better_esc");
    init_worker();
    start_hook();
}
