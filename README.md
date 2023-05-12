# aldulfin

This is a simple repo in rust about macros.

In this example, the message_pattern! macro takes two arguments: a name for the pattern function and the regular expression pattern to match against incoming messages.

The macro defines a new function with the given name and pattern, which returns a boolean indicating whether the message matches the pattern.

In the process_message function, the is_hello and is_goodbye pattern functions are used to determine how to handle incoming messages.
