#[cfg(test)]
mod test {
    use sysinfo::{System, SystemExt};
    #[test]
    fn test_cpu_info() {
        let mut sys = System::new_all();
        sys.refresh_all(); // 刷新所有系统信息
        // 打印 CPU 数量
        println!("Number of CPUs: {}", sys.cpus().len());
        // 打印每个 CPU 的使用率
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("CPU {}: Usage {}%", i, cpu.cpu_usage());
        }
        // 打印总体 CPU 使用率
        println!("Total CPU Usage: {}%", sys.global_cpu_info().cpu_usage());
    }
}