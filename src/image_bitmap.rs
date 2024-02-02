use std::ffi::c_void;
use std::rc::Rc;

use deno_core::error::range_error;
use deno_core::{anyhow, op2, v8, OpState};
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
use super::gc::{borrow_v8, from_v8, into_v8};
use super::image_data::{AlignedImageDataViewMut, ImageData, ImageDataView};
use super::state::CanvasState;
use super::{raqote_ext, to_raqote_point, to_raqote_size, CanvasColorSpace, ARGB32_ALPHA_MASK};

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
) -> anyhow::Result<Size2D<u32>> {
    let (dw, dh) = match (dw, dh) {
        (Some(dw), Some(dh)) => (dw as u128, dh as u128),
        (Some(dw), None) => (dw as u128, (sh as u128 * dw as u128).div_ceil(sw as u128)),
        (None, Some(dh)) => ((sw as u128 * dh as u128).div_ceil(sh as u128), dh as u128),
        (None, None) => (sw as u128, sh as u128),
    };
    let dw = dw
        .try_into()
        .map_err(|_| range_error(format!("Invalid bitmap width: {dw}")))?;
    let dh = dh
        .try_into()
        .map_err(|_| range_error(format!("Invalid bitmap height: {dh}")))?;
    Ok(size2(dw, dh))
}

// TODO use `Rc::make_mut` after rust-lang/rust#116113
fn rc_slice_make_mut<T: Clone>(rc: &mut Rc<[T]>) -> &mut [T] {
    if Rc::strong_count(rc) != 1 || Rc::weak_count(rc) != 0 {
        *rc = rc.as_ref().into();
    }
    Rc::get_mut(rc).unwrap()
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
    pub fn from_canvas_state(src: &CanvasState) -> Self {
        let image = src.as_raqote_image();
        let color_space = src.color_space();
        Self {
            width: image.width as u32,
            height: image.height as u32,
            color_space,
            data: Some(image.data.into()),
        }
    }

    pub fn from_canvas_state_crop(
        src: &CanvasState,
        x: i64,
        y: i64,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        let image = src.as_raqote_image();
        let color_space = src.color_space();
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
    pub fn from_image_data_crop_and_resize(
        src: ImageDataView,
        sx: i64,
        sy: i64,
        sw: Option<u32>,
        sh: Option<u32>,
        dw: Option<u32>,
        dh: Option<u32>,
        resize_quality: ResizeQuality,
        image_orientation: ImageOrientation,
    ) -> anyhow::Result<Self> {
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

    pub fn from_image_data_resize(
        src: ImageData,
        width: u32,
        height: u32,
        quality: ResizeQuality,
        flip_y: bool,
    ) -> Self {
        use image::imageops::{flip_vertical_in_place, resize, FilterType};
        use image::RgbaImage;

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

    pub fn empty(width: u32, height: u32, color_space: CanvasColorSpace) -> Self {
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
    ) -> anyhow::Result<Self> {
        let size = to_raqote_size(width as u64, height as u64)?;
        let mut data = std::iter::repeat(0)
            .take((width * height) as usize)
            .collect::<Rc<[_]>>();
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

    pub fn into_color_space(self, color_space: CanvasColorSpace) -> Self {
        let mut data = self.data;
        if let Some(data) = &mut data {
            match (self.color_space, color_space) {
                (CanvasColorSpace::Srgb, CanvasColorSpace::Srgb)
                | (CanvasColorSpace::DisplayP3, CanvasColorSpace::DisplayP3) => {}
                (CanvasColorSpace::Srgb, CanvasColorSpace::DisplayP3) => {
                    transform_argb32(
                        rc_slice_make_mut(data),
                        premultiplied_linear_srgb_to_premultiplied_linear_display_p3,
                    );
                }
                (CanvasColorSpace::DisplayP3, CanvasColorSpace::Srgb) => {
                    transform_argb32(
                        rc_slice_make_mut(data),
                        premultiplied_linear_display_p3_to_premultiplied_linear_srgb,
                    );
                }
            }
        };
        Self {
            width: self.width,
            height: self.height,
            color_space,
            data,
        }
    }

    pub fn into_raqote_image(self) -> anyhow::Result<Option<raqote_ext::OwnedImage>> {
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

    pub fn as_raqote_image(&self) -> anyhow::Result<Option<raqote::Image>> {
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

    pub fn crop(&self, x: i64, y: i64, width: u32, height: u32) -> anyhow::Result<Self> {
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

    pub fn resize(
        &self,
        width: u32,
        height: u32,
        quality: ResizeQuality,
        flip_y: bool,
    ) -> anyhow::Result<Self> {
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

    pub fn get_image_data(
        &self,
        mut dst: AlignedImageDataViewMut,
        x: i64,
        y: i64,
    ) -> anyhow::Result<()> {
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
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_srgb_to_srgb);
                    }
                    (CanvasColorSpace::Srgb, CanvasColorSpace::DisplayP3) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_srgb_to_display_p3);
                    }
                    (CanvasColorSpace::DisplayP3, CanvasColorSpace::Srgb) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_display_p3_to_srgb);
                    }
                }
            },
        );
        Ok(())
    }

    pub fn remove_alpha(self) -> Self {
        let data = match self.data {
            Some(mut data) => {
                for pixel in rc_slice_make_mut(&mut data) {
                    *pixel |= ARGB32_ALPHA_MASK;
                }
                data
            }
            None => {
                let size = (self.width as u64 * self.height as u64).try_into().unwrap();
                std::iter::repeat(ARGB32_ALPHA_MASK).take(size).collect()
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

#[op2]
pub fn op_canvas_2d_image_bitmap_from_canvas_state<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    canvas_state: *const c_void,
) -> v8::Local<'a, v8::External> {
    let canvas_state = borrow_v8::<CanvasState>(state, canvas_state);
    let result = ImageBitmap::from_canvas_state(&canvas_state);
    into_v8(state, scope, result)
}

#[op2]
pub fn op_canvas_2d_image_bitmap_from_canvas_state_crop<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    canvas_state: *const c_void,
    #[number] x: i64,
    #[number] y: i64,
    width: u32,
    height: u32,
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let canvas_state = borrow_v8::<CanvasState>(state, canvas_state);
    let result = ImageBitmap::from_canvas_state_crop(&canvas_state, x, y, width, height)?;
    Ok(into_v8(state, scope, result))
}

