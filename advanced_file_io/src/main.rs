extern crate libc;
extern crate nix;

use nix::sys::uio::*;
use nix::fcntl::*;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;

use std::str;

fn main() {
    let buffer =
        b"The term buccaneer comes from the word boucan.\n\
        A boucan is a wooden frame used for cooking meat.\n\
        Buccaneer is the West Indies name for a pirate.\n";
    let fd = open("buccaneer.txt", OFlag::O_WRONLY, Mode::S_IWUSR).unwrap();

    let mut iov = [
        IoVec::from_slice(&buffer[0..45]),
        IoVec::from_slice(&buffer[45..96]),
        IoVec::from_slice(&buffer[96..145]),
    ];

    let nr = writev(fd , &iov).unwrap();
    println!("wrote {} bytes", nr);

    let fd = open("buccaneer.txt", OFlag::O_RDONLY, Mode::S_IWUSR).unwrap();

    let mut buffers = vec![
        vec![0;47],
        vec![0;50],
        vec![0;48],
    ];

    let mut iov:Vec<_> = buffers.iter_mut().map(
        |buf| IoVec::from_mut_slice(&mut buf[..])).collect();
    let nr = readv(fd, &mut iov).unwrap();

    for i in 0..3 {
        print!("{}: {}",i, str::from_utf8(&buffers[i]).unwrap());
    }
    println!("read {} bytes", nr);
}
