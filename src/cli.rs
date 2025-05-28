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

    /// filter out lines that start with this pattern
    #[arg(short, long)]
    pub filter: Vec<String>,

    // Plot the Cumulative distribution function
    #[arg(long, default_value = "true")]
    pub cdf: bool,

    // Plot each metric
    #[arg(long, default_value = "false")]
    pub graph: bool,
}

#[derive(Debug, clap::Args, Clone)]
#[group(required = true, multiple = false)]
pub struct SourceGroup {
    /// File path
    #[arg(long)]
    pub file: Option<String>,

    /// Dir path
    #[clap(long)]
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
impl<'a> Source<'a> {
    pub fn display(&self) -> String {
        match self {
            Source::File(f) => f.to_string(),
            Source::Dir(d) => d.to_string(),
        }
    }
}
