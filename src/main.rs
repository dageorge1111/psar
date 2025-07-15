use libc::{fork, setsid};
use std::process::exit;
use std::io::{self, Write};
use std::fs::File;
use std::env;
use std::path::Path;

fn main() {
    if daemonize() {
        let mut log_file = File::create("log.txt").unwrap();
        scrapeProc(&mut log_file);
    } else {
        println!("Error forking");
    }
}

fn scrapeProc(writer: &mut impl Write) {
    writeln!(writer, "logged something random using dependency injection").unwrap();
}

fn daemonize() -> bool {
    unsafe {
        match fork() {
            -1 => { return false; },
            0 => {
                setsid();
                match fork() {
                    -1 => { return false; },
                    0 => {
                        std::fs::create_dir_all("/tmp/psar").unwrap();
                        let root = Path::new("/tmp/psar");
                        assert!(env::set_current_dir(&root).is_ok());
                        return true;
                    }
                    _ => exit(1),
                }
            }
            _ => exit(1),
        }
    }
}