extern crate nix;
extern crate libc;

use std::env::args;
use nix::sys::stat::stat;

use std::fs::*;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("usage: {}", args[0]);
        //panic!();
    }

    let path = &args[1][..];

    let ret = stat(path);

    match ret {
        Ok(sb) => println!("{} is {} bytes", path, sb.st_size),
        Err(err) => eprintln!("stat: {}", err),
    }

    let mut perms = metadata("test.txt").unwrap().permissions();
    perms.set_readonly(true);
    set_permissions("test.txt", perms).unwrap();
}