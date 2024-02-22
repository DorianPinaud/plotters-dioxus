#![allow(non_snake_case)]
use dioxus::html::geometry::ElementPoint;
use dioxus::prelude::*;
use plotters_dioxus::{ Plotters, DioxusDrawingArea };
use plotters::{ define_color, doc, coord::{ ReverseCoordTranslate }, prelude::* };

define_color!(BACKGROUND, 11, 20, 31, "background");
define_color!(ITEM, 57, 90, 131, "item");

use rand::SeedableRng;
use rand_distr::{ Distribution, Normal };
use rand_xorshift::XorShiftRng;

fn main() {
    dioxus_desktop::launch(App);
}

fn draw_scatter_plot(
    drawing_area: DioxusDrawingArea,
    click_coord: ElementPoint,
    x_axis_scale: f64
) -> () {
    let number_sample = 50000;
    let normal_dist = Normal::new(0.5, 0.1).unwrap();
    let mut rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
    let iter_rand = normal_dist.sample_iter(&mut rand);
    let data = iter_rand
        .enumerate()
        .take(number_sample)
        .map(|(idx, data)| (
            f64::from(i32::try_from(idx).expect("Expect to be not more than 1000")),
            data,
        ))
        .collect::<Vec<(f64, f64)>>();
    drawing_area.fill(&BACKGROUND).expect("Expect to work");
    let mut scatter_ctx = ChartBuilder::on(&drawing_area)
        .caption("Test graph", ("sans-serif", 14, &WHITE))
        .margin_top(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..x_axis_scale * (number_sample as f64), 0f64..1f64)
        .expect("Expect to work");

    let original_style = ShapeStyle {
        color: WHITE.mix(0.5),
        filled: true,
        stroke_width: 1,
    };

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_label_style(("sans-serif", 11, &WHITE).into_text_style(&drawing_area))
        .x_label_style(("sans-serif", 11, &WHITE).into_text_style(&drawing_area))
        .x_desc("Count")
        .y_desc("Data")
        .axis_style(original_style)
        .axis_desc_style(("sans-serif", 11, &WHITE).into_text_style(&drawing_area))
        .draw()
        .expect("Succeed");
    let t = data
        .iter()
        .map(|e| Circle::new(*e, 1i32, ITEM))
        .collect::<Vec<Circle<(f64, f64), i32>>>();
    scatter_ctx.draw_series(t).expect("Expect to work");
    scatter_ctx
        .as_coord_spec()
        .reverse_translate((click_coord.x as i32, click_coord.y as i32))
        .map(|coord| {
            scatter_ctx
                .draw_series(
                    LineSeries::new(
                        (0..number_sample).map(|x| (x as f64, coord.1)),
                        WHITE
                    )
                )
                .unwrap();
        });
    drawing_area
        .present()
        .expect(
            "Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir"
        );
}

fn App<'a>(cx: Scope<'a>) -> Element {
    let click_coord_state = use_state(cx, ElementPoint::default);
    let x_axis_scale_state = use_state(cx, || 1.0f64);

    render!(Plotters {
        size: (400, 400),
        init: move |d| draw_scatter_plot(d, **click_coord_state, **x_axis_scale_state),
        on_click: |e: Event<MouseData>| click_coord_state.set(e.element_coordinates()),
        on_wheel: |e: Event<WheelData>|
            x_axis_scale_state.set(
                (
                    **x_axis_scale_state +
                    (if e.delta().strip_units().y > 0.0 { -0.1 } else { 0.1 })
                ).max(0.01)
            ),
    })
}
