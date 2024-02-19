use plotters_backend::{
    BackendColor,
    BackendCoord,
    BackendStyle,
    BackendTextStyle,
    DrawingBackend,
    DrawingErrorKind,
    FontStyle,
    FontTransform,
};
use plotters_backend::text_anchor::{ HPos, VPos };

use std::io::Error;

use dioxus::prelude::*;
use dioxus::core::DynamicNode;

use std::fmt::Write as _;

pub struct Backend<'a> {
    pub svg_children: Vec<LazyNodes<'a, 'a>>,
    size: (u32, u32),
}

impl<'a> Backend<'a> {
    pub fn new(size: (u32, u32)) -> Self {
        Self { svg_children: Vec::<LazyNodes<'a, 'a>>::new(), size: size }
    }
}

fn make_svg_color(color: BackendColor) -> String {
    let (r, g, b) = color.rgb;
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

fn make_svg_opacity(color: BackendColor) -> String {
    return format!("{}", color.alpha);
}

impl<'a> IntoDynNode<'a> for Backend<'a> {
    fn into_vnode(self, cx: &'a ScopeState) -> DynamicNode<'a> {
        rsx!(self.svg_children.into_iter()).into_vnode(cx)
    }
}

impl<'a> DrawingBackend for Backend<'a> {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        self.size
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
        let hex_color = make_svg_color(color);
        self.svg_children.push(
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
        self.svg_children.push(
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
        self.svg_children.push(
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
        );
        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let opacity = make_svg_opacity(style.color());
        let stroke = make_svg_color(style.color());
        let stroke_width = format!("{}", style.stroke_width());
        let points = path.into_iter().fold(String::new(), |mut s, (x, y)| {
            write!(s, "{},{} ", x, y).ok();
            s
        });
        self.svg_children.push(
            rsx! {
                path {
                    fill: "none",
                    opacity: "{opacity}",
                    stroke: "{stroke}",
                    stroke_width: "{stroke_width}",
                    points: "{points}",
                }
            }
        );
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let opacity = make_svg_opacity(style.color());
        let fill = make_svg_color(style.color());
        let points = path.into_iter().fold(String::new(), |mut s, (x, y)| {
            write!(s, "{},{} ", x, y).ok();
            s
        });
        self.svg_children.push(
            rsx! {
                polygon {
                    fill: "{fill}",
                    opacity: "{opacity}",
                    points: "{points}",
                }
            }
        );
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let (stroke, fill) = if !fill {
            (make_svg_color(style.color()), "none".to_string())
        } else {
            ("none".to_string(), make_svg_color(style.color()))
        };
        let stroke_width = format!("{}", style.stroke_width());
        let opacity = make_svg_opacity(style.color());
        self.svg_children.push(
            rsx! {
                circle {
                    cx: "{center.0}",
                    cy: "{center.1}",
                    r: "{radius}",
                    opacity: "{opacity}",
                    fill: "{fill}",
                    stroke: "{stroke}",
                    stroke_width: "{stroke_width}",
                }
            }
        );
        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }

        let (x0, y0) = pos;
        let text_anchor = (
            match style.anchor().h_pos {
                HPos::Left => "start",
                HPos::Right => "end",
                HPos::Center => "middle",
            }
        ).to_string();

        let dy = (
            match style.anchor().v_pos {
                VPos::Top => "0.76em",
                VPos::Center => "0.5ex",
                VPos::Bottom => "-0.5ex",
            }
        ).to_string();

        let (font_weight, font_style) = match style.style() {
            FontStyle::Bold => (Some("bold".to_string()), None),
            other_style => (None, Some(other_style.as_str().to_string())),
        };

        let transf = (
            match style.transform() {
                FontTransform::Rotate90 => { Some(format!("rotate(90, {}, {})", x0, y0)) }
                FontTransform::Rotate180 => { Some(format!("rotate(180, {}, {})", x0, y0)) }
                FontTransform::Rotate270 => { Some(format!("rotate(270, {}, {})", x0, y0)) }
                _ => None,
            }
        ).unwrap_or("".to_string());

        let font_family = style.family().as_str().to_string();
        let font_size = style.size() / 1.24;
        let opacity = make_svg_opacity(color);
        let fill = make_svg_color(color);
        let text = text.to_string();
        let font_weight = font_weight.unwrap_or("".to_string());
        let font_style = font_style.unwrap_or("".to_string());
        self.svg_children.push(
            rsx! {
                    text {
                        x: "{x0}",
                        y: "{y0}",
                        dy: "{dy}",
                        text_anchor: "{text_anchor}",
                        font_family: "{font_family}",
                        font_size: "{font_size}",
                        opacity: "{opacity}",
                        fill: "{fill}",
                        transform: "{transf}",
                        font_style: "{font_style}",
                        font_weight: "{font_weight}",
                        "{text}"
                    }
                }
        );

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
