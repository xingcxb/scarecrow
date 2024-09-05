use crate::kit::enums::os_enum::OsError;
use crate::kit::unit_kit::*;
use sysinfo::System;

#[derive(Debug)]
pub struct MemoryInfo {
    total: f64, // 总内存
    free: f64, // 剩余内存
    used: f64, // 已使用内存
    total_swap: f64, // 总交换内存
    free_swap: f64, // 剩余交换内存
    used_swap: f64, // 已使用交换内存
}

impl MemoryInfo {
    fn new(total: f64, free: f64, used: f64, total_swap: f64, free_swap: f64, used_swap: f64) -> MemoryInfo {
        MemoryInfo {
            total,
            free,
            used,
            total_swap,
            free_swap,
            used_swap,
        }
    }
}

/// 获取内存信息
pub fn get_memory_info() -> Result<MemoryInfo, OsError> {
    let mut sys = System::new_all();
    sys.refresh_memory();
    // 总内存
    let total = sys.total_memory();
    let total_f = unit_kit::bit_unit(total, unit_kit::Carry::Binary, unit_kit::Unit::GB);
    // 剩余内存
    let free = sys.free_memory();
    let free_f = unit_kit::bit_unit(free, unit_kit::Carry::Binary, unit_kit::Unit::GB);
    // 已使用内存
    let used = sys.used_memory();
    let used_f = unit_kit::bit_unit(used, unit_kit::Carry::Binary, unit_kit::Unit::GB);
    // 交换总内存
    let total_swap = sys.total_swap();
    let total_swap_f = unit_kit::bit_unit(total_swap, unit_kit::Carry::Binary, unit_kit::Unit::GB);
    // 交换剩余内存
    let free_swap = sys.free_swap();
    let free_swap_f = unit_kit::bit_unit(free_swap, unit_kit::Carry::Binary, unit_kit::Unit::GB);
    // 交换已使用内存
    let used_swap = sys.used_swap();
    let used_swap_f = unit_kit::bit_unit(used_swap, unit_kit::Carry::Binary, unit_kit::Unit::GB);
    let memory_info = MemoryInfo::new(total_f, free_f,
                                      used_f, total_swap_f,
                                      free_swap_f, used_swap_f);
    Ok(memory_info)
}