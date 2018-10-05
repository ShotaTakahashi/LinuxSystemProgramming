extern crate libc;
extern crate nix;

use nix::sys::uio::*;
use nix::fcntl::*;
use nix::sys::stat::Mode;

//use nix::sys::epoll::{EpollCreateFlags, EpollEvent, EpollFlags, EpollOp};
//use nix::sys::epoll::{epoll_create1, epoll_ctl};

use libc::c_void;
use nix::sys::stat::{fstat, stat};
use nix::sys::mman::{MapFlags, ProtFlags};
use nix::sys::mman::{mmap, munmap};
use nix::unistd::close;
use std::os::unix::io::RawFd;
use std::env::args;

use std::str;

fn main() {
    // scatter-gather I/O
    // write
    let buffer =
        b"The term buccaneer comes from the word boucan.\n\
        A boucan is a wooden frame used for cooking meat.\n\
        Buccaneer is the West Indies name for a pirate.\n";
    let fd = open("buccaneer.txt",
                  OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
                  Mode::S_IWUSR)
            .unwrap();

    let iov = [
        IoVec::from_slice(&buffer[0..45]),
        IoVec::from_slice(&buffer[45..96]),
        IoVec::from_slice(&buffer[96..145]),
    ];

    let nr = writev(fd , &iov).unwrap();
    println!("wrote {} bytes", nr);


    // read
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

    /*
    let fd = open("buccaneer.txt",
                  OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
                  Mode::S_IWUSR)
        .unwrap();

    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap();
    let mut event = EpollEvent {
        event: EpollFlags::EPOLLIN | EpollFlags::EPOLLOUT,
    };

    let ret = epoll_ctl(epfd, EpollOp::EpollCtlAdd, None, &mut event);

    match ret {
        Ok(_) => eprintln!("epoll_ctr"),
        Err(_) => println!("not come")
    }*/

    // mmap
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("usage: {}", args[0]);
    }

    let path = &args[1][..];

    let fd = open(path, OFlag::O_RDONLY, Mode::S_IWUSR).unwrap();

    let stat = stat(path).unwrap();

    let inode = get_inode(fd);
    println!("{}", inode);

    print_blocks(fd);

    unsafe {
        let p = mmap(0 as *mut c_void,
                     stat.st_size as usize,
                     ProtFlags::PROT_READ,
                     MapFlags::MAP_SHARED,
                     fd,
                     0).unwrap();

        close(fd).unwrap();

        println!("{:?}", p);

        let error = munmap(p, stat.st_size as usize);
        match error {
            Ok(_) => println!("success"),
            Err(_) => eprintln!("munmap")
        }
    }
}

fn get_inode(fd: RawFd) -> u64 {
    let ret = fstat(fd);

    match ret {
        Ok(result) => return result.st_ino,
        Err(_) => {
            eprintln!("fstat");
            return 0
        }
    }
}

fn get_block(fd: RawFd, logical_block: u64) -> i64 {
    // not implement. Is there a library having `ioctl` ?
    return 32
}

fn get_nr_blocks(fd: RawFd) -> i64 {
    let ret = fstat(fd).unwrap();
    return ret.st_blocks
}

fn print_blocks(fd: RawFd) {
    let nr_blocks = get_nr_blocks(fd);

    match nr_blocks {
        -1 => eprintln!("get_nr_blocks failed!"),
        0 => println!("no allocated bocks"),
        1 => println!("1 block"),
        _ => println!("{} blocks", nr_blocks)
    }

    for i in 0..nr_blocks {
        let phys_block = get_block(fd, i as u64);

        if phys_block < 0 {
            eprintln!("get_block failed!")
        } else if phys_block == 0 {
            continue;
        }
        println!("({}, {})", i, phys_block);
    }
}