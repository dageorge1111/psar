use libc::{fork, setsid, getpid, pid_t};
use std::process::exit;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{self, Write};
use std::fs::{self, File};
use std::io::BufRead;
use cron::Schedule;
use chrono::Utc;
use std::str::FromStr;
use std::thread;

pub struct Psar<'a> {
    writer: &'a mut dyn Write,
    pid: pid_t,
    schedule: Schedule,
}

impl<'a> Psar<'a> {
    pub fn new(writer: &'a mut dyn Write) -> Self {
        if Self::daemonize() {
            unsafe {
                writeln!(writer, "Starting Daemon with pid: {}", getpid()).unwrap();
                Psar {
                    writer,
                    pid: getpid(),
                    schedule: Self::getSarInterval(),
                }
            }
        } else {
            panic!("Error: Could not generate daemon process for Psar");
        }
    }

    //Maybe race condition with task closing as we are reading?
    //Maybe safely get processes running and match them with directory names rather than just using directory names?
    pub fn start(&mut self) {
        loop {
            let datetime = self.schedule.upcoming(Utc).next().unwrap();
            let now = Utc::now();
            let until = datetime - now;
            thread::sleep(until.to_std().unwrap());
            let paths = fs::read_dir("/proc").unwrap();
            for entry in paths {
                let entry = entry.unwrap();
                let mut path = entry.path();
                let dirname = entry.file_name();
                if let Some(name) = dirname.to_str() {
                    if name.chars().all(|c| c.is_ascii_digit()) {
                        path.push("io");
                        self.scrapeProcIO(path);
                    }
                }
            }
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
    //TODO needs to capture schedule more accurately
    fn getSarInterval() -> Schedule {
        if let Ok(lines) = Self::read_lines("/etc/cron.d/sysstat") {
            for line in lines.map_while(Result::ok) {
                let tokens: Vec<&str> = line.trim().split_whitespace().collect();
                
                if tokens.len() < 2 {continue;}

                let command = &tokens[tokens.len() - 2..];
                let targetCommand: Vec<&str> = ["1", "1"].to_vec();

                if command==targetCommand {
                    let minute_expr = tokens[0..5].join(" ");
                    let cron_expr = format!("0 {}", minute_expr);
                    let schedule = Schedule::from_str("0 */1 * * * *").unwrap();
                    //let schedule = Schedule::from_str(&cron_expr).unwrap();
                    return schedule;
                }
            }
        }
        Schedule::from_str("0 */5 * * * *").unwrap()
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn scrapeProcIO<P>(&mut self, ioFile: P)
    where P: AsRef<Path>, {
        if let Ok(lines) = Self::read_lines(ioFile) {
            for line in lines.map_while(Result::ok) {
                writeln!(self.writer, "{}", line).unwrap();
            }
        }
    }
    //pub fn kill()
}