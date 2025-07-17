use libc::{fork, setsid, getpid, pid_t};
use std::process::exit;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{self, Write};

pub struct Psar {
  writer: &mut impl Write,
  pid: pid_t,
  interval: u64
}

impl Psar {
    pub fn new(writer: &mut impl Write) -> Self {
        if Self::daemonize() {
            Self::getSarInterval();
            Psar {
                writer,
                pid: getpid(),
                interval: 1
            }
        } else {
            panic!("Error: Could not generate daemon process for Psar");
        }
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
    //TODO Somehow we need to implement a default value here
    fn getSarInterval() {
        if let Ok(lines) = Self::read_lines("/etc/cron.d/sysstat") {
            for line in lines.map_while(Result::ok) {
                println!("{}", line);
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}