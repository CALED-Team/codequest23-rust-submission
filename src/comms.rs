use serde_json::{from_str, Value};
use std::io;

/// String received at the end of the game.
pub const END_SIGNAL: &'static str = "END";

/// String received at the end of the world init stage.
pub const END_INIT_SIGNAL: &'static str = "END_INIT";

/// Parses a JSON literal and prints it to standard output.
#[macro_export]
macro_rules! post_message {
    ($json:tt) => {
        println!("{}", serde_json::json!($json));
    };
}

/// Reads a line of input from standard input as JSON, and returns the parsed message.
pub fn read_message() -> Value {
    let mut msg = String::new();

    io::stdin()
        .read_line(&mut msg)
        .expect("Error reading input");

    from_str(&msg).expect("Failed to read input as JSON")
}
