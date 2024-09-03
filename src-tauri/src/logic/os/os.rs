use sysinfo::System;
use crate::kit::enums::os_enum::OsError;

#[derive(Debug)]
pub struct OsInfo {
    pub os_name: String,
    pub os_version: String,
    pub os_arch: String,
    pub os_kernel: String,
    pub os_uptime: String,
    pub os_hostname: String,
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

enum OsName {
    MacOS,
    Windows,
    Linux,
    Other,
}

pub fn get_os_info() -> Result<OsInfo, OsError> {
    let mut os_name = System::name().unwrap().to_string();
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
