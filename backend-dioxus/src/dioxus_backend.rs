use plotters_backend::{
    BackendColor,
    BackendCoord,
    BackendStyle,
    BackendTextStyle,
    DrawingBackend,
    DrawingErrorKind,
};
use std::io::Error;

use dioxus::prelude::*;

pub type Stack<'a, 'b> = &'a mut Vec<Box<dyn Fn() -> LazyNodes<'b, 'b>>>;

pub struct DioxusBackend<'a, 'b: 'a> {
    pub stack: Stack<'a, 'b>,
}

impl<'a, 'b> DioxusBackend<'a, 'b> {
    pub fn new(stack: Stack<'a, 'b>) -> Self {
        Self { stack: stack }
    }
}

fn make_svg_color(color: BackendColor) -> String {
    let (r, g, b) = color.rgb;
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

fn make_svg_opacity(color: BackendColor) -> String {
    return format!("{}", color.alpha);
}

impl<'a, 'b> DrawingBackend for DioxusBackend<'a, 'b> {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        (200, 200)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor
    ) -> Result<(), DrawingErrorKind<Error>> {
        if color.alpha == 0.0 {
            return Ok(());
        }
        self.stack.push(
            Box::new(move || {
                let hex_color = make_svg_color(color);
                rsx! {
                    rect {
                        x: "{point.0}",
                        y: "{point.1}",
                        width: 1,
                        height: 1,
                        fill: "{hex_color}",
                        stroke: "none",
                    }
                }
            })
        );
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let hex_color = make_svg_color(style.color());
        let alpha = make_svg_opacity(style.color());
        let stroke_width = format!("{}", style.stroke_width());
        self.stack.push(
            Box::new(move || {
                let hex_color = hex_color.clone();
                let alpha = alpha.clone();
                let stroke_width = stroke_width.clone();
                rsx! {
                    line {
                        x1: "{from.0}",
                        y1: "{from.1}",
                        x2: "{to.0}",
                        y2: "{to.1}",
                        stroke: "{hex_color}",
                        opacity: "{alpha}",
                        stroke_width: "{stroke_width}",
                    }
                }
            })
        );
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let (fill, stroke) = if !fill {
            ("none".to_string(), make_svg_color(style.color()))
        } else {
            (make_svg_color(style.color()), "none".to_string())
        };

        let alpha = make_svg_opacity(style.color());
        self.stack.push(
            Box::new(move || {
                let alpha = alpha.clone();
                let fill = fill.clone();
                let stroke = stroke.clone();
                rsx! {
                    rect {
                        x: "{upper_left.0}",
                        y: "{upper_left.1}",
                        width: "{bottom_right.0 - upper_left.0}",
                        height: "{bottom_right.1 - upper_left.1}",
                        opacity: "{alpha}",
                        fill: "{fill}",
                        stroke: "{stroke}",
                    }
                }
            })
        );
        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        _: I,
        _: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        _: I,
        _: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        _: BackendCoord,
        _: u32,
        _: &S,
        _: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        _: &str,
        _: &S,
        _: BackendCoord
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "image"))]
    fn blit_bitmap<'b>(
        &mut self,
        _: BackendCoord,
        _: (u32, u32),
        _: &'b [u8]
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }
}
