use sysinfo::{
    Components, Disks, Networks, System,
};

pub struct CpuInfoBasic {
    pub cpu_name: String, // cpu 名称
    pub cpu_count: usize, // cpu 核心数量
    pub cpu_user_time: u64, // cpu 用户态时间
}

pub fn get_cpu_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("cpu name: {:?}", sys.cpus());
    println!("cpu count: {}", sys.cpus().len());
    println!("cpu info: {:?}", sys.cpus());
    return "hello world".to_string();
}

