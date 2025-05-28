use crate::cli::Plot;
use crate::cli::Source;
use crate::stats::cdf;
use plotly::{layout::GridPattern, layout::Layout, layout::LayoutGrid, Scatter};
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_samples(arg: &Plot, filename: impl AsRef<Path>, source: &Source) -> Stats {
    let file = File::open(&filename).unwrap();

    let lines = BufReader::new(file).lines();

    let mut lines = lines.filter(|l| l.is_ok() && !skip_line(arg, l.as_ref().unwrap()));

    // remove the units header
    let legend = lines.next().expect("expected a non-empty file").unwrap();
    let legend = legend.split(',').map(|s| String::from(s.trim())).collect();

    let mut stats = Stats {
        source: source.display(),
        legend,
        values: Vec::new(),
    };

    let legend_len = stats.legend.len();
    for (i, line) in lines.enumerate() {
        let line = line.unwrap();

        let mut split_values = line.split(',').map(|s| String::from(s.trim()));

        // Collect the values from this line
        let mut line_values = Vec::new();
        for _ in 0..legend_len {
            let val = split_values.next().and_then(|value| value.parse().ok());
            if val.is_none() {
                eprintln!("------- {:?} line:{} {:?}", val, i, line);
            }

            let val = val.unwrap();
            line_values.push(val);
        }

        stats.values.push(line_values);
    }

    stats
}

fn skip_line(arg: &Plot, line: &str) -> bool {
    for rex in arg.filter.iter() {
        let re = Regex::new(&rex).unwrap();
        if line.is_empty() {
            return true;
        }

        if re.is_match(line) {
            // Debug which lines to skip
            // println!("--------SKIP {:?} --- {}", re.captures(line), line);
            return true;
        }
    }

    false
}

#[derive(Debug)]
pub struct Stats {
    pub source: String,
    pub legend: Vec<String>,
    pub values: Vec<Vec<usize>>,
}

pub fn graph(stats: &Stats) {
    let mut plot = plotly::Plot::new();

    let title = format!("{}", stats.source);

    // x axis
    let x_values: Vec<usize> = stats.values.iter().map(|v| v[0]).collect();

    // y axis
    let mut y_values = Vec::new();
    let legend_len = stats.legend.len();
    for i in 1..legend_len {
        let y_value: Vec<usize> = stats.values.iter().map(|v| v[i]).collect();
        y_values.push(y_value);
    }

    let legend_and_y_values = stats.legend.iter().skip(1).zip(y_values);

    for (metric_name, metric) in legend_and_y_values {
        let trace = Scatter::new(x_values.clone(), metric)
            .name(metric_name)
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
    let title = format!("{}", stats[0].source);

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
        .title(format!("{} Cumulative distribution function", title))
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
