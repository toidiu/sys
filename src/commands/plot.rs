use crate::stats::cdf;
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Scatter};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_samples(filename: impl AsRef<Path>) -> Stats {
    let file = File::open(&filename).unwrap();

    let mut lines = BufReader::new(file).lines();
    // remove the units header
    let legend = lines.next().expect("expected a non-empty file").unwrap();
    let legend = legend.split(',').map(|s| String::from(s.trim())).collect();

    let filename: &Path = filename.as_ref();
    let mut stats = Stats {
        filename: filename.to_string_lossy().into_owned(),
        legend,
        values: Vec::new(),
    };

    let legend_len = stats.legend.len();
    for line in lines {
        let line = line.unwrap();

        // NEW
        let mut values_instance = Vec::new();

        let mut split_values = line.split(',').map(|s| String::from(s.trim()));
        for _ in 0..legend_len {
            let val: usize = split_values.next().unwrap().parse().unwrap();
            values_instance.push(val);
        }

        stats.values.push(values_instance);
    }

    stats
}

#[derive(Debug)]
pub struct Stats {
    pub filename: String,
    pub legend: Vec<String>,
    pub values: Vec<Vec<usize>>,
}

pub fn plot(stats: Stats) {
    let mut plot = plotly::Plot::new();

    let title = format!("{}", stats.filename);

    // x axis
    let x_name = stats.legend[0].clone();
    let x_values: Vec<usize> = stats.values.iter().map(|v| v[0]).collect();

    // y axis
    let mut metrics = Vec::new();
    let legend_len = stats.legend.len();
    for i in 1..legend_len {
        let metric: Vec<usize> = stats.values.iter().map(|v| v[i]).collect();
        metrics.push(metric);
    }

    let legend_plus_metrics = stats.legend.iter().skip(1).zip(metrics);

    for (legend, metrics) in legend_plus_metrics {
        // let cpu = samples.iter().map(|sample| sample.cpu).collect();
        let trace = Scatter::new(x_values.clone(), metrics)
            .name(legend)
            .x_axis("x")
            .y_axis("y");
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .title(title.as_str())
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

pub fn gen_cdf(stats: &[Stats]) {
    let legend = stats[0].legend.clone();
    let legend_len = legend.len();

    let mut plot = plotly::Plot::new();

    for idx in 1..legend_len {
        let title = format!("{}", legend[idx]);

        let mut x: Vec<f64> = Vec::new();
        for stat in stats.iter() {
            let temp: Vec<f64> = stat.values.iter().map(|v| v[idx] as f64).collect();

            x.extend_from_slice(&temp);
        }

        let x = cdf(&x);
        let (x, y): (Vec<_>, Vec<_>) = x.into_iter().map(|(a, b)| (a, b)).unzip();

        // Graph
        let trace = Scatter::new(x, y).name(&title).x_axis("x").y_axis("y");
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .title("cdf")
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
