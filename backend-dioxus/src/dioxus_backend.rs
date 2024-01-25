use plotters_backend::{
    BackendColor,
    BackendCoord,
    BackendStyle,
    BackendTextStyle,
    DrawingBackend,
    DrawingErrorKind,
};
use std::io::Error;

pub struct DioxusBackend;

impl DioxusBackend {
    pub fn new() -> Self {
        Self{}
    }
}

impl<'a> DrawingBackend for DioxusBackend {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        (0, 0)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        _: BackendCoord,
        _: BackendColor
    ) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        _: BackendCoord,
        _: BackendCoord,
        _: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        _: BackendCoord,
        _: BackendCoord,
        _: &S,
        _: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
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
