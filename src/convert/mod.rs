mod tables;

use image::RgbaImage;
use palette::blend::{PreAlpha, Premultiply as _};
use palette::convert::{FromColorUnclamped as _, IntoColorUnclamped as _};
use palette::stimulus::IntoStimulus as _;
use palette::{LinSrgb, Srgb};

use super::css::color::{DisplayP3, LinDisplayP3};

pub fn premultiply(c: u8, a: u8) -> u8 {
    (((c as u32 * a as u32 + 128) * 257) >> 16) as u8
}

pub fn unpremultiply(c: u8, a: u8) -> u8 {
    ((c as u32 * tables::UNPREMULTIPLY[a as usize] + 32768) >> 16).min(255) as u8
}

pub type Rgba = (u8, u8, u8, u8);

pub fn pack_rgba8_to_argb32(pixels: &mut [u32], f: impl Fn(Rgba) -> Rgba) {
    for pixel in pixels {
        let [r, g, b, a] = pixel.to_ne_bytes();
        let (r, g, b, a) = f((r, g, b, a));
        *pixel = u32::from_be_bytes([a, r, g, b]);
    }
}

pub fn pack_bgra8_to_argb32(pixels: &mut [u32], f: impl Fn(Rgba) -> Rgba) {
    for pixel in pixels {
        let [b, g, r, a] = pixel.to_ne_bytes();
        let (r, g, b, a) = f((r, g, b, a));
        *pixel = u32::from_be_bytes([a, r, g, b]);
    }
}

pub fn unpack_argb32_to_rgba8(pixels: &mut [u32], f: impl Fn(Rgba) -> Rgba) {
    for pixel in pixels {
        let [a, r, g, b] = pixel.to_be_bytes();
        let (r, g, b, a) = f((r, g, b, a));
        *pixel = u32::from_ne_bytes([r, g, b, a]);
    }
}

pub fn permute_bgra8_to_rgba8(pixels: &mut [u32], f: impl Fn(Rgba) -> Rgba) {
    for pixel in pixels {
        let [b, g, r, a] = pixel.to_ne_bytes();
        let (r, g, b, a) = f((r, g, b, a));
        *pixel = u32::from_ne_bytes([r, g, b, a]);
    }
}

pub fn transform_rgba8(pixels: &mut [u32], f: impl Fn(Rgba) -> Rgba) {
    for pixel in pixels {
        let [r, g, b, a] = pixel.to_ne_bytes();
        let (r, g, b, a) = f((r, g, b, a));
        *pixel = u32::from_ne_bytes([r, g, b, a]);
    }
}

pub fn transform_argb32(pixels: &mut [u32], f: impl Fn(Rgba) -> Rgba) {
    for pixel in pixels {
        let [a, r, g, b] = pixel.to_be_bytes();
        let (r, g, b, a) = f((r, g, b, a));
        *pixel = u32::from_be_bytes([a, r, g, b]);
    }
}

pub fn transform_image(image: &mut RgbaImage, f: impl Fn(Rgba) -> Rgba) {
    for pixel in image.pixels_mut() {
        let [r, g, b, a] = pixel.0;
        let (r, g, b, a) = f((r, g, b, a));
        pixel.0 = [r, g, b, a];
    }
}

pub fn srgb_to_premultiplied_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    (
        premultiply(tables::SRGB_GAMMA_DECODE[r as usize], a),
        premultiply(tables::SRGB_GAMMA_DECODE[g as usize], a),
        premultiply(tables::SRGB_GAMMA_DECODE[b as usize], a),
        a,
    )
}

pub fn srgb_to_premultiplied_linear_display_p3((r, g, b, a): Rgba) -> Rgba {
    let c = LinDisplayP3::from_color_unclamped(Srgb::new(r, g, b).into_linear::<f32>())
        .premultiply(a.into_stimulus())
        .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn display_p3_to_premultiplied_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    let c = LinSrgb::from_color_unclamped(DisplayP3::new(r, g, b).into_linear::<f32>())
        .premultiply(a.into_stimulus())
        .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn linear_srgb_to_premultiplied_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    (premultiply(r, a), premultiply(g, a), premultiply(b, a), a)
}

