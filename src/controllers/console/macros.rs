/// Print format to stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        let mut con = DefaultConsoleController::default();
        core::fmt::write(&mut con, core::format_args!($($arg)*)).unwrap_or(());
    })
}

/// Print newline ended format to stdout
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", core::format_args!($($arg)*)));
}