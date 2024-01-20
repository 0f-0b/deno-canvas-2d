mod blur;
mod codec;
mod convert;
mod css_color;
mod gc;
pub mod gradient;
pub mod image_bitmap;
mod image_data;
mod matrix;
pub mod path;
pub mod pattern;
mod raqote_ext;
pub mod state;

use css_color::{parse_and_compute_color, AbsoluteColor, ComputedColor};
use cssparser::BasicParseError;
use deno_core::anyhow;
use deno_core::error::range_error;
use euclid::default::{Point2D, Size2D};
use euclid::{point2, size2};
use palette::stimulus::IntoStimulus as _;
use strum_macros::FromRepr;

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
enum CanvasColorSpace {
    Srgb,
    DisplayP3,
}

fn to_raqote_point(x: i64, y: i64) -> anyhow::Result<Point2D<i32>> {
    let x = x
        .try_into()
        .map_err(|_| range_error(format!("Invalid x: {x}")))?;
    let y = y
        .try_into()
        .map_err(|_| range_error(format!("Invalid y: {y}")))?;
    Ok(point2(x, y))
}

fn to_raqote_size(width: u64, height: u64) -> anyhow::Result<Size2D<i32>> {
    let width = width
        .try_into()
        .map_err(|_| range_error(format!("Invalid width: {width}")))?;
    let height = height
        .try_into()
        .map_err(|_| range_error(format!("Invalid height: {height}")))?;
    let size = width as u64 * height as u64;
    if size > i32::MAX as u64 {
        return Err(range_error(format!("Invalid size: {size}")));
    }
    Ok(size2(width, height))
}

fn to_raqote_color(
    color: AbsoluteColor,
    destination_color_space: CanvasColorSpace,
) -> raqote::Color {
    let alpha = color.alpha;
    let (red, green, blue) = match destination_color_space {
        CanvasColorSpace::Srgb => color.value.into_linear_srgb().into_components(),
        CanvasColorSpace::DisplayP3 => color.value.into_linear_display_p3().into_components(),
    };
    raqote::Color::new(
        alpha.into_stimulus(),
        red.into_stimulus(),
        green.into_stimulus(),
        blue.into_stimulus(),
    )
}

fn to_raqote_solid_source(
    color: AbsoluteColor,
    destination_color_space: CanvasColorSpace,
) -> raqote::SolidSource {
    let alpha = color.alpha;
    let (red, green, blue) = match destination_color_space {
        CanvasColorSpace::Srgb => color.value.into_linear_srgb().into_components(),
        CanvasColorSpace::DisplayP3 => color.value.into_linear_display_p3().into_components(),
    };
    raqote::SolidSource {
        r: (red * alpha).into_stimulus(),
        g: (green * alpha).into_stimulus(),
        b: (blue * alpha).into_stimulus(),
        a: alpha.into_stimulus(),
    }
}

fn parse_color_for_canvas(css: &str) -> Result<AbsoluteColor, BasicParseError> {
    parse_and_compute_color(css).map(|computed| match computed {
        ComputedColor::Absolute(c) => c,
        ComputedColor::CurrentColor => AbsoluteColor::OPAQUE_BLACK,
    })
}

fn serialize_color_for_canvas(color: AbsoluteColor) -> String {
    let alpha = color.alpha;
    let srgb = color.value.into_srgb();
    let r = (srgb.red * 255.0).round() as u8;
    let g = (srgb.green * 255.0).round() as u8;
    let b = (srgb.blue * 255.0).round() as u8;
    let a = (alpha * 1000.0).round() / 1000.0;
    if a == 1.0 {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        format!("rgba({r}, {g}, {b}, {a})")
    }
}

fn premultiply(c: u8, a: u8) -> u8 {
    (((c as u32 * a as u32 + 128) * 257) >> 16) as u8
}

