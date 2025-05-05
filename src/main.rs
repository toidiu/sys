use crate::cli::App;
use crate::cli::Commands;
use clap::Parser;

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>;

mod cli;
mod commands;

fn main() {
    let app = App::parse();
    match app.command {
        Commands::Record(record) => {
            commands::record::run(record).unwrap();
        }
        Commands::Plot(plot) => {
            let samples = commands::plot::read_samples(&plot);
            commands::plot::plot(&plot, samples);
        }
    }
}