#[op2]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_image_bitmap_from_image_data_crop_and_resize<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
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
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let src = ImageDataView {
        width: src_width,
        height: src_height,
        color_space: CanvasColorSpace::from_repr(src_color_space).unwrap(),
        data: src_data,
    };
    let resize_quality = ResizeQuality::from_repr(resize_quality).unwrap();
    let image_orientation = ImageOrientation::from_repr(image_orientation).unwrap();
    let result = ImageBitmap::from_image_data_crop_and_resize(
        src,
        sx,
        sy,
        non_zero_u32(sw),
        non_zero_u32(sh),
        non_zero_u32(dw),
        non_zero_u32(dh),
        resize_quality,
        image_orientation,
    )?;
    Ok(into_v8(state, scope, result))
}

#[op2]
pub fn op_canvas_2d_image_bitmap_empty<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    width: u32,
    height: u32,
) -> v8::Local<'a, v8::External> {
    let result = ImageBitmap::empty(width, height, CanvasColorSpace::Srgb);
    into_v8(state, scope, result)
}

#[op2]
pub fn op_canvas_2d_image_bitmap_empty_resize<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    #[number] sw: u64,
    #[number] sh: u64,
    dw: u32,
    dh: u32,
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let size = aspect_resize(sw, sh, non_zero_u32(dw), non_zero_u32(dh))?;
    let result = ImageBitmap::empty(size.width, size.height, CanvasColorSpace::Srgb);
    Ok(into_v8(state, scope, result))
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_width(state: &OpState, this: *const c_void) -> u32 {
    let this = borrow_v8::<ImageBitmap>(state, this);
    this.width
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_height(state: &OpState, this: *const c_void) -> u32 {
    let this = borrow_v8::<ImageBitmap>(state, this);
    this.height
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_color_space(state: &OpState, this: *const c_void) -> i32 {
    let this = borrow_v8::<ImageBitmap>(state, this);
    this.color_space as i32
}

#[op2]
pub fn op_canvas_2d_image_bitmap_clone<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    this: *const c_void,
) -> v8::Local<'a, v8::External> {
    let this = borrow_v8::<ImageBitmap>(state, this);
    let result = this.clone();
    into_v8(state, scope, result)
}

#[op2]
pub fn op_canvas_2d_image_bitmap_crop<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    this: *const c_void,
    #[number] x: i64,
    #[number] y: i64,
    width: u32,
    height: u32,
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let this = borrow_v8::<ImageBitmap>(state, this);
    let width = non_zero_u32(width).unwrap_or(this.width);
    let height = non_zero_u32(height).unwrap_or(this.height);
    let result = this.crop(x, y, width, height)?;
    Ok(into_v8(state, scope, result))
}

#[op2]
pub fn op_canvas_2d_image_bitmap_resize<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    this: *const c_void,
    width: u32,
    height: u32,
    quality: i32,
    image_orientation: i32,
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let this = from_v8::<ImageBitmap>(state, this);
    let size = aspect_resize(
        this.width as u64,
        this.height as u64,
        non_zero_u32(width),
        non_zero_u32(height),
    )?;
    let quality = ResizeQuality::from_repr(quality).unwrap();
    let image_orientation = ImageOrientation::from_repr(image_orientation).unwrap();
    let result = this.resize(
        size.width,
        size.height,
        quality,
        matches!(image_orientation, ImageOrientation::FlipY),
    )?;
    Ok(into_v8(state, scope, result))
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_image_bitmap_get_image_data(
    state: &OpState,
    this: *const c_void,
    #[buffer] dst_data: &mut [u32],
    dst_width: u32,
    dst_height: u32,
    dst_color_space: i32,
    #[number] x: i64,
    #[number] y: i64,
) -> anyhow::Result<()> {
    let this = borrow_v8::<ImageBitmap>(state, this);
    let dst = AlignedImageDataViewMut {
        width: dst_width,
        height: dst_height,
        color_space: CanvasColorSpace::from_repr(dst_color_space).unwrap(),
        data: dst_data,
    };
    this.get_image_data(dst, x, y)
}

#[op2]
pub fn op_canvas_2d_image_bitmap_remove_alpha<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    this: *const c_void,
) -> v8::Local<'a, v8::External> {
    let this = from_v8::<ImageBitmap>(state, this);
    let result = this.remove_alpha();
    into_v8(state, scope, result)
}

#[op2(fast)]
pub fn op_canvas_2d_image_bitmap_close(state: &OpState, this: *const c_void) {
    drop(from_v8::<ImageBitmap>(state, this));
}
