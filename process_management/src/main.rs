extern crate nix;

use nix::unistd::{getpid, getppid, execv, execve, fork};
use std::ffi::CString;

use nix::sys::wait::{wait, waitpid};
use nix::sys::wait::{WaitStatus, WaitPidFlag};
use nix::sys::signal::Signal;
use nix::unistd::Pid;

use nix::unistd::{getuid, getgid, getsid, setsid};

use nix::unistd::{chdir, close, dup, daemon};
use std::os::raw::c_int;
use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;

fn main() {
    // pid
    println!("My pid={}", getpid());
    println!("Parent's pid={}", getppid());


    let path = &CString::new("/bin/ls").unwrap();

    // exec
    //exec_sample(path);

    // fork
    let pid = fork().unwrap();

    if pid.is_parent() {
        println!("I am the parent of pid={:?}", pid);
    } else if pid.is_child() {
        println!("I am the baby!");
    }

    if pid.is_child() {
        let args = [CString::new("ls").unwrap()];

        let _ret = execv(path, &args);
    }

    let pid = wait().unwrap();

    println!("pid={:?}", pid);

    let ex = WaitStatus::Exited(pid.pid().unwrap(), 0);
    println!("{:?}", ex);
    let sig = WaitStatus::Signaled(pid.pid().unwrap(), Signal::SIGKILL, true);
    println!("{:?}", sig);
    let stop = WaitStatus::Stopped(pid.pid().unwrap(), Signal::SIGSTOP);
    println!("{:?}", stop);
    let continued = WaitStatus::Continued(pid.pid().unwrap());
    println!("{:?}", continued);

    let pid = waitpid(Pid::from_raw(1742), Some(WaitPidFlag::WNOHANG));
    match pid {
        Err(_) => eprintln!("waitpid"),
        Ok(pid) => {
            println!("pid={:?}", pid);
        }
    }

    //my_system(CString::new("help").unwrap());

    println!("user id={:?}, group id={:?}", getuid(), getgid());

    let pid = fork();

    match setsid() {
        Ok(sid) => println!("pid={:?}, getsid={:?}", sid, getsid(Some(sid))),
        Err(_) => eprintln!("setsid")
    }

    match chdir("/") {
        Ok(_) => println!("Change"),
        Err(_) => eprintln!("chdir")
    }
    for i in 0..1024 {
        close(i as c_int).unwrap();
    }

    open("/dev/null", OFlag::O_RDWR, Mode::S_IRWXU).unwrap();
    dup(0).unwrap();
    dup(0).unwrap();

    daemon(false, false).unwrap();

}

fn exec_sample(path: &CString) {
    let _ret = execve(path,
                      &[CString::new("ls").unwrap(), CString::new("-l").unwrap()],
                      &[CString::new("").unwrap()]);
}

fn my_system(cmd: CString) {
    let _pid = fork().unwrap();

    let argv = [
        CString::new("sh").unwrap(),
        CString::new("-c").unwrap(),
        cmd,
        CString::new("").unwrap(),
    ];
    let sh = CString::new("/bin/sh").unwrap();
    let _ret = execv(&sh, &argv).unwrap();
}