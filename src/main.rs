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
            source @ cli::Source::File(filename) => {
                let stats = commands::plot::read_samples(&plot, filename.to_owned(), &source);

                if plot.graph {
                    commands::plot::graph(&stats);
                }

                if plot.cdf {
                    gen_cdf(&[stats]);
                }
            }
            source @ cli::Source::Dir(dir) => {
                let mut stats = Vec::new();

                let paths = fs::read_dir(dir).unwrap();
                for path in paths {
                    let path = path.unwrap();
                    if !is_hidden(&path) {
                        println!("... processing file: {}", path.path().display());
                        let stat = commands::plot::read_samples(&plot, path.path(), &source);
                        stats.push(stat);
                    }
                }

                if plot.graph {
                    for stat in stats.iter() {
                        commands::plot::graph(stat);
                    }
                }

                if plot.cdf {
                    gen_cdf(&stats);
                }
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