pub fn linear_srgb_to_premultiplied_linear_display_p3((r, g, b, a): Rgba) -> Rgba {
    let c = LinDisplayP3::from_color_unclamped(LinSrgb::new(r, g, b).into_format::<f32>())
        .premultiply(a.into_stimulus())
        .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn linear_display_p3_to_premultiplied_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    let c = LinSrgb::from_color_unclamped(LinDisplayP3::new(r, g, b).into_format::<f32>())
        .premultiply(a.into_stimulus())
        .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn premultiplied_linear_srgb_to_srgb((r, g, b, a): Rgba) -> Rgba {
    (
        tables::UNPREMULTIPLY_AND_SRGB_GAMMA_ENCODE[((a as usize) << 8) | r as usize],
        tables::UNPREMULTIPLY_AND_SRGB_GAMMA_ENCODE[((a as usize) << 8) | g as usize],
        tables::UNPREMULTIPLY_AND_SRGB_GAMMA_ENCODE[((a as usize) << 8) | b as usize],
        a,
    )
}

pub fn premultiplied_linear_srgb_to_display_p3((r, g, b, a): Rgba) -> Rgba {
    let c = DisplayP3::from_linear(
        PreAlpha {
            color: LinSrgb::new(r, g, b).into_format::<f32>(),
            alpha: a.into_stimulus(),
        }
        .unpremultiply()
        .into_color_unclamped(),
    );
    (c.red, c.green, c.blue, a)
}

pub fn premultiplied_linear_srgb_to_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    (
        unpremultiply(r, a),
        unpremultiply(g, a),
        unpremultiply(b, a),
        a,
    )
}

pub fn premultiplied_linear_srgb_to_linear_display_p3((r, g, b, a): Rgba) -> Rgba {
    let c = LinDisplayP3::from_color_unclamped(
        PreAlpha {
            color: LinSrgb::new(r, g, b).into_format::<f32>(),
            alpha: a.into_stimulus(),
        }
        .unpremultiply(),
    )
    .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn premultiplied_linear_srgb_to_premultiplied_linear_display_p3((r, g, b, a): Rgba) -> Rgba {
    let c = LinDisplayP3::from_color_unclamped(
        PreAlpha {
            color: LinSrgb::new(r, g, b).into_format::<f32>(),
            alpha: a.into_stimulus(),
        }
        .unpremultiply(),
    )
    .premultiply(a.into_stimulus())
    .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn premultiplied_linear_display_p3_to_srgb((r, g, b, a): Rgba) -> Rgba {
    let c = Srgb::from_linear(
        PreAlpha {
            color: LinDisplayP3::new(r, g, b).into_format::<f32>(),
            alpha: a.into_stimulus(),
        }
        .unpremultiply()
        .into_color_unclamped(),
    );
    (c.red, c.green, c.blue, a)
}

pub fn premultiplied_linear_display_p3_to_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    let c = LinSrgb::from_color_unclamped(
        PreAlpha {
            color: LinDisplayP3::new(r, g, b).into_format::<f32>(),
            alpha: a.into_stimulus(),
        }
        .unpremultiply(),
    )
    .into_format();
    (c.red, c.green, c.blue, a)
}

pub fn premultiplied_linear_display_p3_to_premultiplied_linear_srgb((r, g, b, a): Rgba) -> Rgba {
    let c = LinSrgb::from_color_unclamped(
        PreAlpha {
            color: LinDisplayP3::new(r, g, b).into_format::<f32>(),
            alpha: a.into_stimulus(),
        }
        .unpremultiply(),
    )
    .premultiply(a.into_stimulus())
    .into_format();
    (c.red, c.green, c.blue, a)
}
