use crate::commands::NetworkStatInfo;
use crate::commands::StatSample;
use plotly::{
    color::Rgb, common::Marker, layout::GridPattern, layout::Layout, layout::LayoutGrid, Plot,
    Scatter,
};

// pub fn plot_stats<F, T>(sys_group: SysGroup, f: F, title_units: &str, show: bool)
// where
//     F: FnOnce(SysStats) -> Vec<T> + std::marker::Copy,
//     T: serde::ser::Serialize + std::clone::Clone + 'static,
// {
//     let mut plot = Plot::new();

//     for stat in sys_group.sys.into_iter() {
//         let name = stat.title.clone();
//         let trace = Scatter::new(stat.sec.clone(), f(stat))
//             .name(name)
//             .x_axis("x")
//             .y_axis("y");
//         plot.add_trace(trace);
//     }

//     let layout = Layout::new()
//         .title(
//             format!("{} ({})", sys_group.title.as_str(), title_units)
//                 .as_str()
//                 .into(),
//         )
//         .show_legend(true)
//         .height(1000)
//         .grid(
//             LayoutGrid::new()
//                 .rows(1)
//                 .columns(1)
//                 .pattern(GridPattern::Independent),
//         );
//     plot.set_layout(layout);
//     if show {
//         plot.show();
//     }
//     // println!("{}", plot.to_inline_html(Some("simple_subplot")));
// }

pub fn plot(samples: Vec<StatSample>) {
    let mut plot = Plot::new();

    // for stat in self.samples.iter() {
    // TODO
    // let name = stat.title.clone();
    let name = "title";

    let ts_ms = samples.iter().map(|sample| sample.ts()).collect();
    let cpu = samples.iter().map(|sample| sample.cpu).collect();
    let trace = Scatter::new(ts_ms, cpu).name(name).x_axis("x").y_axis("y");
    plot.add_trace(trace);
    // }

    let layout = Layout::new()
        // .title(format!("{} ({})", "title", "units").as_str().into())
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
