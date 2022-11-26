use std::fmt::Display;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use structopt::StructOpt;
use sysinfo::{self, System, SystemExt};
use sysinfo::{CpuExt, PidExt};
use sysinfo::{CpuRefreshKind, Pid};
use sysinfo::{NetworkExt, ProcessExt};

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub cmd: Option<String>,
    #[structopt(short = "a", long)]
    pub cmd_args: Vec<String>,
}

fn main() {
    let args = Args::from_args();

    println!("Hello, world! {:?} {:?}", args.cmd, args.cmd_args);
    run(args).unwrap();
}

fn run(args: Args) -> Result<()> {
    let proc = args.cmd.map(|cmd| {
        let command = Command::new(cmd).args(args.cmd_args).spawn().unwrap();
        command
    });

    let mut stats = Stats::new(proc.as_ref().map(|p| p.id()));

    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_handle = is_running.clone();

    let handle = std::thread::spawn(move || {
        stats.collect(is_running_handle);
    });

    if proc.is_some() {
        proc.unwrap().wait()?;
        is_running.store(false, Ordering::Relaxed);
    }

    let _ = handle.join();

    Ok(())
}

struct Stats {
    pid: Option<sysinfo::Pid>,
    system: System,
}

impl Stats {
    fn new(pid: Option<u32>) -> Self {
        let pid = pid.map(Pid::from_u32);
        Self {
            pid,
            system: System::new(),
        }
    }

    fn collect(&mut self, is_running: Arc<AtomicBool>) {
        self.system.refresh_networks_list();
        loop {
            let mut info = StatInfo::new(self.pid);
            self.get_cpu(&mut info);
            self.get_net(&mut info);

            println!("{}", info);

            if !is_running.load(Ordering::Relaxed) {
                return;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    fn get_cpu(&mut self, info: &mut StatInfo) {
        if let Some(pid) = self.pid {
            self.system
                .refresh_process_specifics(pid, sysinfo::ProcessRefreshKind::new().with_cpu());

            if let Some(p) = self.system.process(pid) {
                info.cpu = p.cpu_usage();
            }
        } else {
            // Refreshing CPU information.
            self.system
                .refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
            info.cpu = self.system.global_cpu_info().cpu_usage();
        }
    }

    fn get_net(&mut self, info: &mut StatInfo) {
        self.system.refresh_networks();

        for (interface_name, network) in self.system.networks() {
            let net = NetworkStatInfo::new(
                interface_name.to_string(),
                network.received(),
                network.transmitted(),
            );
            info.net.push(net);
        }
    }
}

#[derive(Debug)]
struct StatInfo {
    pid: Option<Pid>,
    cpu: f32,
    net: Vec<NetworkStatInfo>,
}

impl StatInfo {
    fn new(pid: Option<Pid>) -> Self {
        StatInfo {
            pid,
            cpu: 0.0,
            net: Vec::new(),
        }
    }
}

impl Display for StatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pid) = self.pid {
            f.write_fmt(format_args!("{}, ", &pid))?;
        } else {
            f.write_str("global ")?;
        }
        f.write_fmt(format_args!("{}| ", &self.cpu))?;
        for net in &self.net {
            f.write_fmt(format_args!("{} {} {}| ", net.name, net.rx, net.tx))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct NetworkStatInfo {
    name: String,
    rx: u64,
    tx: u64,
}

impl NetworkStatInfo {
    fn new(name: String, rx: u64, tx: u64) -> Self {
        Self { name, rx, tx }
    }
}
