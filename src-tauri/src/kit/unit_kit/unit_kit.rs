use num::pow::Pow;

/// 单位枚举
pub enum Unit {
    /// Byte
    B,
    /// KiloByte
    KB,
    /// MegaByte
    MB,
    /// GigaByte
    GB,
    /// TeraByte
    TB,
}

/// 进制枚举
pub enum Carry {
    /// 二进制
    Binary,
    /// 十进制
    Decimal,
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
pub fn bit_unit(v: u64, carry: Carry, unit: Unit) -> f64 {
    match unit {
        Unit::B => {
            v as f64
        }
        Unit::KB => {
            if let Carry::Binary = carry {
                // 二进制
                return v as f64 / 1024f64;
            }
            // 十进制
            v as f64 / 1e3
        }
        Unit::MB => {
            if let Carry::Binary = carry {
                // 二进制
                return v as f64 / 1024f64.pow(2);
            }
            // 十进制
            v as f64 / 1e6
        }
        Unit::GB => {
            if let Carry::Binary = carry {
                // 二进制
                return v as f64 / 1024f64.pow(3);
            }
            // 十进制
            v as f64 / 1e9
        }
        Unit::TB => {
            if let Carry::Binary = carry {
                // 二进制
                return v as f64 / 1024f64.pow(4);
            }
            // 十进制
            v as f64 / 1e12
        }
    }
}