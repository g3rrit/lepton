
macro_rules! comp_warn {
    ($fmt:expr) => (print!(concat!("Compiler Warning|: ", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("Compiler Warning|: ", $fmt, "\n"), $($arg)*));
}
