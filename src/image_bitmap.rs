use std::cell::RefCell;
use std::ffi::CStr;
use std::rc::Rc;

use deno_core::{GarbageCollected, op2, v8};
use euclid::default::{Box2D, Point2D, Size2D, Transform2D};
use euclid::size2;
use strum_macros::FromRepr;

use super::convert::{
    premultiplied_linear_display_p3_to_premultiplied_linear_srgb,
    premultiplied_linear_display_p3_to_srgb, premultiplied_linear_srgb_to_display_p3,
    premultiplied_linear_srgb_to_premultiplied_linear_display_p3,
    premultiplied_linear_srgb_to_srgb, srgb_to_premultiplied_linear_srgb, transform_argb32,
    unpack_argb32_to_rgba8,
};
use super::error::Canvas2DError;
use super::image_data::{AlignedImageDataViewMut, ImageData, ImageDataView};
use super::state::CanvasState;
use super::wrap::Wrap;
use super::{ARGB32_ALPHA_MASK, CanvasColorSpace, raqote_ext, to_raqote_point, to_raqote_size};

pub fn non_zero_u32(x: u32) -> Option<u32> {
    (x != 0).then_some(x)
}

pub fn out_of_bounds(
    width: u32,
    height: u32,
    crop_x: i64,
    crop_y: i64,
    crop_width: u32,
    crop_height: u32,
) -> bool {
    crop_x <= -(crop_width as i64)
        || crop_y <= -(crop_height as i64)
        || crop_x >= width as i64
        || crop_y >= height as i64
}

pub fn same_size(
    width: u32,
    height: u32,
    crop_x: i64,
    crop_y: i64,
    crop_width: u32,
    crop_height: u32,
) -> bool {
    crop_x == 0 && crop_y == 0 && crop_width == width && crop_height == height
}

