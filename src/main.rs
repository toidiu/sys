use std::fs;
use std::fs::DirEntry;

use crate::cli::App;
use crate::cli::Commands;
use clap::Parser;
use commands::plot::gen_cdf;

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>;

mod cli;
mod commands;
mod stats;

fn main() {
    let app = App::parse();
    match app.command {
        Commands::Record(record) => {
            commands::record::run(record).unwrap();
        }
        Commands::Plot(plot) => match plot.source.get_one() {
            cli::Source::File(filename) => {
                let stats = commands::plot::read_samples(filename.to_owned());
                commands::plot::plot(stats);
            }
            cli::Source::Dir(dir) => {
                let paths = fs::read_dir(dir).unwrap();

                let mut stats = Vec::new();
                for path in paths {
                    let path = path.unwrap();
                    if !is_hidden(&path) {
                        println!("Name: {}", path.path().display());
                        let stat = commands::plot::read_samples(path.path());
                        stats.push(stat);

                        // commands::plot::plot(stats);
                    }
                }

                gen_cdf(&stats);
            }
        },
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
