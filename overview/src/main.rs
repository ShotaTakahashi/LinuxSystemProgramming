use std::io::{stderr, Write};
use std::process::exit;

fn main() {
    eprintln!("Error Example eprintln");

    let stderr = stderr();

    // Using explicit synchronization
    let mut handle = stderr.lock();

    handle.write(b"Error Example stderr").unwrap();

    exit(1);
}
