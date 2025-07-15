use libc::{fork, getpid, c_int, setsid};
use std::time::Duration;
use std::thread;
use std::process::exit;

fn main() {
    if double_fork() {
        println!("I should be the grandchild!");
    } else {
        println!("Error forking")
    }
}



fn double_fork() -> bool {
    unsafe {
        match fork() {
            -1 => { return false; },
            0 => {
                setsid();
                match fork() {
                    -1 => { return false; },
                    0 => {
                        return true;
                    }
                    _ => exit(1),
                }
            }
            _ => exit(1),
        }
    }
}