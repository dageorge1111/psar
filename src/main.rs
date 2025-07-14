use libc::{fork, getpid, c_int};
use std::time::Duration;
use std::thread;
use std::process::exit;

fn main() {
    double_fork()
}

fn double_fork() {
    let pid_return;
    let pid;
    unsafe {
        pid_return = fork();
        pid = getpid()
    }

    if pid_return>0 {
        println!("The child process pid: {pid_return} My pid: {pid}", pid_return=pid_return, pid=pid);
        println!("Sleeping");
        thread::sleep(Duration::from_secs(10));
        println!("Awake and killing self");
        std::process::exit(1);
    } else{
        println!("I am the child");
        thread::sleep(Duration::from_secs(20));
    }
    //println!("The child process pid: {pid_return} My pid: {pid}", pid_return=pid_return, pid=pid);
}