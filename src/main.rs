macro_rules! message_pattern {
    ($name:ident, $pattern:expr) => {
        fn $name(msg: &str) -> bool {
            regex::Regex::new($pattern).unwrap().is_match(msg)
        }
    };
}

message_pattern!(is_hello, r"^hello .*$");
message_pattern!(is_goodbye, r"^goodbye .*$");

fn process_message(msg: &str) {
    if is_hello(msg) {
        println!("Received a hello message: {}", msg);
    } else if is_goodbye(msg) {
        println!("Received a goodbye message: {}", msg);
    } else {
        println!("Received an unknown message: {}", msg);
    }
}

fn main() {
    process_message("hello world");
    process_message("goodbye cruel world");
    process_message("foo bar");
}
