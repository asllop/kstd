/// Print format to stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        let mut con = thek::controllers::stdout::StdoutController::default();
        core::fmt::write(&mut con, core::format_args!($($arg)*)).unwrap_or(());
    })
}

/// Print newline ended format to stdout
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", core::format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    () => {
        let mut con = thek::controllers::stdout::StdoutController::default();
        core::fmt::write(&mut con, core::format_args!("[{}:{}]\n", file!(), line!())).unwrap_or(())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                let mut con = thek::controllers::stdout::StdoutController::default();
                core::fmt::write(
                    &mut con,
                    core::format_args!(
                        "[{}:{}] {} = {:#?}\n",
                        file!(), line!(), stringify!($val), &tmp
                    )
                ).unwrap_or(());
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
