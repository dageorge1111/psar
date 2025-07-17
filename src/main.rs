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
    let psar = Psar::new(log_file);
}
//fn matchConfig()
fn scrapeProc(writer: &mut impl Write) {
    writeln!(writer, "logged something random using dependency injection").unwrap();
}