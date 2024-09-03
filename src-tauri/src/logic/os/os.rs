use sysinfo::System;
use crate::kit::enums::os_enum::OsError;

#[derive(Debug)]
pub struct OsInfo {
    pub os_name: String, // 操作系统名称
    pub os_version: String, // 操作系统版本
    pub os_arch: String, // 操作系统架构
    pub os_kernel: String, // 操作系统内核版本
    pub os_uptime: String, // 操作系统运行时间
    pub os_hostname: String, // 操作系统主机名
}

impl OsInfo {
    pub fn new(os_name: String, os_version: String,
               os_arch: String, os_kernel: String,
               os_uptime: String, os_hostname: String) -> OsInfo {
        OsInfo {
            os_name,
            os_version,
            os_arch,
            os_kernel,
            os_uptime,
            os_hostname,
        }
    }
}


/// 获取操作系统信息
pub fn get_os_info() -> Result<OsInfo, OsError> {
    let os_name;
    match System::name().unwrap().to_string().as_str() {
        "Darwin" => os_name = "MacOS".to_string(),
        "Windows" => os_name = "Windows".to_string(),
        "Linux" => os_name = "Linux".to_string(),
        _ => os_name = "Other".to_string(),
    }
    let _os_info = OsInfo::new(os_name, System::os_version().unwrap().to_string(),
                               System::cpu_arch().unwrap().to_string(),
                               System::kernel_version().unwrap().to_string(),
                               System::uptime().to_string(),
                               System::host_name().unwrap().to_string());
    Ok(_os_info)
}
