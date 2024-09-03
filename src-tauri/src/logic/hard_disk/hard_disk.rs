use crate::kit::enums::os_enum::OsError;
use sysinfo::Disks;
use crate::kit::unit_kit::unit_kit;
use crate::kit::unit_kit::unit_kit::Unit;

#[derive(Debug)]
pub struct HardDiskInfo {
    disk_name: String, // 硬盘名称
    disk_vendor: String, // 硬盘厂商
    disk_model: String, // 硬盘型号
    disk_serial_number: String, // 硬盘序列号
    disk_size: String, // 硬盘容量
    disk_kind: String, // 硬盘类型
    disk_read_speed: String, // 硬盘读速度
    disk_write_speed: String, // 硬盘写速度
    disk_usage: String, // 硬盘使用率
    disk_status: String, // 硬盘状态
    disk_available: String, // 硬盘可用空间
    disk_file_system: String, // 硬盘文件系统
    disk_is_removable: bool, // 硬盘是否可移除
    disk_mount_point: String, // 硬盘挂载点
}

/// 获取硬盘信息
pub fn get_hard_disk_info() -> Result<Vec<HardDiskInfo>, OsError> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_infos: Vec<HardDiskInfo> = Vec::new();
    for disk in &disks {
        // 获取硬盘容量
        let _disk_total_space = unit_kit::convert_unit(disk.total_space(), Unit::GB);
        // 获取硬盘可用空间
        let _disk_available_space = unit_kit::convert_unit(disk.available_space(), Unit::GB);
        // 已使用的空间
        let _disk_used_space = _disk_total_space - _disk_available_space;
        let disk_info = HardDiskInfo {
            disk_name: disk.name().to_str().unwrap().to_string(),
            disk_vendor: "".to_string(),
            disk_model: "".to_string(),
            disk_serial_number: "".to_string(),
            disk_kind: disk.kind().to_string(),
            disk_read_speed: "".to_string(),
            disk_write_speed: "".to_string(),
            disk_size: _disk_total_space.to_string(),
            disk_available: _disk_available_space.to_string(),
            disk_usage: (_disk_used_space / _disk_total_space * 100.0).to_string(),
            disk_status: "".to_string(),
            disk_file_system: disk.file_system().to_str().unwrap().to_string(),
            disk_is_removable: disk.is_removable(),
            disk_mount_point: disk.mount_point().to_str().unwrap().to_string(),
        };
        disk_infos.push(disk_info);
    };
    Ok(disk_infos)
}