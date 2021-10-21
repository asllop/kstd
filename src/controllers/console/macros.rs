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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        {
            let mut con = ScreenConsole::default();
            core::fmt::write(&mut con, format_args!($($arg)*)).unwrap_or(());
        }
    })
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}