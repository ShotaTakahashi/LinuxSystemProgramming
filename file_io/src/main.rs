#![feature(mpsc_select)]
extern crate libc;
extern crate nix;

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::sync::{Arc, Mutex};
use std::thread;
use nix::sys::select;
use nix::sys::time::{TimeValLike, TimeVal};
use nix::poll::{EventFlags, PollFd, poll};

const TIMEOUT: i64 = 5;

fn main() {
    // create, open, read, write
    let path = "test.txt";

    {
        let mut fd = File::create(&path).unwrap();
        fd.write(b"hello world").unwrap();
    }

    let mut fd = File::open(&path).unwrap();
    let mut buffer = String::new();
    fd.read_to_string(&mut buffer).unwrap();

    assert_eq!(buffer, "hello world");


    // seek
    fd.seek(SeekFrom::Start(6)).unwrap();
    let mut seek_buffer = String::new();
    fd.read_to_string(&mut seek_buffer).unwrap();

    assert_eq!(seek_buffer, "world");


    // synchronized
    let fd = File::open(&path).unwrap();
    let fd = Arc::new(Mutex::new(fd));

    for _ in 0..10 {
        let fd = fd.clone();
        let mut buffer = String::new();
        thread::spawn(move || {
            let mut fd = fd.lock().unwrap();
            fd.read_to_string(&mut buffer).unwrap();
            println!("{:?}", buffer);
        });
    }


    // select
    let mut readfds = select::FdSet::new();
    readfds.insert(libc::STDIN_FILENO);

    let mut tv: TimeVal = TimeValLike::seconds(TIMEOUT);

    let ret = select::select(Some(readfds.highest().unwrap() + 1),
                   &mut readfds,
                   None,
                   None,
                   &mut tv).unwrap();

    match ret {
        -1 => eprintln!("select"),
        0 => println!("{} seconds elapsed.", TIMEOUT),
        _ => {
            if readfds.contains(libc::STDIN_FILENO) {
                /*
                 * Not Implemented
                 */
            }
        }
    }


    // poll
    let stdin = PollFd::new(libc::STDIN_FILENO, EventFlags::POLLIN);
    let stdout = PollFd::new(libc::STDOUT_FILENO, EventFlags::POLLOUT);

    let mut fds = [stdin, stdout];

    let ret = poll(&mut fds, (TIMEOUT * 1000) as i32).unwrap();

    if ret == -1 { println!("poll"); };
    if ret == 0 { println!("TIMEOUT"); };

    if fds[0].revents().unwrap().intersects(EventFlags::POLLIN) { println!("stdin is readable");};
    if fds[1].revents().unwrap().intersects(EventFlags::POLLOUT) { println!("stdout is writable");}
}
