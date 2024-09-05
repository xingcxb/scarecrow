// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scarecrow_lib::logic::cpu::cpu::get_cpu_info;
use scarecrow_lib::logic::os::os::get_os_info;
use scarecrow_lib::logic::hard_disk::hard_disk::get_hard_disk_info;
use scarecrow_lib::logic::memory::memory::get_memory_info;
use scarecrow_lib::logic::network::network::{get_network_info, get_network_speed};
use scarecrow_lib::logic::temperature::temperature::get_temperature_info;

fn main() {
    let cpu_info = get_cpu_info();
    let os_info = get_os_info();
    let disk_info = get_hard_disk_info();
    let memory_info = get_memory_info();
    // let temperature_info = get_temperature_info();
    let network_info = get_network_info();
    let network_speed = get_network_speed();
    println!("CPU Info: {:?}\n", cpu_info);
    println!("os Info: {:?}\n", os_info);
    println!("Disk Info: {:?}\n", disk_info);
    println!("Memory Info: {:?}\n", memory_info);
    // println!("temperature Info: {:?}\n", temperature_info);
    println!("Network Info: {:?}\n", network_info);
    println!("Network Speed: {:?}\n", network_speed);
    scarecrow_lib::run();
}