pub fn aspect_resize(
    sw: u64,
    sh: u64,
    dw: Option<u32>,
    dh: Option<u32>,
) -> Result<Size2D<u32>, Canvas2DError> {
    let (dw, dh) = match (dw, dh) {
        (Some(dw), Some(dh)) => (dw as u128, dh as u128),
        (Some(dw), None) => (dw as u128, (sh as u128 * dw as u128).div_ceil(sw as u128)),
        (None, Some(dh)) => ((sw as u128 * dh as u128).div_ceil(sh as u128), dh as u128),
        (None, None) => (sw as u128, sh as u128),
    };
    let dw = dw
        .try_into()
        .map_err(|_| Canvas2DError::InvalidBitmapWidth { width: dw })?;
    let dh = dh
        .try_into()
        .map_err(|_| Canvas2DError::InvalidBitmapHeight { height: dh })?;
    Ok(size2(dw, dh))
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum ImageOrientation {
    FromImage,
    FlipY,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum ResizeQuality {
    Pixelated,
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug)]
pub struct ImageBitmap {
    pub width: u32,
    pub height: u32,
    pub color_space: CanvasColorSpace,
    pub data: Option<Rc<[u32]>>,
}

impl ImageBitmap {
    pub(crate) fn from_canvas_state(src: &CanvasState) -> Self {
        let image = src.as_raqote_image();
        let color_space = src.color_space();
        Self {
            width: image.width as u32,
            height: image.height as u32,
            color_space,
            data: Some(image.data.into()),
        }
    }

    pub(crate) fn from_canvas_state_crop(
        src: &CanvasState,
        x: i64,
        y: i64,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, Canvas2DError> {
        let image = src.as_raqote_image();
        let color_space = src.color_space();
        let width = width.unwrap_or(image.width as u32);
        let height = height.unwrap_or(image.height as u32);
        if out_of_bounds(image.width as u32, image.height as u32, x, y, width, height) {
            return Ok(Self {
                width,
                height,
                color_space,
                data: None,
            });
        }
        if same_size(image.width as u32, image.height as u32, x, y, width, height) {
            return Ok(Self {
                width,
                height,
                color_space,
                data: Some(image.data.into()),
            });
        }
        let src_origin = to_raqote_point(x, y)?;
        let src = raqote::DrawTarget::from_backing(image.width, image.height, image.data);
        Self::new_with(width, height, color_space, |dst| {
            dst.copy_surface(
                &src,
                Box2D::from_origin_and_size(src_origin, size2(dst.width(), dst.height())),
                Point2D::origin(),
            );
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_image_data_crop_and_resize(
        src: ImageDataView,
        sx: i64,
        sy: i64,
        sw: Option<u32>,
        sh: Option<u32>,
        dw: Option<u32>,
        dh: Option<u32>,
        resize_quality: ResizeQuality,
        image_orientation: ImageOrientation,
    ) -> Result<Self, Canvas2DError> {
        use image::imageops::replace;
        use image::{ImageBuffer, Rgba, RgbaImage};

        let color_space = src.color_space;
        let sw = sw.unwrap_or(src.width);
        let sh = sh.unwrap_or(src.height);
        let (dw, dh) = aspect_resize(sw as u64, sh as u64, dw, dh)?.to_tuple();
        if out_of_bounds(src.width, src.height, sx, sy, sw, sh) {
            return Ok(Self {
                width: dw,
                height: dh,
                color_space,
                data: None,
            });
        }
        let flip_y = match image_orientation {
            ImageOrientation::FromImage => false,
            ImageOrientation::FlipY => true,
        };
        let cropped = if same_size(src.width, src.height, sx, sy, sw, sh) {
            RgbaImage::from_vec(src.width, src.height, src.data.to_owned()).unwrap()
        } else {
            let mut tmp = RgbaImage::new(sw, sh);
            replace(
                &mut tmp,
                &ImageBuffer::<Rgba<u8>, _>::from_raw(src.width, src.height, src.data).unwrap(),
                -sx,
                -sy,
            );
            tmp
        };
        Ok(Self::from_image_data_resize(
            ImageData {
                width: sw,
                height: sh,
                color_space: CanvasColorSpace::Srgb,
                data: cropped.into_vec(),
            },
            dw,
            dh,
            resize_quality,
            flip_y,
        ))
    }

    pub(crate) fn from_image_data_resize(
        src: ImageData,
        width: u32,
        height: u32,
        quality: ResizeQuality,
        flip_y: bool,
    ) -> Self {
        use image::RgbaImage;
        use image::imageops::{FilterType, flip_vertical_in_place, resize};

        let color_space = src.color_space;
        let mut tmp = RgbaImage::from_vec(src.width, src.height, src.data).unwrap();
        for pixel in tmp.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let (r, g, b, a) = srgb_to_premultiplied_linear_srgb((r, g, b, a));
            pixel.0 = [r, g, b, a];
        }
        if src.width != width || src.height != height {
            let filter = match quality {
                ResizeQuality::Pixelated => FilterType::Nearest,
                ResizeQuality::Low => FilterType::Triangle,
                ResizeQuality::Medium => FilterType::CatmullRom,
                ResizeQuality::High => FilterType::Lanczos3,
            };
            tmp = resize(&tmp, width, height, filter);
        }
        if flip_y {
            flip_vertical_in_place(&mut tmp);
        }
        let data = tmp
            .pixels()
            .map(|pixel| {
                let [r, g, b, a] = pixel.0;
                u32::from_be_bytes([a, r, g, b])
            })
            .collect();
        Self {
            width,
            height,
            color_space,
            data: Some(data),
        }
    }

    pub(crate) fn empty(width: u32, height: u32, color_space: CanvasColorSpace) -> Self {
        Self {
            width,
            height,
            color_space,
            data: None,
        }
    }

    fn new_with(
        width: u32,
        height: u32,
        color_space: CanvasColorSpace,
        f: impl FnOnce(&mut raqote::DrawTarget<&mut [u32]>),
    ) -> Result<Self, Canvas2DError> {
        let size = to_raqote_size(width as u64, height as u64)?;
        let mut data = std::iter::repeat_n(0, (width * height) as usize).collect::<Rc<[_]>>();
        let mut dst = raqote::DrawTarget::from_backing(
            size.width,
            size.height,
            Rc::get_mut(&mut data).unwrap(),
        );
        f(&mut dst);
        Ok(Self {
            width,
            height,
            color_space,
            data: Some(data),
        })
    }

    pub(crate) fn into_color_space(self, color_space: CanvasColorSpace) -> Self {
        let mut data = self.data;
        if let Some(data) = &mut data {
            match (self.color_space, color_space) {
                (CanvasColorSpace::Srgb, CanvasColorSpace::Srgb)
                | (CanvasColorSpace::DisplayP3, CanvasColorSpace::DisplayP3) => {}
                (CanvasColorSpace::Srgb, CanvasColorSpace::DisplayP3) => transform_argb32(
                    Rc::make_mut(data),
                    premultiplied_linear_srgb_to_premultiplied_linear_display_p3,
                ),
                (CanvasColorSpace::DisplayP3, CanvasColorSpace::Srgb) => transform_argb32(
                    Rc::make_mut(data),
                    premultiplied_linear_display_p3_to_premultiplied_linear_srgb,
                ),
            }
        };
        Self {
            width: self.width,
            height: self.height,
            color_space,
            data,
        }
    }

    pub(crate) fn into_raqote_image(self) -> Result<Option<raqote_ext::OwnedImage>, Canvas2DError> {
        Ok(match self.data {
            Some(data) => {
                let size = to_raqote_size(self.width as u64, self.height as u64)?;
                Some(raqote_ext::OwnedImage {
                    width: size.width,
                    height: size.height,
                    data,
                })
            }
            None => None,
        })
    }

    pub(crate) fn as_raqote_image(&self) -> Result<Option<raqote::Image<'_>>, Canvas2DError> {
        Ok(match self.data {
            Some(ref data) => {
                let size = to_raqote_size(self.width as u64, self.height as u64)?;
                Some(raqote::Image {
                    width: size.width,
                    height: size.height,
                    data,
                })
            }
            None => None,
        })
    }

    pub(crate) fn crop(
        &self,
        x: i64,
        y: i64,
        width: u32,
        height: u32,
    ) -> Result<Self, Canvas2DError> {
        let color_space = self.color_space;
        if out_of_bounds(self.width, self.height, x, y, width, height) {
            return Ok(Self {
                width,
                height,
                color_space,
                data: None,
            });
        }
        if same_size(self.width, self.height, x, y, width, height) {
            return Ok(self.clone());
        }
        let Some(ref data) = self.data else {
            return Ok(Self {
                width,
                height,
                color_space,
                data: None,
            });
        };
        let src_origin = to_raqote_point(x, y)?;
        let src_size = to_raqote_size(self.width as u64, self.height as u64)?;
        let src = raqote::DrawTarget::from_backing(src_size.width, src_size.height, data.as_ref());
        Self::new_with(width, height, color_space, |dst| {
            dst.copy_surface(
                &src,
                Box2D::from_origin_and_size(src_origin, size2(dst.width(), dst.height())),
                Point2D::origin(),
            );
        })
    }

    pub(crate) fn resize(
        &self,
        width: u32,
        height: u32,
        quality: ResizeQuality,
        flip_y: bool,
    ) -> Result<Self, Canvas2DError> {
        if width == self.width && height == self.height {
            return Ok(self.clone());
        }
        let Some(src) = self.as_raqote_image()? else {
            return Ok(Self {
                width,
                height,
                color_space: self.color_space,
                data: None,
            });
        };
        Self::new_with(width, height, self.color_space, |dst| {
            let mut transform = Transform2D::scale(
                src.width as f32 / dst.width() as f32,
                src.height as f32 / dst.height() as f32,
            );
            if flip_y {
                transform = Transform2D::new(1.0, 0.0, 0.0, -1.0, 0.0, dst.height() as f32)
                    .then(&transform);
            }
            dst.fill_rect(
                0.0,
                0.0,
                dst.width() as f32,
                dst.height() as f32,
                &raqote::Source::Image(
                    src,
                    raqote::ExtendMode::Pad,
                    match quality {
                        ResizeQuality::Pixelated => raqote::FilterMode::Nearest,
                        _ => raqote::FilterMode::Bilinear,
                    },
                    transform,
                    false,
                    false,
                ),
                &raqote::DrawOptions {
                    blend_mode: raqote::BlendMode::Src,
                    ..Default::default()
                },
            );
        })
    }

    pub(crate) fn get_image_data(
        &self,
        mut dst: AlignedImageDataViewMut,
        x: i64,
        y: i64,
    ) -> Result<(), Canvas2DError> {
        let Some(image) = self.as_raqote_image()? else {
            dst.data.fill(0);
            return Ok(());
        };
        let dst_color_space = dst.color_space;
        let mut dst = dst.as_raqote_surface_rgba8()?;
        let src_origin = to_raqote_point(x, y)?;
        let src = raqote::DrawTarget::from_backing(image.width, image.height, image.data);
        dst.composite_surface(
            &src,
            Box2D::from_origin_and_size(src_origin, size2(dst.width(), dst.height())),
            Point2D::origin(),
            |src, dst| {
                dst.copy_from_slice(src);
                match (self.color_space, dst_color_space) {
                    (CanvasColorSpace::Srgb, CanvasColorSpace::Srgb)
                    | (CanvasColorSpace::DisplayP3, CanvasColorSpace::DisplayP3) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_srgb_to_srgb)
                    }
                    (CanvasColorSpace::Srgb, CanvasColorSpace::DisplayP3) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_srgb_to_display_p3)
                    }
                    (CanvasColorSpace::DisplayP3, CanvasColorSpace::Srgb) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_display_p3_to_srgb)
                    }
                }
            },
        );
        Ok(())
    }

    pub(crate) fn remove_alpha(self) -> Self {
        let data = match self.data {
            Some(mut data) => {
                for pixel in Rc::make_mut(&mut data) {
                    *pixel |= ARGB32_ALPHA_MASK;
                }
                data
            }
            None => {
                let size = (self.width as u64 * self.height as u64).try_into().unwrap();
                std::iter::repeat_n(ARGB32_ALPHA_MASK, size).collect()
            }
        };
        Self {
            width: self.width,
            height: self.height,
            color_space: self.color_space,
            data: Some(data),
        }
    }
}

