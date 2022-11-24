use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use structopt::StructOpt;
use sysinfo::Pid;
use sysinfo::PidExt;
use sysinfo::{self, CpuExt, CpuRefreshKind, System, SystemExt};
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
            self.get_cpu();
            self.get_net();
            if !is_running.load(Ordering::Relaxed) {
                return;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    fn get_cpu(&mut self) {
        // Refreshing CPU information.
        // self.system
        // .refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());

        self.system
            .refresh_process_specifics(self.pid, sysinfo::ProcessRefreshKind::new().with_cpu());

        if let Some(p) = self.system.process(self.pid) {
            let usage = p.cpu_usage();
            println!("---------pid: {}, cpu: {}% ", self.pid, usage);
        } else {
            println!("----------cant find");
        }
    }

    fn get_net(&mut self) {
        self.system.refresh_networks();

        for (interface_name, network) in self.system.networks() {
            println!(
                "{} rx: {} tx: {}",
                interface_name,
                network.received(),
                network.transmitted()
            );
            // println!("in: {} B", network.received());
        }
    }
}
