use std::fmt::Display;
use std::time::Instant;
use sysinfo::Pid;

pub mod plot;
pub mod record;

#[derive(Debug)]
pub struct StatSample {
    ts_ms: Instant,
    pid: Option<Pid>,
    cpu: f32,
    net: Vec<NetworkStatInfo>,
}

impl StatSample {
    pub fn fake(cpu: f32, ts: Instant) -> Self {
        StatSample {
            ts_ms: ts,
            pid: None,
            cpu,
            net: Vec::new(),
        }
    }

    fn new(pid: Option<Pid>, ts: Instant) -> Self {
        StatSample {
            ts_ms: ts,
            pid,
            cpu: 0.0,
            net: Vec::new(),
        }
    }

    // Time elapsed in millis
    fn ts(&self) -> u128 {
        self.ts_ms.elapsed().as_millis()
    }
}

impl Display for StatSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TS
        f.write_fmt(format_args!("{}, ", &self.ts()))?;

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
