use libc::{fork, setsid, getpid, pid_t};
use std::process::exit;
use std::path::{Path, PathBuf};
use std::env;
use std::io::{self, Write, BufRead};
use std::fs::{self, File, DirEntry};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use cron::Schedule;
use chrono::Utc;
use std::str::FromStr;
use std::thread;
use std::collections::HashMap;

//TODO needs to run with sudo priviliges
//TODO need proper error handling not just panic everything
//TODO we need to figure out how to determine flushed vs buffered writes to quantify write_bytes contribution to iowait
//For now we will just ignore write operations but we need to figure out how to measure write operations
pub struct Psar<'a> {
    writer: &'a mut dyn Write,
    pid: pid_t,
    schedule: Schedule,
    process_io_dict: HashMap<u32, u64>,
    total_io: u64
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
                    process_io_dict: HashMap::new(),
                    total_io: 0
                }
            }
        } else {
            panic!("Error: Could not generate daemon process for Psar");
        }
    }

    //Maybe race condition with task closing as we are reading?
    //Maybe safely get processes running and match them with directory 
    // names rather than just using directory names?
    pub fn start(&mut self) {
        loop {
            let datetime = self.schedule.upcoming(Utc).next().unwrap();
            let now = Utc::now();
            let until = datetime - now;
            thread::sleep(until.to_std().unwrap());
            let paths = fs::read_dir("/proc").unwrap();
            
            let (tx, rx) = mpsc::channel();

            for entry in paths {
                let entry = entry.unwrap();
                let tx1 = tx.clone();

                thread::spawn(move || {
                    Psar::processPath(entry, tx1);
                });
            }

            for received in rx {
                let (pid, io_value) = received;
                println!("Reciever got PID: {}, IO: {}", pid, io_value);
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
    fn getSarInterval() -> Schedule {
        if let Ok(lines) = Self::read_lines("/etc/cron.d/sysstat") {
            for line in lines.map_while(Result::ok) {
                let tokens: Vec<&str> = line
                    .trim().split_whitespace().collect();
                
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

    fn processPath(entry: DirEntry, tx: Sender<(u32, u64)>){
        let mut path = entry.path();
        let dirname = entry.file_name();

        if let Some(proc_pid) = dirname.to_str() {
            if proc_pid.chars().all(|c| c.is_ascii_digit()) {
                path.push("io");
                let io_value = Psar::scrapeProcIO(path, proc_pid.parse::<u32>().unwrap());
                tx.send((proc_pid.parse::<u32>().unwrap(), io_value)).unwrap();
                println!("Parsed PID: {}, Total IO: {}", proc_pid, io_value);
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn scrapeProcIO<P>(ioFile: P, proc_pid: u32) -> u64
    where P: AsRef<Path>, {
        let mut total_io = 0;

        if let Ok(lines) = Self::read_lines(ioFile) {
            for line in lines.map_while(Result::ok) {
                let tokens: Vec<&str> = line
                    .trim().split_whitespace().collect();

                if tokens.len() < 2 {continue;}

                match tokens[0] {
                    "read_bytes:" => {
                        total_io += tokens[1].parse::<u64>().unwrap();
                    }
                    "write_bytes:" => {
                        total_io += tokens[1].parse::<u64>().unwrap();
                    }
                    &_ => {
                        continue;
                    }
                }
            }
        }

        total_io
    }
    //pub fn kill()
}