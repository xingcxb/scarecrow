// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scarecrow_lib::logic::cpu::cpu::get_cpu_info;

fn main() {
    let cpu_info = get_cpu_info();
    println!("CPU Info: {}", cpu_info);
    scarecrow_lib::run();
}
