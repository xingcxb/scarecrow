use crate::kit::enums::os_enum::OsError;
use sysinfo::Disks;
use crate::kit::unit_kit::unit_kit;
use crate::kit::unit_kit::unit_kit::Unit;

#[derive(Debug)]
pub struct HardDiskInfo {
    disk_name: String, // 硬盘名称
    disk_vendor: String, // 硬盘厂商❌
    disk_model: String, // 硬盘型号❌
    disk_serial_number: String, // 硬盘序列号❌
    disk_size: String, // 硬盘容量
    disk_kind: String, // 硬盘类型
    disk_read_speed: String, // 硬盘读速度❌
    disk_write_speed: String, // 硬盘写速度❌
    disk_status: String, // 硬盘状态❌
    disk_total_space: String, // 硬盘总空间
    disk_usage: String, // 硬盘使用率
    disk_available: String, // 硬盘可用空间
    disk_file_system: String, // 硬盘文件系统
    disk_is_removable: bool, // 硬盘是否可移除
    disk_mount_point: String, // 硬盘挂载点
}

impl HardDiskInfo {
    fn new(disk_name: String, disk_vendor: String, disk_model: String,
           disk_serial_number: String, disk_size: String, disk_kind: String,
           disk_read_speed: String, disk_write_speed: String, disk_status: String,
           disk_usage: String, disk_available: String, disk_total_space: String,
           disk_file_system: String, disk_is_removable: bool, disk_mount_point: String) -> HardDiskInfo {
        HardDiskInfo {
            disk_name, // 硬盘名称
            disk_vendor, // 硬盘厂商
            disk_model, // 硬盘型号
            disk_serial_number, // 硬盘序列号
            disk_size, // 硬盘容量
            disk_kind, // 硬盘类型
            disk_read_speed, // 硬盘读速度
            disk_write_speed, // 硬盘写速度
            disk_status, // 硬盘状态
            disk_total_space, // 硬盘总空间
            disk_usage, // 硬盘使用率
            disk_available, // 硬盘可用空间
            disk_file_system, // 硬盘文件系统
            disk_is_removable, // 硬盘是否可移除
            disk_mount_point, // 硬盘挂载点
        }
    }
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
        let disk_info = HardDiskInfo::new(
            disk.name().to_str().unwrap().to_string(),
            "".to_string(), "".to_string(), "".to_string(),
            _disk_total_space.to_string(), disk.kind().to_string(),
            "".to_string(), "".to_string(), "".to_string(), 
            (_disk_used_space / _disk_total_space * 100.0).to_string(), 
            _disk_available_space.to_string(),_disk_total_space.to_string(),
            disk.file_system().to_str().unwrap().to_string(),
            disk.is_removable(), disk.mount_point().to_str().unwrap().to_string());
        disk_infos.push(disk_info);
    };
    Ok(disk_infos)
}