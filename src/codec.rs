use std::cell::Cell;
use std::io::Cursor;
use std::sync::LazyLock;

use deno_core::op2;
use image::error::EncodingError;
use image::imageops::replace;
use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageError, ImageFormat, ImageReader, RgbaImage};
use strum_macros::FromRepr;

use super::error::Canvas2DError;
use super::image_bitmap::{
    ImageBitmap, ImageOrientation, ResizeQuality, aspect_resize, non_zero_u32, out_of_bounds,
    same_size,
};
use super::image_data::ImageData;
use super::wrap::Wrap;
use super::{CanvasColorSpace, PredefinedColorSpace};

fn encode_png(
    data: &[u8],
    width: u64,
    height: u64,
    color_space: PredefinedColorSpace,
) -> Result<Vec<u8>, png::EncodingError> {
    use png::EncodingError::LimitsExceeded;
    use png::chunk;

    let width = width.try_into().map_err(|_| LimitsExceeded)?;
    let height = height.try_into().map_err(|_| LimitsExceeded)?;
    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    match color_space {
        PredefinedColorSpace::Srgb => writer.write_chunk(chunk::cICP, &[1, 13, 0, 1])?,
        PredefinedColorSpace::SrgbLinear => writer.write_chunk(chunk::cICP, &[1, 8, 0, 1])?,
        PredefinedColorSpace::DisplayP3 => writer.write_chunk(chunk::cICP, &[12, 13, 0, 1])?,
        PredefinedColorSpace::DisplayP3Linear => writer.write_chunk(chunk::cICP, &[12, 8, 0, 1])?,
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
    let color_space = PredefinedColorSpace::from_repr(color_space).unwrap();
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

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum ColorSpace {
    FromImage,
    Srgb,
}

fn transform_to_srgb(image: &mut RgbaImage, profile: Option<&[u8]>) {
    use qcms::{DataType, Intent, Profile, Transform};

    static SRGB: LazyLock<Profile> = LazyLock::new(|| {
        let mut profile = *Profile::new_sRGB();
        profile.precache_output_transform();
        profile
    });
    if let Some(profile) = profile.and_then(|buf| Profile::new_from_slice(buf, false))
        && let Some(transform) = Transform::new(&profile, &SRGB, DataType::RGBA8, Intent::default())
    {
        transform.apply(image);
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
    color_space: ColorSpace,
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
    let mut decoder = ImageReader::with_format(Cursor::new(buf), format)
        .into_decoder()
        .map_err(Canvas2DError::DecodeImage)?;
    let profile = decoder.icc_profile().map_err(Canvas2DError::DecodeImage)?;
    let orientation = decoder.orientation().map_err(Canvas2DError::DecodeImage)?;
    let orientation = match image_orientation {
        ImageOrientation::FromImage => orientation,
        ImageOrientation::FlipY => Orientation::FlipVertical,
    };
    let image = DynamicImage::from_decoder(decoder).map_err(Canvas2DError::DecodeImage)?;
    let width = image.width();
    let height = image.height();
    let sw = sw.unwrap_or(width);
    let sh = sh.unwrap_or(height);
    let (dw, dh) = aspect_resize(sw as u64, sh as u64, dw, dh)?.to_tuple();
    let (dw, dh) = match orientation {
        Orientation::NoTransforms
        | Orientation::Rotate180
        | Orientation::FlipHorizontal
        | Orientation::FlipVertical => (dw, dh),
        Orientation::Rotate90
        | Orientation::Rotate270
        | Orientation::Rotate90FlipH
        | Orientation::Rotate270FlipH => (dh, dw),
    };
    if out_of_bounds(width, height, sx, sy, sw, sh) {
        return Ok(ImageBitmap {
            width: dw,
            height: dh,
            color_space: CanvasColorSpace::Srgb,
            data: None,
        });
    }
    let mut image = if same_size(width, height, sx, sy, sw, sh) {
        image
    } else {
        let mut tmp = DynamicImage::new(sw, sh, image.color());
        replace(&mut tmp, &image, -sx, -sy);
        tmp
    };
    image.apply_orientation(orientation);
    let mut image = image.into_rgba8();
    match color_space {
        ColorSpace::FromImage => transform_to_srgb(&mut image, profile.as_deref()),
        ColorSpace::Srgb => {}
    }
    let sw = image.width();
    let sh = image.height();
    let data = image.into_vec();
    Ok(ImageBitmap::from_image_data_resize(
        ImageData {
            width: sw,
            height: sh,
            color_space: PredefinedColorSpace::Srgb,
            data,
        },
        dw,
        dh,
        resize_quality,
        false,
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
    color_space: i32,
) -> Result<Wrap<Cell<ImageBitmap>>, Canvas2DError> {
    let resize_quality = ResizeQuality::from_repr(resize_quality).unwrap();
    let image_orientation = ImageOrientation::from_repr(image_orientation).unwrap();
    let color_space = ColorSpace::from_repr(color_space).unwrap();
    Ok(Wrap::new(Cell::new(decode_image(
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
        color_space,
    )?)))
}
