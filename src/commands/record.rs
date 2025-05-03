use crate::cli::Record;
use crate::commands::NetworkStatInfo;
use crate::commands::StatSample;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use sysinfo::{self, System, SystemExt};
use sysinfo::{CpuExt, PidExt};
use sysinfo::{CpuRefreshKind, Pid};
use sysinfo::{NetworkExt, ProcessExt};

pub fn run(record: Record) -> crate::Result<()> {
    let arg_clone = record.clone();
    let proc = record.cmd.map(|cmd| {
        let command = process::Command::new(cmd)
            .args(record.args)
            .spawn()
            .unwrap();
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
}

impl StatContext {
    fn new(pid: Option<u32>) -> Self {
        let pid = pid.map(Pid::from_u32);
        Self {
            pid,
            system: System::new(),
            start_ts: Instant::now(),
        }
    }

    fn collect(&mut self, is_running: Arc<AtomicBool>, record_cmd: Record) {
        println!("elapsed_ms, pid, cpu, [[net, rx, tx], ...]");

        self.system.refresh_networks_list();
        loop {
            let mut info = StatSample::new(self.pid, self.start_ts);
            self.get_cpu(&mut info);
            self.get_net(&mut info, &record_cmd);

            // Print the stats info each round
            println!("{}", &info);

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

    fn get_net(&mut self, info: &mut StatSample, record_cmd: &Record) {
        self.system.refresh_networks();

        for (interface_name, network) in self.system.networks() {
            if let Some(net_name) = &record_cmd.network_interface {
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
