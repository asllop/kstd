#[macro_export]
macro_rules! print {
    ($writer:ident, $($arg:tt)*) => ({
        core::fmt::write(&mut $writer, format_args!($($arg)*)).unwrap_or(());
    })
}

#[macro_export]
macro_rules! println {
    ($writer:ident) => (print!($writer, "\n"));
    ($writer:ident, $($arg:tt)*) => (print!($writer, "{}\n", format_args!($($arg)*)));
}