#[derive(Debug)]
pub enum OsError {
    Unknown, // 未知
    Unsupported, // 不支持操作系统
    NotImplemented, // 未实现功能
    PermissionDenied, // 权限不足
}

/// 获取错误信息
pub fn get_os_err_info(err: OsError) -> String {
    match err {
        OsError::Unknown => "未知错误".to_string(),
        OsError::Unsupported => "不支持的操作系统".to_string(),
        OsError::NotImplemented => "未实现功能".to_string(),
        OsError::PermissionDenied => "权限不足".to_string(),
    }
}