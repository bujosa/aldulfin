# aldulfin

This is a simple repo in rust about macros.

In this example, the message_pattern! macro takes two arguments: a name for the pattern function and the regular expression pattern to match against incoming messages.

```rust
macro_rules! message_pattern {
    ($name:ident, $pattern:expr) => {
        fn $name(msg: &str) -> bool {
            regex::Regex::new($pattern).unwrap().is_match(msg)
        }
    };
}
```

The macro defines a new function with the given name and pattern, which returns a boolean indicating whether the message matches the pattern.

```rust
message_pattern!(is_hello, r"^hello$");
message_pattern!(is_goodbye, r"^goodbye$");
```

In the process_message function, the is_hello and is_goodbye pattern functions are used to determine how to handle incoming messages.

```rust
fn process_message(msg: &str) {
    if is_hello(msg) {
        println!("Hello to you too!");
    } else if is_goodbye(msg) {
        println!("Goodbye to you too!");
    } else {
        println!("Sorry, I don't understand you.");
    }
}
```
