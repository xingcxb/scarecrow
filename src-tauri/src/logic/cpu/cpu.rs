use sysinfo::{
    Components, Disks, Networks, System,
};
use crate::kit::enums::os_enum::OsError;

#[derive(Debug)]
pub struct CpuInfoBasic {
    pub cpu_name: String, // cpu 名称
    pub cpu_vendor: String, // cpu 厂商
    pub cpu_count: usize, // cpu 核心数量
    pub cpu_usage: Vec<f32>, // 各个 cpu 使用率
    pub global_cpu_usage: f32, // 全局 cpu 使用率
}

impl CpuInfoBasic {
    fn new(cpu_name: String, cpu_vendor: String, cpu_count: usize, cpu_usage: Vec<f32>, global_cpu_usage: f32) -> CpuInfoBasic {
        CpuInfoBasic {
            cpu_name,
            cpu_vendor,
            cpu_count,
            cpu_usage,
            global_cpu_usage,
        }
    }
}

/// 获取CPU信息
pub fn get_cpu_info() -> Result<CpuInfoBasic, OsError> {
    let mut sys = System::new_all();
    sys.refresh_all();
    // 获取CPU的名称
    let cpu_name = sys.cpus()[0].brand().to_string();
    // 获取CPU的厂商
    let cpu_vendor = sys.cpus()[0].vendor_id().to_string();
    // 获取CPU的总核心数
    let cpu_count = sys.physical_core_count().unwrap();
    // 获取CPU单个核心的当前使用率
    let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect::<Vec<_>>();
    // 获取全局CPU的使用率
    let global_cpu_usage = sys.global_cpu_usage();
    let _cpu_info = CpuInfoBasic::new(cpu_name, cpu_vendor, cpu_count, cpu_usage, global_cpu_usage);
    Ok(_cpu_info)
}

