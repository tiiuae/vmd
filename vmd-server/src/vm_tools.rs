use regex::Regex;
use std::process::Command;
use std::fmt::Display;

#[derive(Debug)]
pub struct Process {
    pub user: String,
    pub pid: u32,
    pub cpu: f32,
    pub mem: f32,
    pub vsz: u64,
    pub rss: u64,
    pub tty: String,
    pub stat: String,
    pub start: String,
    pub time: String,
    pub command: String,
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            self.user,
            self.pid,
            self.cpu,
            self.mem,
            self.vsz,
            self.rss,
            self.tty,
            self.stat,
            self.start,
            self.time,
            self.command
        )
    }
}

impl From<&str> for Process {
    fn from(line: &str) -> Self {
        let re = Regex::new(r#"\s+"#).unwrap();
        let fields: Vec<&str> = re.split(line).collect();

        let process = Process {
            user: fields[0].to_string(),
            pid: fields[1].parse().unwrap_or(0),
            cpu: fields[2].parse().unwrap_or(0.0),
            mem: fields[3].parse().unwrap_or(0.0),
            vsz: fields[4].parse().unwrap_or(0),
            rss: fields[5].parse().unwrap_or(0),
            tty: fields[6].to_string(),
            stat: fields[7].to_string(),
            start: fields[8].to_string(),
            time: fields[9].to_string(),
            command: fields[10].to_string(),
        };

        process
    }
}

pub struct ProcessList {
    pub processes: Vec<Process>,
}

impl ProcessList {
    pub fn new() -> Self {
        let output = Command::new("ps")
            .args(&["aux"])
            .output()
            .expect("Failed to execute command");

        let ps_output = String::from_utf8(output.stdout).expect("Failed to parse command output");

        let processes = ps_output.lines()
            .skip(1)
            .map(|line| Process::from(line))
            .collect();

        ProcessList { processes }
    }
}

impl Display for ProcessList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!(
            "{:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
            "USER", "PID", "%CPU", "%MEM", "VSZ", "RSS", "TTY", "STAT", "START", "TIME", "COMMAND"
        );
        for process in &self.processes {
            writeln!(f, "{}", process)?;
        }

        Ok(())
    }
}

