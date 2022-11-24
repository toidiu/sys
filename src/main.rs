use std::fmt::Display;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use structopt::StructOpt;
use sysinfo::Pid;
use sysinfo::PidExt;
use sysinfo::{self, System, SystemExt};
use sysinfo::{NetworkExt, ProcessExt};

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub cmd: String,
}

fn main() {
    let args = Args::from_args();

    println!("Hello, world! {}", args.cmd);
    run(args).unwrap();
}

fn run(args: Args) -> Result<()> {
    let mut command = Command::new(&args.cmd);
    let mut proc = command.spawn()?;

    let mut stats = Stats::new(proc.id());

    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_handle = is_running.clone();

    let handle = std::thread::spawn(move || {
        stats.collect(is_running_handle);
    });

    proc.wait()?;
    is_running.store(false, Ordering::Relaxed);

    let _ = handle.join();

    Ok(())
}

struct Stats {
    pid: sysinfo::Pid,
    system: System,
}

impl Stats {
    fn new(pid: u32) -> Self {
        Self {
            pid: Pid::from_u32(pid),
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
        // Refreshing CPU information.
        // self.system
        // .refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());

        self.system
            .refresh_process_specifics(self.pid, sysinfo::ProcessRefreshKind::new().with_cpu());

        if let Some(p) = self.system.process(self.pid) {
            info.cpu = p.cpu_usage();
        } else {
            println!("----------cant find");
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
    pid: sysinfo::Pid,
    cpu: f32,
    net: Vec<NetworkStatInfo>,
}

impl StatInfo {
    fn new(pid: sysinfo::Pid) -> Self {
        StatInfo {
            pid,
            cpu: 0.0,
            net: Vec::new(),
        }
    }
}

impl Display for StatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, ", &self.pid))?;
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
