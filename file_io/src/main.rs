use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let path = "test.txt";

    {
        let mut fd = File::create(&path).unwrap();
        fd.write(b"hello world").unwrap();
    }

    let mut fd = File::open(&path).unwrap();
    let mut buffer = String::new();
    fd.read_to_string(&mut buffer).unwrap();

    assert_eq!(buffer, "hello world");

    let fd = Arc::new(Mutex::new(fd));

    for _ in 0..10 {
        let fd = fd.clone();
        let mut buffer = String::new();
        spawn(move || {
            let mut fd = fd.lock().unwrap();
            fd.read_to_string(&mut buffer).unwrap();
            println!("{}", buffer);
        });
    }
}
