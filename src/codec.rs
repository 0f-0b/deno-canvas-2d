use std::cell::RefCell;

use deno_core::op2;
use image::error::EncodingError;
use image::imageops::replace;
use image::{GenericImageView as _, ImageError, ImageFormat, RgbaImage};

use super::CanvasColorSpace;
use super::error::Canvas2DError;
use super::image_bitmap::{
    ImageBitmap, ImageOrientation, ResizeQuality, aspect_resize, non_zero_u32, out_of_bounds,
    same_size,
};
use super::image_data::ImageData;
use super::wrap::Wrap;

fn encode_png(
    data: &[u8],
    width: u64,
    height: u64,
    color_space: CanvasColorSpace,
) -> Result<Vec<u8>, png::EncodingError> {
    use png::EncodingError::LimitsExceeded;

    mod chunk {
        #![allow(non_upper_case_globals)]
        use png::chunk::*;

        pub const cICP: ChunkType = ChunkType(*b"cICP");
    }

    let width = width.try_into().map_err(|_| LimitsExceeded)?;
    let height = height.try_into().map_err(|_| LimitsExceeded)?;
    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    match color_space {
        CanvasColorSpace::Srgb => writer.write_chunk(chunk::cICP, &[1, 13, 0, 1])?,
        CanvasColorSpace::DisplayP3 => writer.write_chunk(chunk::cICP, &[12, 13, 0, 1])?,
    }
    writer.write_image_data(data)?;
    writer.finish()?;
    Ok(buf)
}

#[op2]
#[buffer]
pub fn op_canvas_2d_encode_png(
    #[buffer] data: &[u8],
    #[number] width: u64,
    #[number] height: u64,
    color_space: i32,
) -> Result<Vec<u8>, Canvas2DError> {
    let color_space = CanvasColorSpace::from_repr(color_space).unwrap();
    encode_png(data, width, height, color_space).map_err(|e| {
        Canvas2DError::EncodeImage(ImageError::Encoding(EncodingError::new(
            ImageFormat::Png.into(),
            e,
        )))
    })
}

fn mimesniff_image<'a>(header: &[u8], supplied_type: &'a str) -> &'a str {
    if matches!(supplied_type, "text/xml" | "application/xml") || supplied_type.ends_with("+xml") {
        return supplied_type;
    }
    match header {
        [0x00, 0x00, 0x01 | 0x02, 0x00, ..] => "image/x-icon",
        [0x42, 0x4d, ..] => "image/bmp",
        [0x47, 0x49, 0x46, 0x38, 0x37 | 0x39, 0x61, ..] => "image/gif",
        [
            0x52,
            0x49,
            0x46,
            0x46,
            _,
            _,
            _,
            _,
            0x57,
            0x45,
            0x42,
            0x50,
            0x56,
            0x50,
            ..,
        ] => "image/webp",
        [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, ..] => "image/png",
        [0xff, 0xd8, 0xff, ..] => "image/jpeg",
        _ => supplied_type,
    }
}

#[allow(clippy::too_many_arguments)]
fn decode_image(
    buf: &[u8],
    mime_type: &str,
    sx: i64,
    sy: i64,
    sw: Option<u32>,
    sh: Option<u32>,
    dw: Option<u32>,
    dh: Option<u32>,
    resize_quality: ResizeQuality,
    image_orientation: ImageOrientation,
) -> Result<ImageBitmap, Canvas2DError> {
    let format = {
        let essence = mime_type
            .split(';')
            .next()
            .unwrap()
            .trim()
            .to_ascii_lowercase();
        match ImageFormat::from_mime_type(mimesniff_image(buf, &essence)) {
            Some(format) if format.reading_enabled() => format,
            _ => return Err(Canvas2DError::UnsupportedImageFormat { mime_type: essence }),
        }
    };
    let src =
        image::load_from_memory_with_format(buf, format).map_err(Canvas2DError::DecodeImage)?;
    let (src_width, src_height) = src.dimensions();
    let sw = sw.unwrap_or(src_width);
    let sh = sh.unwrap_or(src_height);
    let (dw, dh) = aspect_resize(sw as u64, sh as u64, dw, dh)?.to_tuple();
    if out_of_bounds(src_width, src_height, sx, sy, sw, sh) {
        return Ok(ImageBitmap {
            width: dw,
            height: dh,
            color_space: CanvasColorSpace::Srgb,
            data: None,
        });
    }
    let flip_y = match image_orientation {
        ImageOrientation::FromImage => false,
        ImageOrientation::FlipY => true,
    };
    let cropped = if same_size(src_width, src_height, sx, sy, sw, sh) {
        src.into_rgba8()
    } else {
        let mut tmp = RgbaImage::new(sw, sh);
        replace(&mut tmp, &src, -sx, -sy);
        tmp
    };
    Ok(ImageBitmap::from_image_data_resize(
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

#[op2]
#[cppgc]
pub fn op_canvas_2d_decode_image(
    #[buffer] buf: &[u8],
    #[string] mime_type: &str,
    #[number] sx: i64,
    #[number] sy: i64,
    sw: u32,
    sh: u32,
    dw: u32,
    dh: u32,
    resize_quality: i32,
    image_orientation: i32,
) -> Result<Wrap<RefCell<ImageBitmap>>, Canvas2DError> {
    let resize_quality = ResizeQuality::from_repr(resize_quality).unwrap();
    let image_orientation = ImageOrientation::from_repr(image_orientation).unwrap();
    Ok(Wrap::new(RefCell::new(decode_image(
        buf,
        mime_type,
        sx,
        sy,
        non_zero_u32(sw),
        non_zero_u32(sh),
        non_zero_u32(dw),
        non_zero_u32(dh),
        resize_quality,
        image_orientation,
    )?)))
}
