use std::backtrace::Backtrace;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct ErrorCommon {
    pub source: Option<Box<Error>>,
    pub backtrace: Option<String>,
}

#[derive(Clone)]
pub enum Error {
    NullPointer { common: ErrorCommon },
    InvalidUTF8 { common: ErrorCommon },
}
