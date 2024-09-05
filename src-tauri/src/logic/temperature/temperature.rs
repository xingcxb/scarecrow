use sysinfo::Components;
use crate::kit::enums::os_enum::OsError;

pub struct TemperatureInfo {

}

/// 获取温度信息
pub fn get_temperature_info() {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        // println!("=> component: {}", component.temperature());
        println!("{component:?}");
    }
}