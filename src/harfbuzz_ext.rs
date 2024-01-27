use std::ffi::{c_int, c_uint};

use harfbuzz_rs::{self as hb, HarfbuzzObject};

extern "C" {
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
}

pub trait FontExt {
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
        let mut coord = 0;
        let result = unsafe {
            hb_ot_layout_get_baseline(
                font,
                baseline_tag,
                direction,
                script_tag,
                language_tag,
                &mut coord,
            )
        };
        (result != 0).then_some(coord)
    }

    fn get_position(&self, metrics_tag: impl Into<hb::Tag>) -> Option<hb::Position> {
        let font = self.as_raw();
        let metrics_tag = metrics_tag.into().0;
        let mut position = 0;
        let result = unsafe { hb_ot_metrics_get_position(font, metrics_tag, &mut position) };
        (result != 0).then_some(position)
    }
}
