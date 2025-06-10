// Macro print orange

#[macro_export]
macro_rules! println_orange {
    ($($arg:tt)*) => {{
        println!("[aws-sso-rs] - {}", format!($($arg)*).truecolor(255, 165, 0));
    }};
}
