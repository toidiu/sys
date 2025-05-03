use crate::cli::Plot as PlotCmd;
use crate::commands::StatSample;
use crate::Instant;
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Scatter};
use std::time::Duration;

// TODO parse file and plot data
pub fn parse_data(plot_cmd: PlotCmd) -> Vec<StatSample> {
    let mut samples = Vec::new();
    let start = Instant::now();

    samples.push(StatSample::fake(2.0, start - Duration::from_secs(1)));
    samples.push(StatSample::fake(5.0, start - Duration::from_secs(2)));
    samples.push(StatSample::fake(2.0, start - Duration::from_secs(3)));
    samples.push(StatSample::fake(9.0, start - Duration::from_secs(4)));
    samples
}

pub fn plot(samples: Vec<StatSample>) {
    let mut plot = plotly::Plot::new();

    // TODO
    let name = "data1";

    let ts_ms = samples.iter().map(|sample| sample.ts()).collect();
    let cpu = samples.iter().map(|sample| sample.cpu).collect();
    let trace = Scatter::new(ts_ms, cpu).name(name).x_axis("x").y_axis("y");
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
    // if show {
    plot.show();
    // }
    // println!("{}", plot.to_inline_html(Some("simple_subplot")));
}
