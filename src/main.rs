//#![windows_subsystem = "windows"]

pub mod elf;
pub mod gui;
pub mod utils;

fn main() {
    crate::gui::run();
}
