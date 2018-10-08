extern crate nix;

use nix::unistd::{sysconf, SysconfVar};
use std::time::*;
use std::thread::sleep;

fn main() {
    let hz = sysconf(SysconfVar::_POSIX_MONOTONIC_CLOCK).unwrap();
    println!("{:?}", hz);

    let ret = SystemTime::now();
    println!("{:?}", ret);

    sleep(Duration::new(0, 200));
    println!("finish sleeping");

}
