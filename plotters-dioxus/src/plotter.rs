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
    pub on_click: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_dblclick: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_mousemove: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_mouseout: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_mouseup: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_mousedown: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_mouseover: Option<EventHandler<'a, Event<MouseData>>>,
    pub on_wheel: Option<EventHandler<'a, Event<WheelData>>>,
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

    encoder
        .write_image(buffer.as_slice(), cx.props.size.0, cx.props.size.1, color)
        .expect("The Png encoder is expected to write the image");

    let buffer_base64 = BASE64_STANDARD.encode(data);

    render!(img {
        onclick: |e| (
            if cx.props.on_click.is_some() {
                cx.props.on_click.as_ref().unwrap().call(e)
            }
        ),
        ondblclick: |e| (
            if cx.props.on_dblclick.is_some() {
                cx.props.on_dblclick.as_ref().unwrap().call(e)
            }
        ),
        onmousemove: |e| {
            if cx.props.on_mousemove.is_some() {
                cx.props.on_mousemove.as_ref().unwrap().call(e)
            }
        },
        onmousedown: |e| (
            if cx.props.on_mousedown.is_some() {
                cx.props.on_mousedown.as_ref().unwrap().call(e)
            }
        ),
        onmouseup: |e| (
            if cx.props.on_mouseup.is_some() {
                cx.props.on_mouseup.as_ref().unwrap().call(e)
            }
        ),
        onmouseout: |e| {
            if cx.props.on_mouseout.is_some() {
                cx.props.on_mouseout.as_ref().unwrap().call(e)
            }
        },
        onmouseover: |e| {
            if cx.props.on_mouseover.is_some() {
                cx.props.on_mouseover.as_ref().unwrap().call(e)
            }
        },
        onwheel: |e| (
            if cx.props.on_wheel.is_some() {
                cx.props.on_wheel.as_ref().unwrap().call(e)
            }
        ),
        src: "data:image/png;base64,{buffer_base64}",
    })
}
