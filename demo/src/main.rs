#![allow(non_snake_case)]
use dioxus::prelude::*;
use plotters_dioxus::{Backend, Plotters};
use plotters::{coord::Shift, prelude::*};

fn main() {
    dioxus_desktop::launch(App);
}

fn draw_histogram(drawing_area : DrawingArea<Backend, Shift>) -> () {
    let _ = drawing_area.fill(&WHITE);
    let mut chart = ChartBuilder::on(&drawing_area)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)
        .expect("Expect a chart to be build");

    let _ = chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw();

    let data = [0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3];

    let _ = chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x: &u32| (*x, 1)))
    );

    // To avoid the IO failure being ignored silently, we manually call the present function
    drawing_area
        .present()
        .expect(
            "Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir"
        );
}

fn App<'a>(cx: Scope<'a>) -> Element {
    render!(Plotters {
        size: (400, 400),
        on_drawing: draw_histogram,
    })
}
