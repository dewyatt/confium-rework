use std::backtrace::Backtrace;

pub type Result<T> = std::result::Result<T, Error>;

pub struct ErrorCommon {
    pub source: Option<Box<Error>>,
    pub backtrace: Option<Backtrace>,
}

pub enum Error {
    NullPointer {
        common: ErrorCommon,
        param: &'static str,
    },
    InvalidUTF8 {
        common: ErrorCommon,
    },
}

impl Error {
    pub fn code(&self) -> u32 {
        error_code(&self)
    }
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ErrorCode {
    NULL_POINTER = 1,
    INVALID_UTF8 = 2,
}

fn error_code(error: &Error) -> u32 {
    match error {
        Error::NullPointer { .. } => ErrorCode::NULL_POINTER.into(),
        Error::InvalidUTF8 { .. } => ErrorCode::INVALID_UTF8.into(),
    }
}

impl From<ErrorCode> for u32 {
    #[inline]
    fn from(code: ErrorCode) -> u32 {
        code as u32
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NullPointer { param, .. } => {
                write!(f, "Null pointer (parameter '{}')", param)
            }
            Error::InvalidUTF8 { .. } => {
                write!(f, "Invalid UTF-8")
            }
        }
    }
}
