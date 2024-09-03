// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scarecrow_lib::logic::cpu::cpu::get_cpu_info;
use scarecrow_lib::logic::os::os::get_os_info;
use scarecrow_lib::logic::hard_disk::hard_disk::get_hard_disk_info;

fn main() {
    let cpu_info = get_cpu_info();
    let os_info = get_os_info();
    let disk_info = get_hard_disk_info();
    println!("CPU Info: {:?}\n", cpu_info);
    println!("os Info: {:?}\n",os_info);
    println!("Disk Info: {:?}\n", disk_info);
    scarecrow_lib::run();
}
