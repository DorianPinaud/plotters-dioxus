#![allow(non_snake_case)]
use dioxus::prelude::*;
use backend_dioxus::DioxusBackend;
use plotters::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App<'a>(cx: Scope<'a>) -> Element {
    let mut stack = Vec::<Box<dyn Fn() -> LazyNodes<'a, 'a>>>::new();

    {
        let root_area = DioxusBackend::new(&mut stack).into_drawing_area();
        let _ = root_area.fill(&RED);
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

    cx.render(
        rsx! {
            svg {
                height: "200",
                width: "200",
                stack.iter().map(|e| (*e)()),
            }
        }
    )
}
