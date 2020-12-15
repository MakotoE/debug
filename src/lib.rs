// https://github.com/rust-lang/rust/blob/e1cce06e4ff5206daf397e1dcf91ed53653be171/library/std/src/macros.rs#L284
#[macro_export]
macro_rules! debug {
    () => {
        log::debug!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        match $val {
            tmp => {
                log::debug!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($val:expr,) => { dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}
