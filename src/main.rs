#![windows_subsystem = "windows"]
mod hook;
use crate::hook::{init_worker, start_hook};
mod key;

fn main() {
    init_worker();
    start_hook();
}
