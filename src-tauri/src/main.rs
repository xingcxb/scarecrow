// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scarecrow_lib::logic::cpu::cpu::get_cpu_info;
use scarecrow_lib::logic::os::os::get_os_info;

fn main() {
    let cpu_info = get_cpu_info();
    let os_info = get_os_info();
    println!("CPU Info: {:?}", cpu_info);
    println!("test {:?}",os_info);
    scarecrow_lib::run();
}
