#[macro_export]
macro_rules! w_print {
    ($writer:ident, $($arg:tt)*) => ({
        core::fmt::write(&mut $writer, format_args!($($arg)*)).unwrap_or(());
    })
}

#[macro_export]
macro_rules! w_println {
    ($writer:ident) => (w_print!($writer, "\n"));
    ($writer:ident, $($arg:tt)*) => (w_print!($writer, "{}\n", format_args!($($arg)*)));
}