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

#[derive(Clone, Debug, Parser)]
pub struct Plot {
    /// File path
    file: String,
    // /// Dir path
    // dir: String,
}
