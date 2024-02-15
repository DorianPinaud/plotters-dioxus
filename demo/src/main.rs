#![allow(non_snake_case)]
use dioxus::prelude::*;
use backend_dioxus::DioxusBackend;
use plotters::prelude::*;
use plotters::coord::Shift;
use std::rc::Rc;


fn main() {
    dioxus_desktop::launch(App);
}

fn build_histogram<'a>(size: (u32, u32)) -> DioxusBackend<'a> {
    let backend = Rc::new(std::cell::RefCell::new(DioxusBackend::new(size)));
    {
        let root_area = DrawingArea::<DioxusBackend, Shift>::from(&backend);
    let _ = root_area.fill(&WHITE);
    let mut chart = ChartBuilder::on(&root_area)
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
    root_area
        .present()
        .expect(
            "Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir"
        );
    }
    Rc::into_inner(backend).expect("Should be working").into_inner()
}

fn App<'a>(cx: Scope<'a>) -> Element {
    let size = (400, 400);

    let backend = build_histogram(size);

    cx.render(
        rsx! {
            svg {
                height: "{size.0}",
                width: "{size.1}",
                backend,
            }
        }
    )
}
