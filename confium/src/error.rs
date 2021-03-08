use std::backtrace::Backtrace;

pub struct ErrorCommon {
    pub source: Option<Box<Error>>,
    pub backtrace: Option<Backtrace>,
}

pub enum Error {
    Unknown {
        common: ErrorCommon,
    },
    NullPointer {
        common: ErrorCommon,
        param: &'static str,
    },
    InvalidUTF8 {
        common: ErrorCommon,
    },
    WrongType {
        common: ErrorCommon,
        expected: &'static str,
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
    UNKNOWN = 1,
    NULL_POINTER = 2,
    INVALID_UTF8 = 3,
    WRONG_TYPE = 4,
}

fn error_code(error: &Error) -> u32 {
    match error {
        Error::Unknown { .. } => ErrorCode::UNKNOWN.into(),
        Error::NullPointer { .. } => ErrorCode::NULL_POINTER.into(),
        Error::InvalidUTF8 { .. } => ErrorCode::INVALID_UTF8.into(),
        Error::WrongType { .. } => ErrorCode::WRONG_TYPE.into(),
    }
}

fn error_common(error: &Error) -> &ErrorCommon {
    match error {
        Error::Unknown { common, .. } => common,
        Error::NullPointer { common, .. } => common,
        Error::InvalidUTF8 { common, .. } => common,
        Error::WrongType { common, .. } => common,
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
            Error::Unknown { .. } => {
                write!(f, "Unknown error")
            }
            Error::NullPointer { param, .. } => {
                write!(f, "Null pointer (parameter '{}')", param)
            }
            Error::InvalidUTF8 { .. } => {
                write!(f, "Invalid UTF-8")
            }
            Error::WrongType { expected, .. } => {
                write!(f, "Wrong type (expected '{}')", expected)
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)?;
        if let Some(bt) = self.backtrace() {
            if std::backtrace::BacktraceStatus::Captured == bt.status() {
                write!(f, "{}", bt.to_string())?;
            }
        }
        while let Some(src) = self.source() {
            write!(f, "{:?}", src)?;
        }
        Ok(())
    }
}
