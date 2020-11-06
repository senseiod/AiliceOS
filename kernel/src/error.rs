#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AcoreError {
    Ok = 0,
    Internel = -1,
    NotSupported = -2,
    NoResources = -3,
    NoMemory = -4,
    InvalidArgs = -5,
    OutOfRange = -6,
    BadState = -7,
    NotFound = -8,
    AlreadyExists = -9,
    AccessDenied = -10,
}

pub type AcoreResult<T = ()> = Result<T, AcoreError>;
