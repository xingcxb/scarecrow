#[derive(Debug)]
pub enum OsError {
    Unknown, // 未知
    Unsupported, // 不支持操作系统
    NotImplemented, // 未实现功能
    PermissionDenied, // 权限不足
}