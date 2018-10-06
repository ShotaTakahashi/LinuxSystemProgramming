extern crate libc;
extern crate num_cpus;

use libc::{getpriority, setpriority};

fn main() {
    unsafe {
        let ret = getpriority(libc::PRIO_PROCESS, 0);
        println!("nice value is {}", ret);

        let _ret = setpriority(libc::PRIO_PGRP, 0, 10);
        println!("{}", getpriority(libc::PRIO_PROCESS, 0));
    }

    let cpus = num_cpus::get();
    println!("We are on a multicore system with {} CPUs", cpus);

    let physical_cpus = num_cpus::get_physical();
    println!("{} CPUs are using now", physical_cpus);
}
