/// Print format using writer
#[macro_export]
macro_rules! w_print {
    ($writer:ident, $($arg:tt)*) => ({
        core::fmt::write(&mut $writer, format_args!($($arg)*)).unwrap_or(());
    })
}

/// Print newline ended format using writer
#[macro_export]
macro_rules! w_println {
    ($writer:ident) => (w_print!($writer, "\n"));
    ($writer:ident, $($arg:tt)*) => (w_print!($writer, "{}\n", format_args!($($arg)*)));
}

/// Print format to stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        let mut con = DefaultConsoleController::default();
        core::fmt::write(&mut con, format_args!($($arg)*)).unwrap_or(());
    })
}

/// Print newline ended format to stdout
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}