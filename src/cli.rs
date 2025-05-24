use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser, Clone)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Record stats
    Record(Record),

    /// Plot stats
    Plot(Plot),
}

#[derive(Clone, Debug, Parser)]
pub struct Record {
    /// Command to run or collect entire system stats
    pub cmd: Option<String>,

    /// Args for the command.
    #[arg(short, long, requires = "cmd")]
    pub args: Vec<String>,

    /// Specify the network interface name to only emit stats for that interface.
    #[arg(short, long)]
    pub network_interface: Option<String>,
}

// #[derive(Clone, Debug, Parser)]
#[derive(Debug, clap::Args, Clone)]
#[clap(name = "plot")]
pub struct Plot {
    /// source
    #[clap(flatten)]
    pub source: SourceGroup,
}

#[derive(Debug, clap::Args, Clone)]
#[group(required = true, multiple = false)]
pub struct SourceGroup {
    /// File path
    #[arg(short, long)]
    pub file: Option<String>,

    /// Dir path
    #[clap(short, long)]
    pub dir: Option<String>,
}

impl SourceGroup {
    pub fn get_one(&self) -> Source {
        match (&self.dir, &self.file) {
            (Some(dir), None) => Source::Dir(&dir),
            (None, Some(file)) => Source::File(&file),
            (Some(_), Some(_)) | (None, None) => unreachable!(),
        }
    }
}

pub enum Source<'a> {
    File(&'a str),
    Dir(&'a str),
}
