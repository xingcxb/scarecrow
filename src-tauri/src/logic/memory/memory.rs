use crate::kit::enums::os_enum::OsError;
use sysinfo::System;

pub struct MemoryInfo {}

// 获取内存信息
// pub fn get_memory_info() {
//     let mut sys = System::new_all();
//     sys.refresh_memory();
//     sys.free_memory()
// }