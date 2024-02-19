#![allow(non_snake_case)]

use dioxus::prelude::*;

use plotters::prelude::*;
use plotters::coord::Shift;

use std::rc::Rc;

use crate::backend::Backend;

#[derive(Props)]
pub struct PlotterProps<'a> {
    pub size: (u32, u32),
    pub on_drawing: EventHandler<'a, DrawingArea<Backend<'a>, Shift>>,
}

pub fn Plotter<'a>(cx: Scope<'a, PlotterProps<'a>>) -> Element<'a> {
    let backend = Rc::new(std::cell::RefCell::new(Backend::new(cx.props.size)));
    let drawing_area = DrawingArea::<Backend, Shift>::from(&backend);
    cx.props.on_drawing.call(drawing_area);
    let dynamic_node = Rc::into_inner(backend)
        .expect("Only one strong reference should exist")
        .into_inner();
    render!(svg {
        height: "{cx.props.size.0}",
        width: "{cx.props.size.1}",
        dynamic_node,
    })
}
