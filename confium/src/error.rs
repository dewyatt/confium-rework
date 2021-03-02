use std::backtrace::Backtrace;

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
    Overflow {
        common: ErrorCommon,
    },
    InvalidFormat {
        common: ErrorCommon,
    },
    InvalidHexDigit {
        common: ErrorCommon,
        ch: char,
    },
}

impl Error {
    pub fn code(&self) -> u32 {
        error_code(&self)
    }

    pub fn source(&self) -> &Option<Box<Error>> {
        &error_common(&self).source
    }

    pub fn backtrace(&self) -> &Option<Backtrace> {
        &error_common(&self).backtrace
    }
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ErrorCode {
    NULL_POINTER = 1,
    INVALID_UTF8 = 2,
    OVERFLOW = 3,
    INVALID_HEX_DIGIT = 4,
    INVALID_FORMAT = 5,
}

fn error_code(error: &Error) -> u32 {
    match error {
        Error::NullPointer { .. } => ErrorCode::NULL_POINTER.into(),
        Error::InvalidUTF8 { .. } => ErrorCode::INVALID_UTF8.into(),
        Error::Overflow { .. } => ErrorCode::OVERFLOW.into(),
        Error::InvalidHexDigit { .. } => ErrorCode::INVALID_HEX_DIGIT.into(),
        Error::InvalidFormat { .. } => ErrorCode::INVALID_FORMAT.into(),
    }
}

fn error_common(error: &Error) -> &ErrorCommon {
    match error {
        Error::NullPointer { common, .. } => common,
        Error::InvalidUTF8 { common, .. } => common,
        Error::Overflow { common, .. } => common,
        Error::InvalidHexDigit { common, .. } => common,
        Error::InvalidFormat { common, .. } => common,
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
            Error::Overflow { .. } => {
                write!(f, "Overflow")
            }
            Error::InvalidHexDigit { ch, .. } => {
                write!(f, "Invalid hex digit '{}'", ch)
            }
            Error::InvalidFormat { .. } => {
                write!(f, "Invalid format")
            }
        }
    }
}
