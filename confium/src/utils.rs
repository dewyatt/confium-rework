#[macro_escape]
macro_rules! err_common {
    ($($source:tt)+) => {{
        $crate::error::ErrorCommon {
            source: $($source)+,
            backtrace: Some(::std::backtrace::Backtrace::capture()),
        }
    }};
}

#[macro_escape]
macro_rules! check_not_null {
    ($param:ident) => {{
        if $param.is_null() {
            return Err($crate::error::Error::NullPointer {
                common: err_common!(None),
                param: stringify!($param),
            });
        }
    }};
}
