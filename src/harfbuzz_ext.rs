#![allow(clippy::undocumented_unsafe_blocks)] // TODO document these

use std::ffi::{c_int, c_uint};
use std::mem::MaybeUninit;

use harfbuzz_rs::{self as hb, HarfbuzzObject};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VariationAxisInfo {
    pub axis_index: c_uint,
    pub tag: hb::Tag,
    pub name_id: c_uint,
    pub flags: c_uint,
    pub min_value: f32,
    pub default_value: f32,
    pub max_value: f32,
}

unsafe extern "C" {
    fn hb_font_set_synthetic_bold(
        font: *mut <hb::Font as HarfbuzzObject>::Raw,
        x_embolden: f32,
        y_embolden: f32,
        in_place: c_int,
    );
    fn hb_font_set_synthetic_slant(font: *mut <hb::Font as HarfbuzzObject>::Raw, slant: f32);
    fn hb_ot_layout_get_baseline(
        font: *mut <hb::Font as HarfbuzzObject>::Raw,
        baseline_tag: u32,
        direction: c_uint,
        script_tag: u32,
        language_tag: u32,
        coord: *mut i32,
    ) -> c_int;
    fn hb_ot_metrics_get_position(
        font: *mut <hb::Font as HarfbuzzObject>::Raw,
        metrics_tag: u32,
        position: *mut i32,
    ) -> c_int;
    fn hb_ot_var_find_axis_info(
        font: *mut <hb::Face as HarfbuzzObject>::Raw,
        axis_tag: u32,
        axis_info: *mut VariationAxisInfo,
    ) -> c_int;
}

pub trait FaceExt {
    fn find_variation_axis_info(&self, axis_tag: impl Into<hb::Tag>) -> Option<VariationAxisInfo>;
}

impl FaceExt for hb::Face<'_> {
    fn find_variation_axis_info(&self, axis_tag: impl Into<hb::Tag>) -> Option<VariationAxisInfo> {
        let font = self.as_raw();
        let axis_tag = axis_tag.into().0;
        let mut axis_info = MaybeUninit::uninit();
        let result = unsafe { hb_ot_var_find_axis_info(font, axis_tag, axis_info.as_mut_ptr()) };
        (result != 0).then(|| unsafe { axis_info.assume_init() })
    }
}

pub trait FontExt {
    fn set_synthetic_bold(&mut self, x_embolden: f32, y_embolden: f32, in_place: bool);

    fn set_synthetic_slant(&mut self, slant: f32);

    fn get_baseline(
        &self,
        baseline_tag: impl Into<hb::Tag>,
        direction: hb::Direction,
        script_tag: impl Into<hb::Tag>,
        language_tag: impl Into<hb::Tag>,
    ) -> Option<hb::Position>;

    fn get_generic_baseline(&self, baseline_tag: impl Into<hb::Tag>) -> Option<hb::Position> {
        self.get_baseline(baseline_tag, hb::Direction::Ltr, b"DFLT", b"dflt")
    }

    fn get_position(&self, metrics_tag: impl Into<hb::Tag>) -> Option<hb::Position>;
}

impl FontExt for hb::Font<'_> {
    fn set_synthetic_bold(&mut self, x_embolden: f32, y_embolden: f32, in_place: bool) {
        let font = self.as_raw_mut();
        let in_place = in_place as c_int;
        unsafe { hb_font_set_synthetic_bold(font, x_embolden, y_embolden, in_place) }
    }

    fn set_synthetic_slant(&mut self, slant: f32) {
        let font = self.as_raw_mut();
        unsafe { hb_font_set_synthetic_slant(font, slant) }
    }

    fn get_baseline(
        &self,
        baseline_tag: impl Into<hb::Tag>,
        direction: hb::Direction,
        script_tag: impl Into<hb::Tag>,
        language_tag: impl Into<hb::Tag>,
    ) -> Option<hb::Position> {
        let font = self.as_raw();
        let baseline_tag = baseline_tag.into().0;
        let direction = direction.to_raw();
        let script_tag = script_tag.into().0;
        let language_tag = language_tag.into().0;
        let mut coord = MaybeUninit::uninit();
        let result = unsafe {
            hb_ot_layout_get_baseline(
                font,
                baseline_tag,
                direction,
                script_tag,
                language_tag,
                coord.as_mut_ptr(),
            )
        };
        (result != 0).then(|| unsafe { coord.assume_init() })
    }

    fn get_position(&self, metrics_tag: impl Into<hb::Tag>) -> Option<hb::Position> {
        let font = self.as_raw();
        let metrics_tag = metrics_tag.into().0;
        let mut position = MaybeUninit::uninit();
        let result =
            unsafe { hb_ot_metrics_get_position(font, metrics_tag, position.as_mut_ptr()) };
        (result != 0).then(|| unsafe { position.assume_init() })
    }
}
