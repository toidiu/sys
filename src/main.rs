use std::fmt::Display;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use structopt::StructOpt;
use sysinfo::{self, System, SystemExt};
use sysinfo::{CpuExt, PidExt};
use sysinfo::{CpuRefreshKind, Pid};
use sysinfo::{NetworkExt, ProcessExt};

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>;

mod plot;

#[derive(Debug, StructOpt, Clone)]
pub struct Args {
    /// Command to run.
    pub cmd: Option<String>,

    /// Args for the command.
    #[structopt(short = "a", long)]
    pub cmd_args: Vec<String>,

    /// Specify the network interface name to only emit stats for that interface.
    #[structopt(short = "n", long)]
    pub network_interface: Option<String>,

    /// Print stats
    #[structopt(short = "p", long)]
    pub print: bool,
}

fn main() {
    let args = Args::from_args();

    println!("Command: {:?} {:?}", args.cmd, args.cmd_args);
    run(args).unwrap();
}

fn run(args: Args) -> Result<()> {
    let arg_clone = args.clone();
    let proc = args.cmd.map(|cmd| {
        let command = Command::new(cmd).args(args.cmd_args).spawn().unwrap();
        command
    });

    let mut stats = StatContext::new(proc.as_ref().map(|p| p.id()));

    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_handle = is_running.clone();

    let handle = std::thread::spawn(move || {
        stats.collect(is_running_handle, arg_clone);
    });

    if proc.is_some() {
        proc.unwrap().wait()?;
        is_running.store(false, Ordering::Relaxed);
    }

    let _ = handle.join();

    Ok(())
}

struct StatContext {
    pid: Option<Pid>,
    system: System,
    start_ts: Instant,
    samples: Vec<StatSample>,
}

impl StatContext {
    fn new(pid: Option<u32>) -> Self {
        let pid = pid.map(Pid::from_u32);
        Self {
            pid,
            system: System::new(),
            start_ts: Instant::now(),
            samples: Vec::new(),
        }
    }

    fn collect(&mut self, is_running: Arc<AtomicBool>, args: Args) {
        if args.print {
            println!("elapsed_ms, pid, cpu, [[net, rx, tx], ...]");
        }

        self.system.refresh_networks_list();
        loop {
            let mut info = StatSample::new(self.pid, self.start_ts);
            self.get_cpu(&mut info);
            self.get_net(&mut info, &args);

            // Print the stats info each round
            if args.print {
                println!("{}", &info);
            }
            self.samples.push(info);

            if !is_running.load(Ordering::Relaxed) {
                return;
            }

            let sleep_duration = std::time::Duration::from_millis(1000);
            std::thread::sleep(sleep_duration);
        }
    }

    fn get_cpu(&mut self, info: &mut StatSample) {
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

    fn get_net(&mut self, info: &mut StatSample, args: &Args) {
        self.system.refresh_networks();

        for (interface_name, network) in self.system.networks() {
            if let Some(net_name) = &args.network_interface {
                if interface_name != net_name {
                    continue;
                }
            }

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
struct StatSample {
    start_ts: Instant,
    pid: Option<Pid>,
    cpu: f32,
    net: Vec<NetworkStatInfo>,
}

impl StatSample {
    fn new(pid: Option<Pid>, ts: Instant) -> Self {
        StatSample {
            start_ts: ts,
            pid,
            cpu: 0.0,
            net: Vec::new(),
        }
    }
}

impl Display for StatSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TS
        f.write_fmt(format_args!("{}, ", &self.start_ts.elapsed().as_millis()))?;

        // PID
        if let Some(pid) = self.pid {
            f.write_fmt(format_args!("{}, ", &pid))?;
        } else {
            f.write_str("global ")?;
        }

        // CPU
        f.write_fmt(format_args!("{}, ", &self.cpu))?;

        // NET
        for net in &self.net {
            f.write_fmt(format_args!("{}, {}, {}, ", net.name, net.rx, net.tx))?;
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
