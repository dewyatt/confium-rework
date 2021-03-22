use snafu::Backtrace;
use snafu::Snafu;
use std::path::PathBuf;

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("Null pointer on parameter '{}'", param))]
    NullPointer {
        param: &'static str,
        backtrace: Backtrace,
    },
    #[snafu(display("Invalid UTF-8"))]
    InvalidUTF8 { backtrace: Backtrace },
    #[snafu(display("Wrong type (expected '{}')", expected))]
    WrongType {
        expected: &'static str,
        backtrace: Backtrace,
    },
    #[snafu(display("Plugin load failed for plugin '{}'", plugin.display()))]
    PluginLoadFailed {
        plugin: PathBuf,
        source: libloading::Error,
    },
    #[snafu(display("Plugin initialization failed for plugin '{}': {}", plugin.display(), reason))]
    PluginInitializationFailed {
        plugin: PathBuf,
        reason: &'static str,
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
    UNKNOWN = 1,
    NULL_POINTER = 2,
    INVALID_UTF8 = 3,
    WRONG_TYPE = 4,
    PLUGIN_LOAD_FAILED = 5,
    PLUGIN_INIT_FAILED = 6,
}

fn error_code(error: &Error) -> u32 {
    match error {
        Error::NullPointer { .. } => ErrorCode::NULL_POINTER.into(),
        Error::InvalidUTF8 { .. } => ErrorCode::INVALID_UTF8.into(),
        Error::WrongType { .. } => ErrorCode::WRONG_TYPE.into(),
        Error::PluginLoadFailed { .. } => ErrorCode::PLUGIN_LOAD_FAILED.into(),
        Error::PluginInitializationFailed { .. } => ErrorCode::PLUGIN_INIT_FAILED.into(),
    }
}

impl From<ErrorCode> for u32 {
    #[inline]
    fn from(code: ErrorCode) -> u32 {
        code as u32
    }
}

impl From<Error> for u32 {
    #[inline]
    fn from(err: Error) -> u32 {
        error_code(&err)
    }
}
