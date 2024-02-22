#![allow(non_snake_case)]

use dioxus::prelude::*;

use plotters_bitmap::BitMapBackend;

use plotters::prelude::*;
use plotters::coord::Shift;

use base64::prelude::*;

use image::ImageEncoder;
use image::codecs::png::PngEncoder;

use std::io::Cursor;

pub type DioxusDrawingArea<'a> = DrawingArea<BitMapBackend<'a>, Shift>;

#[derive(Props)]
pub struct PlottersProps<'a, F: Fn(DioxusDrawingArea)> {
    pub size: (u32, u32),
    pub init: F,
    pub on_click: EventHandler<'a, Event<MouseData>>,
    pub on_wheel: EventHandler<'a, Event<WheelData>>,
}

impl<'a, F: Fn(DioxusDrawingArea)> PartialEq for PlottersProps<'a, F> {
    fn eq(&self, other: &PlottersProps<F>) -> bool {
        self.size == other.size
    }
}

pub fn Plotters<'a, F: Fn(DioxusDrawingArea)>(cx: Scope<'a, PlottersProps<'a, F>>) -> Element<'a> {
    let buffer_size = ((cx.props.size.1 * cx.props.size.0) as usize) * 3usize;
    let mut buffer = vec![0u8; buffer_size];
    let drawing_area = BitMapBackend::with_buffer(
        buffer.as_mut_slice(),
        cx.props.size
    ).into_drawing_area();
    (cx.props.init)(drawing_area);

    let mut data = vec![0; 0];
    let cursor = Cursor::new(&mut data);
    let encoder = PngEncoder::new(cursor);
    let color = image::ColorType::Rgb8;

    encoder.write_image(buffer.as_slice(), cx.props.size.0, cx.props.size.1, color).expect("Should work");

    let buffer_base64 = BASE64_STANDARD.encode(data);

    render!(img {
        onclick: |e| cx.props.on_click.call(e),
        onwheel: |e| cx.props.on_wheel.call(e),
        src: "data:image/png;base64,{buffer_base64}",
    })
}
