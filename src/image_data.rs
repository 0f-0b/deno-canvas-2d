use super::error::Canvas2DError;
use super::{to_raqote_size, CanvasColorSpace};

#[derive(Clone, Debug)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct ImageDataView<'a> {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: &'a [u8],
}

#[derive(Clone, Copy, Debug)]
pub struct AlignedImageDataView<'a> {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: &'a [u32],
}

impl AlignedImageDataView<'_> {
    pub fn as_raqote_surface_rgba8(&self) -> Result<raqote::DrawTarget<&[u32]>, Canvas2DError> {
        let size = to_raqote_size(self.width as u64, self.height as u64)?;
        Ok(raqote::DrawTarget::from_backing(
            size.width,
            size.height,
            self.data,
        ))
    }
}

#[derive(Debug)]
pub struct AlignedImageDataViewMut<'a> {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: &'a mut [u32],
}

impl AlignedImageDataViewMut<'_> {
    pub fn as_raqote_surface_rgba8(
        &mut self,
    ) -> Result<raqote::DrawTarget<&mut [u32]>, Canvas2DError> {
        let size = to_raqote_size(self.width as u64, self.height as u64)?;
        Ok(raqote::DrawTarget::from_backing(
            size.width,
            size.height,
            self.data,
        ))
    }
}
