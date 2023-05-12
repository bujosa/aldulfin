#[macro_export]
macro_rules! message_pattern {
    ($name:ident, $pattern:expr) => {
        fn $name(msg: &str) -> bool {
            regex::Regex::new($pattern).unwrap().is_match(msg)
        }
    };
}

