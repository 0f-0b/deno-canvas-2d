use deno_core::anyhow;

use super::{to_raqote_size, CanvasColorSpace};

#[derive(Clone, Copy, Debug)]
pub(super) struct BorrowedImageData<'a> {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: &'a [u32],
}

impl<'a> BorrowedImageData<'a> {
    pub fn as_raqote_surface_rgba8(&self) -> anyhow::Result<raqote::DrawTarget<&[u32]>> {
        let size = to_raqote_size(self.width as u64, self.height as u64)?;
        Ok(raqote::DrawTarget::from_backing(
            size.width,
            size.height,
            self.data,
        ))
    }
}

#[derive(Debug)]
pub(super) struct BorrowedMutImageData<'a> {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: &'a mut [u32],
}

impl<'a> BorrowedMutImageData<'a> {
    pub fn as_raqote_surface_rgba8(&mut self) -> anyhow::Result<raqote::DrawTarget<&mut [u32]>> {
        let size = to_raqote_size(self.width as u64, self.height as u64)?;
        Ok(raqote::DrawTarget::from_backing(
            size.width,
            size.height,
            self.data,
        ))
    }
}