impl Default for ImageBitmap {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            color_space: CanvasColorSpace::Srgb,
            data: None,
        }
    }
}

// SAFETY: this type has no members.
unsafe impl GarbageCollected for Wrap<RefCell<ImageBitmap>> {
    fn get_name(&self) -> &'static CStr {
        c"ImageBitmap"
    }

    fn trace(&self, _: &mut v8::cppgc::Visitor) {}
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_from_canvas_state(
    #[cppgc] canvas_state: &Wrap<RefCell<CanvasState>>,
) -> Wrap<RefCell<ImageBitmap>> {
    let canvas_state = canvas_state.borrow();
    Wrap::new(RefCell::new(ImageBitmap::from_canvas_state(&canvas_state)))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_from_canvas_state_crop(
    #[cppgc] canvas_state: &Wrap<RefCell<CanvasState>>,
    #[number] x: i64,
    #[number] y: i64,
    width: u32,
    height: u32,
) -> Result<Wrap<RefCell<ImageBitmap>>, Canvas2DError> {
    let canvas_state = canvas_state.borrow();
    Ok(Wrap::new(RefCell::new(
        ImageBitmap::from_canvas_state_crop(
            &canvas_state,
            x,
            y,
            non_zero_u32(width),
            non_zero_u32(height),
        )?,
    )))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_from_image_data_crop_and_resize(
    #[buffer] src_data: &[u8],
    src_width: u32,
    src_height: u32,
    src_color_space: i32,
    #[number] sx: i64,
    #[number] sy: i64,
    sw: u32,
    sh: u32,
    dw: u32,
    dh: u32,
    resize_quality: i32,
    image_orientation: i32,
) -> Result<Wrap<RefCell<ImageBitmap>>, Canvas2DError> {
    let src = ImageDataView {
        width: src_width,
        height: src_height,
        color_space: CanvasColorSpace::from_repr(src_color_space).unwrap(),
        data: src_data,
    };
    let resize_quality = ResizeQuality::from_repr(resize_quality).unwrap();
    let image_orientation = ImageOrientation::from_repr(image_orientation).unwrap();
    Ok(Wrap::new(RefCell::new(
        ImageBitmap::from_image_data_crop_and_resize(
            src,
            sx,
            sy,
            non_zero_u32(sw),
            non_zero_u32(sh),
            non_zero_u32(dw),
            non_zero_u32(dh),
            resize_quality,
            image_orientation,
        )?,
    )))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_empty(width: u32, height: u32) -> Wrap<RefCell<ImageBitmap>> {
    Wrap::new(RefCell::new(ImageBitmap::empty(
        width,
        height,
        CanvasColorSpace::Srgb,
    )))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_empty_resize(
    #[number] sw: u64,
    #[number] sh: u64,
    dw: u32,
    dh: u32,
) -> Result<Wrap<RefCell<ImageBitmap>>, Canvas2DError> {
    let size = aspect_resize(sw, sh, non_zero_u32(dw), non_zero_u32(dh))?;
    Ok(Wrap::new(RefCell::new(ImageBitmap::empty(
        size.width,
        size.height,
        CanvasColorSpace::Srgb,
    ))))
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_width(#[cppgc] this: &Wrap<RefCell<ImageBitmap>>) -> u32 {
    let this = this.borrow();
    this.width
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_height(#[cppgc] this: &Wrap<RefCell<ImageBitmap>>) -> u32 {
    let this = this.borrow();
    this.height
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_color_space(#[cppgc] this: &Wrap<RefCell<ImageBitmap>>) -> i32 {
    let this = this.borrow();
    this.color_space as i32
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_clone(
    #[cppgc] this: &Wrap<RefCell<ImageBitmap>>,
) -> Wrap<RefCell<ImageBitmap>> {
    Wrap::new((*this).clone())
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_crop(
    #[cppgc] this: &Wrap<RefCell<ImageBitmap>>,
    #[number] x: i64,
    #[number] y: i64,
    width: u32,
    height: u32,
) -> Result<Wrap<RefCell<ImageBitmap>>, Canvas2DError> {
    let this = this.borrow();
    let width = non_zero_u32(width).unwrap_or(this.width);
    let height = non_zero_u32(height).unwrap_or(this.height);
    Ok(Wrap::new(RefCell::new(this.crop(x, y, width, height)?)))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_resize(
    #[cppgc] this: &Wrap<RefCell<ImageBitmap>>,
    width: u32,
    height: u32,
    quality: i32,
    image_orientation: i32,
) -> Result<Wrap<RefCell<ImageBitmap>>, Canvas2DError> {
    let this = this.take();
    let size = aspect_resize(
        this.width as u64,
        this.height as u64,
        non_zero_u32(width),
        non_zero_u32(height),
    )?;
    let quality = ResizeQuality::from_repr(quality).unwrap();
    let image_orientation = ImageOrientation::from_repr(image_orientation).unwrap();
    Ok(Wrap::new(RefCell::new(this.resize(
        size.width,
        size.height,
        quality,
        matches!(image_orientation, ImageOrientation::FlipY),
    )?)))
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_get_image_data(
    #[cppgc] this: &Wrap<RefCell<ImageBitmap>>,
    #[buffer] dst_data: &mut [u32],
    dst_width: u32,
    dst_height: u32,
    dst_color_space: i32,
    #[number] x: i64,
    #[number] y: i64,
) -> Result<(), Canvas2DError> {
    let this = this.borrow();
    let dst = AlignedImageDataViewMut {
        width: dst_width,
        height: dst_height,
        color_space: CanvasColorSpace::from_repr(dst_color_space).unwrap(),
        data: dst_data,
    };
    this.get_image_data(dst, x, y)
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_image_bitmap_remove_alpha(
    #[cppgc] this: &Wrap<RefCell<ImageBitmap>>,
) -> Wrap<RefCell<ImageBitmap>> {
    let this = this.take();
    Wrap::new(RefCell::new(this.remove_alpha()))
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_close(#[cppgc] this: &Wrap<RefCell<ImageBitmap>>) {
    this.take();
}
