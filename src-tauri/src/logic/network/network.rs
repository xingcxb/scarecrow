use std::time::Duration;
use sysinfo::Networks;
use crate::kit::unit_kit::unit_kit;
use crate::kit::unit_kit::unit_kit::{Carry, Unit};

/// 获取网络信息
pub fn get_network_info() {
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
    }
    println!("Total: {} B (down) / {} B (up)",
             networks.list().get("en7").unwrap().total_received(),
             networks.list().get("en7").unwrap().total_transmitted())
}

/// 获取网速
pub fn get_network_speed() {
    let mut networks = Networks::new_with_refreshed_list();
    // 第一次的下载数据
    let first_received = networks.list().get("en7").unwrap().total_received();
    // 第一次的上传数据
    let first_transmitted = networks.list().get("en7").unwrap().total_transmitted();
    std::thread::sleep(Duration::from_secs(1));
    // 刷新网络信息
    networks.refresh();
    // 第二次的下载数据
    let second_received = networks.list().get("en7").unwrap().total_received();
    // 第二次的上传数据
    let second_transmitted = networks.list().get("en7").unwrap().total_transmitted();

    let down_speed_b = second_received - first_received;
    let up_speed_b = second_transmitted - first_transmitted;
    let down_speed = unit_kit::bit_unit(down_speed_b, Carry::Binary, Unit::KB);
    let up_speed = unit_kit::bit_unit(up_speed_b, Carry::Binary, Unit::KB);
    println!("网速: 下载 {} KB/s, 上传 {} KB/s", down_speed, up_speed)
}