deno_core::extension!(
    deno_canvas_2d,
    deps = [deno_web],
    ops = [
        matrix::op_canvas_2d_parse_matrix,
        codec::op_canvas_2d_encode_png,
        codec::op_canvas_2d_decode_image,
        state::op_canvas_2d_state_new,
        state::op_canvas_2d_state_width,
        state::op_canvas_2d_state_set_width,
        state::op_canvas_2d_state_height,
        state::op_canvas_2d_state_set_height,
        state::op_canvas_2d_state_save,
        state::op_canvas_2d_state_restore,
        state::op_canvas_2d_state_reset,
        state::op_canvas_2d_state_clear,
        state::op_canvas_2d_state_line_width,
        state::op_canvas_2d_state_set_line_width,
        state::op_canvas_2d_state_line_cap,
        state::op_canvas_2d_state_set_line_cap,
        state::op_canvas_2d_state_line_join,
        state::op_canvas_2d_state_set_line_join,
        state::op_canvas_2d_state_miter_limit,
        state::op_canvas_2d_state_set_miter_limit,
        state::op_canvas_2d_state_dash_list,
        state::op_canvas_2d_state_set_dash_list,
        state::op_canvas_2d_state_line_dash_offset,
        state::op_canvas_2d_state_set_line_dash_offset,
        state::op_canvas_2d_state_font,
        state::op_canvas_2d_state_set_font,
        state::op_canvas_2d_state_text_align,
        state::op_canvas_2d_state_set_text_align,
        state::op_canvas_2d_state_text_baseline,
        state::op_canvas_2d_state_set_text_baseline,
        state::op_canvas_2d_state_direction,
        state::op_canvas_2d_state_set_direction,
        state::op_canvas_2d_state_letter_spacing,
        state::op_canvas_2d_state_set_letter_spacing,
        state::op_canvas_2d_state_word_spacing,
        state::op_canvas_2d_state_set_word_spacing,
        state::op_canvas_2d_state_font_kerning,
        state::op_canvas_2d_state_set_font_kerning,
        state::op_canvas_2d_state_font_stretch,
        state::op_canvas_2d_state_set_font_stretch,
        state::op_canvas_2d_state_font_variant_caps,
        state::op_canvas_2d_state_set_font_variant_caps,
        state::op_canvas_2d_state_text_rendering,
        state::op_canvas_2d_state_set_text_rendering,
        state::op_canvas_2d_state_scale,
        state::op_canvas_2d_state_rotate,
        state::op_canvas_2d_state_translate,
        state::op_canvas_2d_state_transform,
        state::op_canvas_2d_state_get_transform,
        state::op_canvas_2d_state_set_transform,
        state::op_canvas_2d_state_reset_transform,
        state::op_canvas_2d_state_fill_style,
        state::op_canvas_2d_state_set_fill_style_color,
        state::op_canvas_2d_state_set_fill_style_gradient,
        state::op_canvas_2d_state_set_fill_style_pattern,
        state::op_canvas_2d_state_stroke_style,
        state::op_canvas_2d_state_set_stroke_style_color,
        state::op_canvas_2d_state_set_stroke_style_gradient,
        state::op_canvas_2d_state_set_stroke_style_pattern,
        state::op_canvas_2d_state_clear_rect,
        state::op_canvas_2d_state_fill_rect,
        state::op_canvas_2d_state_stroke_rect,
        state::op_canvas_2d_state_fill_text,
        state::op_canvas_2d_state_stroke_text,
        state::op_canvas_2d_state_measure_text,
        state::op_canvas_2d_state_fill,
        state::op_canvas_2d_state_stroke,
        state::op_canvas_2d_state_clip,
        state::op_canvas_2d_state_is_point_in_path,
        state::op_canvas_2d_state_is_point_in_stroke,
        state::op_canvas_2d_state_draw_image,
        state::op_canvas_2d_state_get_image_data,
        state::op_canvas_2d_state_put_image_data,
        state::op_canvas_2d_state_global_alpha,
        state::op_canvas_2d_state_set_global_alpha,
        state::op_canvas_2d_state_global_composite_operation,
        state::op_canvas_2d_state_set_global_composite_operation,
        state::op_canvas_2d_state_image_smoothing_enabled,
        state::op_canvas_2d_state_set_image_smoothing_enabled,
        state::op_canvas_2d_state_image_smoothing_quality,
        state::op_canvas_2d_state_set_image_smoothing_quality,
        state::op_canvas_2d_state_shadow_color,
        state::op_canvas_2d_state_set_shadow_color,
        state::op_canvas_2d_state_shadow_offset_x,
        state::op_canvas_2d_state_set_shadow_offset_x,
        state::op_canvas_2d_state_shadow_offset_y,
        state::op_canvas_2d_state_set_shadow_offset_y,
        state::op_canvas_2d_state_shadow_blur,
        state::op_canvas_2d_state_set_shadow_blur,
        state::op_canvas_2d_state_set_filter,
        gradient::op_canvas_2d_gradient_new_linear,
        gradient::op_canvas_2d_gradient_new_radial,
        gradient::op_canvas_2d_gradient_new_conic,
        gradient::op_canvas_2d_gradient_add_color_stop,
        pattern::op_canvas_2d_pattern_new,
        pattern::op_canvas_2d_pattern_set_transform,
        path::op_canvas_2d_path_new,
        path::op_canvas_2d_path_clone,
        path::op_canvas_2d_path_clear,
        path::op_canvas_2d_path_extend,
        path::op_canvas_2d_path_ensure_subpath,
        path::op_canvas_2d_path_move_to,
        path::op_canvas_2d_path_line_to,
        path::op_canvas_2d_path_quad_to,
        path::op_canvas_2d_path_cubic_to,
        path::op_canvas_2d_path_arc_to,
        path::op_canvas_2d_path_ellipse,
        path::op_canvas_2d_path_rect,
        path::op_canvas_2d_path_round_rect,
        path::op_canvas_2d_path_close,
        image_bitmap::op_canvas_2d_image_bitmap_from_canvas_state,
        image_bitmap::op_canvas_2d_image_bitmap_from_canvas_state_crop,
        image_bitmap::op_canvas_2d_image_bitmap_from_image_data_crop_and_resize,
        image_bitmap::op_canvas_2d_image_bitmap_empty,
        image_bitmap::op_canvas_2d_image_bitmap_empty_resize,
        image_bitmap::op_canvas_2d_image_bitmap_width,
        image_bitmap::op_canvas_2d_image_bitmap_height,
        image_bitmap::op_canvas_2d_image_bitmap_clone,
        image_bitmap::op_canvas_2d_image_bitmap_crop,
        image_bitmap::op_canvas_2d_image_bitmap_resize,
        image_bitmap::op_canvas_2d_image_bitmap_close,
    ],
    esm = [
        dir "js",
        "00_array_buffer_primordials.js",
        "00_blob_primordials.js",
        "00_event_primordials.js",
        "00_event_target_primordials.js",
        "00_image_data_primordials.js",
        "00_ops.js",
        "01_default_to.js",
        "01_promise.js",
        "01_require_object.js",
        "01_same_value_zero.js",
        "01_try_get_array_buffer_resizable.js",
        "01_try_get_blob_size.js",
        "01_try_get_image_data_data.js",
        "02_is_blob.js",
        "02_is_image_data.js",
        "02_require_fixed_array_buffer.js",
        "04_create_dictionary_converter.js",
        "04_create_enum_converter.js",
        "04_create_sequence_from_iterable.js",
        "05_convert_boolean.js",
        "05_convert_dom_string.js",
        "05_convert_double.js",
        "05_convert_enforce_range_long.js",
        "05_convert_enforce_range_unsigned_long.js",
        "05_convert_enforce_range_unsigned_long_long.js",
        "05_convert_event_handler.js",
        "05_convert_float32_array.js",
        "05_convert_float64_array.js",
        "05_convert_image_data.js",
        "05_convert_legacy_null_to_empty_string_dom_string.js",
        "05_convert_long.js",
        "05_convert_unrestricted_double.js",
        "15_event.js",
        "15_geometry.js",
        "16_canvas.js",
        "17_context_2d.js",
    ],
    state = |state| {
        gc::init(state);
    },
);

pub fn get_error_class_name(e: &anyhow::Error) -> Option<&'static str> {
    if e.is::<png::EncodingError>() {
        return Some("DOMExceptionEncodingError");
    }
    if e.is::<image::ImageError>() {
        return Some("DOMExceptionInvalidStateError");
    }
    None
}
