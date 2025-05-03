use crate::cli::App;
use crate::cli::Commands;
use clap::Parser;
use commands::StatSample;
use std::time::Duration;
use std::time::Instant;

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>;

mod cli;
mod commands;

fn main() {
    let app = App::parse();
    match app.command {
        Commands::Record(record) => {
            // println!("Command: {:?} {:?}", record.cmd, record.args);
            commands::record::run(record).unwrap();
        }
        Commands::Plot(plot) => {
            let mut samples = Vec::new();
            let start = Instant::now();

            samples.push(StatSample::fake(2.0, start - Duration::from_secs(1)));
            samples.push(StatSample::fake(5.0, start - Duration::from_secs(2)));
            samples.push(StatSample::fake(2.0, start - Duration::from_secs(3)));
            samples.push(StatSample::fake(9.0, start - Duration::from_secs(4)));

            commands::plot::plot(samples);
        }
    }
}
