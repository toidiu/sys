use crate::cli::Plot as PlotCmd;
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Scatter};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_samples(plot_cmd: PlotCmd) -> Vec<StatSample> {
    let file = File::open(&plot_cmd.file).unwrap();

    let mut samples = Vec::new();

    let mut lines = BufReader::new(file).lines();
    // remove the units header
    lines.next();

    for line in lines {
        let sample = StatSample::parse(line.unwrap());
        samples.push(sample);
    }

    samples
}

pub fn plot(samples: Vec<StatSample>) {
    let mut plot = plotly::Plot::new();

    let ts_ms: Vec<u64> = samples.iter().map(|sample| sample.ts).collect();

    // CPU
    let cpu = samples.iter().map(|sample| sample.cpu).collect();
    let trace = Scatter::new(ts_ms.clone(), cpu)
        .name("cpu")
        .x_axis("x")
        .y_axis("y");
    plot.add_trace(trace);

    // RX
    let rx = samples.iter().map(|sample| sample.rx).collect();
    let trace = Scatter::new(ts_ms.clone(), rx)
        .name("rx")
        .x_axis("x")
        .y_axis("y");
    plot.add_trace(trace);

    // TX
    let tx = samples.iter().map(|sample| sample.tx).collect();
    let trace = Scatter::new(ts_ms, tx).name("tx").x_axis("x").y_axis("y");
    plot.add_trace(trace);

    let layout = Layout::new()
        .title(format!("{} ({})", "title", "units").as_str())
        .show_legend(true)
        .height(1000)
        .grid(
            LayoutGrid::new()
                .rows(1)
                .columns(1)
                .pattern(GridPattern::Independent),
        );
    plot.set_layout(layout);
    plot.show();
}

#[derive(Debug)]
pub struct StatSample {
    ts: u64,
    cpu: f32,
    network_interface: String,
    rx: u64,
    tx: u64,
}

impl StatSample {
    fn parse(s: String) -> Self {
        let mut s = s.split(',').map(|s| String::from(s.trim()));

        let ts: u64 = s.next().unwrap().parse().unwrap();
        let cpu = s.next().unwrap().parse().unwrap();
        let network_interface = s.next().unwrap().parse().unwrap();
        let rx = s.next().unwrap().parse().unwrap();
        let tx = s.next().unwrap().parse().unwrap();

        StatSample {
            ts: ts / 1000,
            cpu,
            network_interface,
            rx,
            tx,
        }
    }
}
