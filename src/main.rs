mod psar;
use psar::Psar;

use libc::{fork, setsid};
use std::process::exit;
use std::io::{self, Write};
use std::fs::File;
use std::env;
use std::path::Path;

fn main() {
    let mut log_file = File::create("log.txt").unwrap();
    let mut psar = Psar::new(&mut log_file);
    psar.start();
}