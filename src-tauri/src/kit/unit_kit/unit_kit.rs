pub enum Unit {
    B, // Byte
    KB, // KiloByte
    MB, // MegaByte
    GB, // GigaByte
    TB, // TeraByte
}

/// 容量单位转换
/// **Important**: 默认为字节转换到指定单位
///
/// # Arguments
/// v : 待转换的容量大小
///
/// unit: 转换到的单位
/// # Returns
/// 转换后的数值
pub fn convert_unit(v: u64, unit: Unit) -> f64 {
    match unit {
        Unit::B => {
            v as f64
        }
        Unit::KB => {
            v as f64 / 1e3
        }
        Unit::MB => {
            v as f64 / 1e6
        }
        Unit::GB => {
            v as f64 / 1e9
        }
        Unit::TB => {
            v as f64 / 1e12
        }
    }
}