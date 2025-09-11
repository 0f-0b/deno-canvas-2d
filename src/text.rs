use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::num::NonZeroU64;
use std::ops::Range;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{process, ptr};

use cssparser::ToCss as _;
use deno_core::{GarbageCollected, OpState, op2, v8};
use euclid::default::{Box2D, Point2D, Transform2D, Vector2D};
use euclid::{point2, size2, vec2};
use harfbuzz_rs as hb;
use hashlink::LinkedHashMap;
use unicase::UniCase;
use unicode_bidi::{self as bidi, ParagraphBidiInfo};

use super::css::font::font_face::{
    ComputedFontStyleRange, ComputedFontWeightRange, ComputedFontWidthRange, SpecifiedFontDisplay,
    SpecifiedFontFeatureSettings, SpecifiedFontSource, SpecifiedFontSources,
    SpecifiedFontStyleRange, SpecifiedFontVariationSettings, SpecifiedFontWeightRange,
    SpecifiedFontWidthRange, SpecifiedMetricsOverride, SpecifiedMetricsOverrideValue,
    SpecifiedUnicodeRange,
};
use super::css::font::{
    ComputedFamilyName, ComputedFont, ComputedFontStyle, ComputedFontVariantCaps,
    ComputedFontWeight, ComputedFontWidth, SpecifiedSpecificFamily,
};
use super::css::{self, FromCss as _, UnicodeRangeSet};
use super::error::Canvas2DError;
use super::path::Path;
use super::state::{
    CanvasDirection, CanvasFontKerning, CanvasTextAlign, CanvasTextBaseline, CanvasTextRendering,
    DrawingState,
};
use super::wrap::Wrap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontFaceId(NonZeroU64);

impl FontFaceId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(
            NonZeroU64::new(NEXT.fetch_add(1, Ordering::Relaxed))
                .unwrap_or_else(|| process::abort()),
        )
    }

    pub fn as_u64(self) -> u64 {
        self.0.get()
    }
}

#[derive(Debug)]
pub struct CachedFamily {
    specified: SpecifiedSpecificFamily,
    casefolded: String,
}

impl CachedFamily {
    pub fn new(specified: SpecifiedSpecificFamily) -> Self {
        let casefolded = UniCase::new(specified.name.as_ref()).to_folded_case();
        Self {
            specified,
            casefolded,
        }
    }
}

#[derive(Debug)]
pub struct CachedStyle {
    specified: SpecifiedFontStyleRange,
    computed: ComputedFontStyleRange,
}

impl CachedStyle {
    pub fn new(specified: SpecifiedFontStyleRange) -> Self {
        let computed = specified.compute();
        Self {
            specified,
            computed,
        }
    }
}

#[derive(Debug)]
pub struct CachedWeight {
    specified: SpecifiedFontWeightRange,
    computed: ComputedFontWeightRange,
}

impl CachedWeight {
    pub fn new(specified: SpecifiedFontWeightRange) -> Self {
        let computed = specified.compute();
        Self {
            specified,
            computed,
        }
    }
}

#[derive(Debug)]
pub struct CachedWidth {
    specified: SpecifiedFontWidthRange,
    computed: ComputedFontWidthRange,
}

impl CachedWidth {
    pub fn new(specified: SpecifiedFontWidthRange) -> Self {
        let computed = specified.compute();
        Self {
            specified,
            computed,
        }
    }
}

#[derive(Debug)]
pub struct CachedUnicodeRange {
    specified: SpecifiedUnicodeRange,
    simplified: UnicodeRangeSet,
}

impl CachedUnicodeRange {
    pub fn new(specified: SpecifiedUnicodeRange) -> Self {
        let simplified = UnicodeRangeSet::new(specified.range_list.iter().cloned());
        Self {
            specified,
            simplified,
        }
    }
}

#[derive(Debug, Default)]
pub enum FontFaceState {
    #[default]
    Unloaded,
    Loaded(hb::Shared<hb::Font<'static>>),
    Errored,
}

#[derive(Debug)]
pub struct FontFaceData {
    family: CachedFamily,
    style: CachedStyle,
    weight: CachedWeight,
    width: CachedWidth,
    unicode_range: CachedUnicodeRange,
    feature_settings: SpecifiedFontFeatureSettings,
    variation_settings: SpecifiedFontVariationSettings,
    display: SpecifiedFontDisplay,
    ascent_override: SpecifiedMetricsOverride,
    descent_override: SpecifiedMetricsOverride,
    line_gap_override: SpecifiedMetricsOverride,
    state: FontFaceState,
}

impl FontFaceData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: SpecifiedSpecificFamily,
        style: SpecifiedFontStyleRange,
        weight: SpecifiedFontWeightRange,
        width: SpecifiedFontWidthRange,
        unicode_range: SpecifiedUnicodeRange,
        feature_settings: SpecifiedFontFeatureSettings,
        variation_settings: SpecifiedFontVariationSettings,
        display: SpecifiedFontDisplay,
        ascent_override: SpecifiedMetricsOverride,
        descent_override: SpecifiedMetricsOverride,
        line_gap_override: SpecifiedMetricsOverride,
        state: FontFaceState,
    ) -> Self {
        Self {
            family: CachedFamily::new(family),
            style: CachedStyle::new(style),
            weight: CachedWeight::new(weight),
            width: CachedWidth::new(width),
            unicode_range: CachedUnicodeRange::new(unicode_range),
            feature_settings,
            variation_settings,
            display,
            ascent_override,
            descent_override,
            line_gap_override,
            state,
        }
    }

    pub fn family(&self) -> SpecifiedSpecificFamily {
        self.family.specified.clone()
    }

    pub fn set_family(&mut self, value: SpecifiedSpecificFamily) {
        self.family = CachedFamily::new(value);
    }

    pub fn style(&self) -> SpecifiedFontStyleRange {
        self.style.specified
    }

    pub fn set_style(&mut self, value: SpecifiedFontStyleRange) {
        self.style = CachedStyle::new(value);
    }

    pub fn weight(&self) -> SpecifiedFontWeightRange {
        self.weight.specified
    }

    pub fn set_weight(&mut self, value: SpecifiedFontWeightRange) {
        self.weight = CachedWeight::new(value);
    }

    pub fn width(&self) -> SpecifiedFontWidthRange {
        self.width.specified
    }

    pub fn set_width(&mut self, value: SpecifiedFontWidthRange) {
        self.width = CachedWidth::new(value);
    }

    pub fn unicode_range(&self) -> SpecifiedUnicodeRange {
        self.unicode_range.specified.clone()
    }

    pub fn set_unicode_range(&mut self, value: SpecifiedUnicodeRange) {
        self.unicode_range = CachedUnicodeRange::new(value);
    }

    pub fn feature_settings(&self) -> SpecifiedFontFeatureSettings {
        self.feature_settings.clone()
    }

    pub fn set_feature_settings(&mut self, value: SpecifiedFontFeatureSettings) {
        self.feature_settings = value;
    }

    pub fn variation_settings(&self) -> SpecifiedFontVariationSettings {
        self.variation_settings.clone()
    }

    pub fn set_variation_settings(&mut self, value: SpecifiedFontVariationSettings) {
        self.variation_settings = value;
    }

    pub fn display(&self) -> SpecifiedFontDisplay {
        self.display
    }

    pub fn set_display(&mut self, value: SpecifiedFontDisplay) {
        self.display = value;
    }

    pub fn ascent_override(&self) -> SpecifiedMetricsOverride {
        self.ascent_override
    }

    pub fn set_ascent_override(&mut self, value: SpecifiedMetricsOverride) {
        self.ascent_override = value;
    }

    pub fn descent_override(&self) -> SpecifiedMetricsOverride {
        self.descent_override
    }

    pub fn set_descent_override(&mut self, value: SpecifiedMetricsOverride) {
        self.descent_override = value;
    }

    pub fn line_gap_override(&self) -> SpecifiedMetricsOverride {
        self.line_gap_override
    }

    pub fn set_line_gap_override(&mut self, value: SpecifiedMetricsOverride) {
        self.line_gap_override = value;
    }

    pub fn load(&mut self, blob: &[u8], from_url: bool) -> Result<(), Canvas2DError> {
        match self.state {
            FontFaceState::Unloaded => {
                let blob = fontsan::process(blob).map_err(|_| {
                    if from_url {
                        Canvas2DError::DecodeFontFromUrl
                    } else {
                        Canvas2DError::DecodeFont
                    }
                })?;
                self.state = FontFaceState::Loaded(hb::Font::new(hb::Face::new(blob, 0)).into());
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct FontFace {
    id: FontFaceId,
    data: RefCell<FontFaceData>,
}

impl FontFace {
    pub fn new(data: FontFaceData) -> Self {
        Self {
            id: FontFaceId::new(),
            data: RefCell::new(data),
        }
    }

    pub fn id(&self) -> FontFaceId {
        self.id
    }

    pub fn data(&self) -> &RefCell<FontFaceData> {
        &self.data
    }
}

// SAFETY: this type has no members.
unsafe impl GarbageCollected for Wrap<Rc<FontFace>> {
    fn get_name(&self) -> &'static CStr {
        c"FontFace"
    }

    fn trace(&self, _: &mut v8::cppgc::Visitor) {}
}

#[derive(Clone, Copy, Debug)]
pub struct FontAttributes {
    pub style: ComputedFontStyle,
    pub weight: ComputedFontWeight,
    pub width: ComputedFontWidth,
}

#[derive(Debug, Default)]
pub struct FontFaceSet {
    entries: LinkedHashMap<FontFaceId, Rc<FontFace>>,
}

impl FontFaceSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, font: Rc<FontFace>) {
        self.entries.insert(font.id, font);
    }

    pub fn remove(&mut self, id: FontFaceId) {
        self.entries.remove(&id);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn match_fonts(
        &self,
        family: &ComputedFamilyName,
        attrs: FontAttributes,
    ) -> Vec<Rc<FontFace>> {
        fn width_distance(
            ComputedFontWidth(desired): ComputedFontWidth,
            ComputedFontWidthRange(min, max): ComputedFontWidthRange,
        ) -> (u8, f32) {
            if desired > 1.0 {
                match (min > desired, max < desired) {
                    (false, false) => (0, 0.0),
                    (false, true) => (1, desired - max),
                    (true, false) => (0, min - desired),
                    (true, true) => unreachable!(),
                }
            } else {
                match (min > desired, max < desired) {
                    (false, false) => (0, 0.0),
                    (false, true) => (0, desired - max),
                    (true, false) => (1, min - desired),
                    (true, true) => unreachable!(),
                }
            }
        }

        fn style_distance(desired: ComputedFontStyle, range: ComputedFontStyleRange) -> (u8, f32) {
            match (desired, range) {
                (ComputedFontStyle::Normal, ComputedFontStyleRange::Normal)
                | (ComputedFontStyle::Italic, ComputedFontStyleRange::Italic) => {
                    return (0, 0.0);
                }
                (ComputedFontStyle::Normal, ComputedFontStyleRange::Italic) => {
                    return (3, 0.0);
                }
                _ => {}
            }
            let desired = match desired {
                ComputedFontStyle::Normal => 0.0,
                ComputedFontStyle::Italic => 11.0,
                ComputedFontStyle::Oblique(angle) => angle.deg,
            };
            let (min, max) = match range {
                ComputedFontStyleRange::Normal => (0.0, 0.0),
                ComputedFontStyleRange::Italic => (11.0, 11.0),
                ComputedFontStyleRange::Oblique(min, max) => (min.deg, max.deg),
            };
            if desired >= 11.0 {
                match (min > desired, max < desired) {
                    (false, false) => (1, 0.0),
                    (false, true) => (2, desired - max),
                    (true, false) => (1, min - desired),
                    (true, true) => unreachable!(),
                }
            } else if desired >= 0.0 {
                match (min > desired, max < desired) {
                    (false, false) => (1, 0.0),
                    (false, true) => (if max < 0.0 { 3 } else { 1 }, desired - max),
                    (true, false) => (2, min - desired),
                    (true, true) => unreachable!(),
                }
            } else if desired > -11.0 {
                match (min > desired, max < desired) {
                    (false, false) => (1, 0.0),
                    (false, true) => (2, desired - max),
                    (true, false) => (if max < 0.0 { 3 } else { 1 }, min - desired),
                    (true, true) => unreachable!(),
                }
            } else {
                match (min > desired, max < desired) {
                    (false, false) => (1, 0.0),
                    (false, true) => (1, desired - max),
                    (true, false) => (2, min - desired),
                    (true, true) => unreachable!(),
                }
            }
        }

        fn weight_distance(
            ComputedFontWeight(desired): ComputedFontWeight,
            ComputedFontWeightRange(min, max): ComputedFontWeightRange,
        ) -> (u8, f32) {
            if desired > 500.0 {
                match (min > desired, max < desired) {
                    (false, false) => (0, 0.0),
                    (false, true) => (1, desired - max),
                    (true, false) => (0, min - desired),
                    (true, true) => unreachable!(),
                }
            } else if desired >= 400.0 {
                match (min > desired, max < desired) {
                    (false, false) => (0, 0.0),
                    (false, true) => (1, desired - max),
                    (true, false) => (if min > 500.0 { 2 } else { 0 }, min - desired),
                    (true, true) => unreachable!(),
                }
            } else {
                match (min > desired, max < desired) {
                    (false, false) => (0, 0.0),
                    (false, true) => (0, desired - max),
                    (true, false) => (1, min - desired),
                    (true, true) => unreachable!(),
                }
            }
        }

        let family = match *family {
            ComputedFamilyName::Specific(ref v) => &v.name,
            ComputedFamilyName::Generic(_) => return vec![], // TODO actually select a family
        };
        let casefolded_family = UniCase::new(family).to_folded_case();
        let mut result = Vec::new();
        let mut min_distance = [(u8::MAX, f32::INFINITY); 3];
        for font in self.entries.values().rev() {
            let data = font.data.borrow();
            if casefolded_family != data.family.casefolded {
                continue;
            }
            let distance = [
                width_distance(attrs.width, data.width.computed),
                style_distance(attrs.style, data.style.computed),
                weight_distance(attrs.weight, data.weight.computed),
            ];
            if distance < min_distance {
                min_distance = distance;
                result.clear();
            }
            result.push(font.clone());
        }
        result
    }

    pub fn first_available_font(
        &self,
        family: &[ComputedFamilyName],
        attrs: FontAttributes,
    ) -> Option<Rc<FontFace>> {
        family.iter().find_map(|family| {
            self.match_fonts(family, attrs).into_iter().find(|font| {
                let data = font.data.borrow();
                matches!(
                    data.state,
                    FontFaceState::Loaded(_) if data.unicode_range.simplified.contains(' ' as u32),
                )
            })
        })
    }

    pub fn find_available_font_for_char(
        &self,
        c: char,
        family: &[ComputedFamilyName],
        attrs: FontAttributes,
    ) -> Option<Rc<FontFace>> {
        family.iter().find_map(|family| {
            self.match_fonts(family, attrs).into_iter().find(|font| {
                let data = font.data.borrow();
                matches!(
                    data.state,
                    FontFaceState::Loaded(ref font)
                        if data.unicode_range.simplified.contains(c as u32)
                            && font.get_nominal_glyph(c).is_some(),
                )
            })
        })
    }

    pub fn find_all_fonts_for_str(
        &self,
        s: &str,
        family: &[ComputedFamilyName],
        attrs: FontAttributes,
    ) -> Vec<Rc<FontFace>> {
        family
            .iter()
            .flat_map(move |family| {
                self.match_fonts(family, attrs).into_iter().filter(|font| {
                    let data = font.data.borrow();
                    s.chars()
                        .any(|c| data.unicode_range.simplified.contains(c as u32))
                })
            })
            .collect()
    }
}

// SAFETY: this type has no members.
unsafe impl GarbageCollected for Wrap<Rc<RefCell<FontFaceSet>>> {
    fn get_name(&self) -> &'static CStr {
        c"FontFaceSet"
    }

    fn trace(&self, _: &mut v8::cppgc::Visitor) {}
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    fn from_bidi_level(level: bidi::Level) -> Self {
        if level.is_rtl() { Self::Rtl } else { Self::Ltr }
    }

    fn to_bidi_level(self) -> bidi::Level {
        match self {
            Self::Ltr => bidi::LTR_LEVEL,
            Self::Rtl => bidi::RTL_LEVEL,
        }
    }

    fn to_harfbuzz(self) -> hb::Direction {
        match self {
            Self::Ltr => hb::Direction::Ltr,
            Self::Rtl => hb::Direction::Rtl,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum PhysicalAlignment {
    Left,
    Right,
    Center,
}

#[derive(Debug)]
struct TextPathBuilder {
    path: Path,
    transform: Transform2D<f32>,
}

impl hb::DrawFuncs for TextPathBuilder {
    fn move_to(&mut self, _: &hb::draw_funcs::DrawState, x: f32, y: f32) {
        let p = self.transform.transform_point(point2(x, y)).cast();
        self.path.move_to(p.x, p.y)
    }

    fn line_to(&mut self, _: &hb::draw_funcs::DrawState, x: f32, y: f32) {
        let p = self.transform.transform_point(point2(x, y)).cast();
        self.path.line_to(p.x, p.y)
    }

    fn quadratic_to(&mut self, _: &hb::draw_funcs::DrawState, cx: f32, cy: f32, x: f32, y: f32) {
        let c = self.transform.transform_point(point2(cx, cy)).cast();
        let p = self.transform.transform_point(point2(x, y)).cast();
        self.path.quad_to(c.x, c.y, p.x, p.y)
    }

    fn cubic_to(
        &mut self,
        _: &hb::draw_funcs::DrawState,
        c1x: f32,
        c1y: f32,
        c2x: f32,
        c2y: f32,
        x: f32,
        y: f32,
    ) {
        let c1 = self.transform.transform_point(point2(c1x, c1y)).cast();
        let c2 = self.transform.transform_point(point2(c2x, c2y)).cast();
        let p = self.transform.transform_point(point2(x, y)).cast();
        self.path.cubic_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y)
    }

    fn close_path(&mut self, _: &hb::draw_funcs::DrawState) {
        self.path.close()
    }
}

fn replace_ascii_whitespace(s: &str) -> Cow<'_, str> {
    fn should_replace(b: u8) -> bool {
        matches!(b, b'\t' | b'\n' | b'\r' | b'\x0c')
    }

    if s.bytes().any(should_replace) {
        let bytes = s
            .bytes()
            .map(|b| if should_replace(b) { b' ' } else { b })
            .collect();
        // SAFETY: `s` is valid UTF-8; the code above only modifies ASCII bytes, resulting in
        // still valid UTF-8.
        Cow::Owned(unsafe { String::from_utf8_unchecked(bytes) })
    } else {
        Cow::Borrowed(s)
    }
}

#[derive(Clone, Copy, Debug)]
struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub em_ascent: f32,
    pub em_descent: f32,
    pub hanging_baseline: f32,
    pub alphabetic_baseline: f32,
    pub ideographic_baseline: f32,
}

impl FontMetrics {
    pub fn new(
        font: &hb::Font,
        ascent_override: Option<f32>,
        descent_override: Option<f32>,
    ) -> Self {
        let unit = 1.0 / font.face().upem() as f32;
        let ascent = ascent_override.unwrap_or_else(|| match font.get_position(b"hasc") {
            Some(v) => v as f32 * unit,
            None => 0.8,
        });
        let descent = descent_override.unwrap_or_else(|| match font.get_position(b"hdsc") {
            Some(v) => v as f32 * unit,
            None => -0.2,
        });
        let os2_ascent = match font.get_position(b"Oasc") {
            Some(v) => v as f32 * unit,
            None => 0.8,
        };
        let os2_descent = match font.get_position(b"Odsc") {
            Some(v) => v as f32 * unit,
            None => -0.2,
        };
        let em_scale = 1.0 / (os2_ascent - os2_descent);
        let em_ascent = os2_ascent * em_scale;
        let em_descent = os2_descent * em_scale;
        let hanging_baseline =
            match font.get_baseline(b"hang", hb::Direction::Ltr, b"DFLT", b"dflt") {
                Some(v) => v as f32,
                None => ascent * 0.8,
            };
        let alphabetic_baseline =
            match font.get_baseline(b"romn", hb::Direction::Ltr, b"DFLT", b"dflt") {
                Some(v) => v as f32,
                None => 0.0,
            };
        let ideographic_baseline =
            match font.get_baseline(b"ideo", hb::Direction::Ltr, b"DFLT", b"dflt") {
                Some(v) => v as f32,
                None => descent,
            };
        Self {
            ascent,
            descent,
            em_ascent,
            em_descent,
            hanging_baseline,
            alphabetic_baseline,
            ideographic_baseline,
        }
    }

    pub fn empty() -> Self {
        Self {
            ascent: 0.0,
            descent: 0.0,
            em_ascent: 0.0,
            em_descent: 0.0,
            hanging_baseline: 0.0,
            alphabetic_baseline: 0.0,
            ideographic_baseline: 0.0,
        }
    }

    pub fn scale(self, scale: f32) -> Self {
        Self {
            ascent: self.ascent * scale,
            descent: self.descent * scale,
            em_ascent: self.em_ascent * scale,
            em_descent: self.em_descent * scale,
            hanging_baseline: self.hanging_baseline * scale,
            alphabetic_baseline: self.alphabetic_baseline * scale,
            ideographic_baseline: self.ideographic_baseline * scale,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextMetrics {
    pub width: f32,
    pub actual_bounding_box_left: f32,
    pub actual_bounding_box_right: f32,
    pub font_bounding_box_ascent: f32,
    pub font_bounding_box_descent: f32,
    pub actual_bounding_box_ascent: f32,
    pub actual_bounding_box_descent: f32,
    pub em_height_ascent: f32,
    pub em_height_descent: f32,
    pub hanging_baseline: f32,
    pub alphabetic_baseline: f32,
    pub ideographic_baseline: f32,
}

impl TextMetrics {
    pub fn empty() -> Self {
        Self {
            width: 0.0,
            actual_bounding_box_left: 0.0,
            actual_bounding_box_right: 0.0,
            font_bounding_box_ascent: 0.0,
            font_bounding_box_descent: 0.0,
            actual_bounding_box_ascent: 0.0,
            actual_bounding_box_descent: 0.0,
            em_height_ascent: 0.0,
            em_height_descent: 0.0,
            hanging_baseline: 0.0,
            alphabetic_baseline: 0.0,
            ideographic_baseline: 0.0,
        }
    }
}

#[derive(Debug)]
struct TextRun {
    range: Range<usize>,
    direction: Direction,
    font: Option<Rc<FontFace>>,
}

fn split_text_to_runs(
    fonts: &FontFaceSet,
    font_family: &[ComputedFamilyName],
    font_attrs: FontAttributes,
    base_direction: Direction,
    text: &str,
) -> Vec<TextRun> {
    let bidi_info = ParagraphBidiInfo::new(text, Some(base_direction.to_bidi_level()));
    if bidi_info.levels.is_empty() {
        return vec![];
    }
    let (levels, runs) = bidi_info.visual_runs(0..bidi_info.levels.len());
    runs.into_iter()
        .flat_map(|range| {
            let direction = Direction::from_bidi_level(levels[range.start]);
            let mut runs = Vec::new();
            let mut font_mapping = text[range.clone()].char_indices().map(|(i, c)| {
                (
                    range.start + i,
                    fonts.find_available_font_for_char(c, font_family, font_attrs),
                )
            });
            if let Some((mut start, mut font)) = font_mapping.next() {
                for (next, next_font) in font_mapping {
                    match (&font, &next_font) {
                        (None, None) => continue,
                        (Some(x), Some(y)) if x.id == y.id => continue,
                        _ => {}
                    }
                    runs.push(TextRun {
                        range: start..next,
                        direction,
                        font,
                    });
                    start = next;
                    font = next_font;
                }
                runs.push(TextRun {
                    range: start..range.end,
                    direction,
                    font,
                });
            }
            match direction {
                Direction::Ltr => {}
                Direction::Rtl => runs.reverse(),
            }
            runs
        })
        .collect()
}

pub fn prepare_text(
    fonts: &FontFaceSet,
    drawing_state: &DrawingState,
    text: &str,
    max_width: f32,
) -> (Path, TextMetrics) {
    if max_width <= 0.0 {
        return (Path::new(), TextMetrics::empty());
    }
    let text = replace_ascii_whitespace(text);
    let lang = drawing_state
        .lang
        .parse()
        .unwrap_or(hb::Language(ptr::null()));
    let script = hb::Script::from_iso15924_tag(hb::Tag(drawing_state.script));
    let font_size = drawing_state.font_size.0;
    let font_family = drawing_state.font_family.family_list.as_ref();
    let font_attrs = FontAttributes {
        style: drawing_state.font_style,
        weight: drawing_state.font_weight,
        width: drawing_state.font_stretch.modernize(),
    };
    let letter_spacing = drawing_state.letter_spacing.compute();
    let word_spacing = drawing_state.word_spacing.compute();
    let optimize_speed = matches!(
        drawing_state.text_rendering,
        CanvasTextRendering::OptimizeSpeed,
    );
    let direction = match drawing_state.direction {
        CanvasDirection::Ltr | CanvasDirection::Inherit => Direction::Ltr,
        CanvasDirection::Rtl => Direction::Rtl,
    };
    let physical_alignment = match (drawing_state.text_align, direction) {
        (CanvasTextAlign::Left, _)
        | (CanvasTextAlign::Start, Direction::Ltr)
        | (CanvasTextAlign::End, Direction::Rtl) => PhysicalAlignment::Left,
        (CanvasTextAlign::Right, _)
        | (CanvasTextAlign::End, Direction::Ltr)
        | (CanvasTextAlign::Start, Direction::Rtl) => PhysicalAlignment::Right,
        (CanvasTextAlign::Center, _) => PhysicalAlignment::Center,
    };
    let runs = split_text_to_runs(fonts, font_family, font_attrs, direction, &text);
    let mut path_builder = TextPathBuilder {
        path: Path::new(),
        transform: Transform2D::scale(0.0, 0.0),
    };
    let mut cursor = Point2D::zero();
    let mut bounds = Box2D::zero();
    let mut font_cache = HashMap::new();
    let mut text_has_nonzero_advance = false;
    runs.into_iter().fold(hb::UnicodeBuffer::new(), |buf, run| {
        let Some(font) = run.font else {
            return buf;
        };
        let Some((ref font, ref features)) = *font_cache.entry(font.id).or_insert_with(|| {
            let data = font.data.borrow();
            match data.state {
                FontFaceState::Loaded(ref font) => {
                    let mut font = hb::Font::create_sub_font(font.clone());
                    let face = font.face();
                    let mut features = HashMap::<hb::Tag, _>::new();
                    if optimize_speed {
                        features.insert(b"kern".into(), 0);
                        features.insert(b"liga".into(), 0);
                        features.insert(b"clig".into(), 0);
                        features.insert(b"calt".into(), 0);
                    }
                    if let Some(ref list) = data.feature_settings.feature_tag_value_list {
                        for entry in list.iter() {
                            features.insert((&entry.tag).into(), entry.value);
                        }
                    }
                    match drawing_state.font_variant_caps {
                        ComputedFontVariantCaps::Normal => {}
                        // TODO synthesize `small-caps` and `all-small-caps` if unsupported by font
                        ComputedFontVariantCaps::SmallCaps => {
                            features.insert(b"smcp".into(), 1);
                        }
                        ComputedFontVariantCaps::AllSmallCaps => {
                            features.insert(b"c2sc".into(), 1);
                            features.insert(b"smcp".into(), 1);
                        }
                        ComputedFontVariantCaps::PetiteCaps => {
                            features.insert(b"pcap".into(), 1);
                        }
                        ComputedFontVariantCaps::AllPetiteCaps => {
                            features.insert(b"c2pc".into(), 1);
                            features.insert(b"pcap".into(), 1);
                        }
                        ComputedFontVariantCaps::Unicase => {
                            features.insert(b"unic".into(), 1);
                        }
                        ComputedFontVariantCaps::TitlingCaps => {
                            features.insert(b"titl".into(), 1);
                        }
                    }
                    match drawing_state.font_kerning {
                        CanvasFontKerning::Auto => {}
                        CanvasFontKerning::Normal => {
                            features.insert(b"kern".into(), 1);
                        }
                        CanvasFontKerning::None => {
                            features.insert(b"kern".into(), 0);
                        }
                    }
                    if letter_spacing.px != 0.0 {
                        features.insert(b"liga".into(), 0);
                        features.insert(b"clig".into(), 0);
                        features.insert(b"dlig".into(), 0);
                        features.insert(b"hlig".into(), 0);
                        features.insert(b"calt".into(), 0);
                    }
                    let features = features
                        .into_iter()
                        .map(|(tag, value)| hb::Feature::new(tag, value, ..))
                        .collect::<Box<[_]>>();
                    let mut variations = HashMap::new();
                    let weight = drawing_state.font_weight.0;
                    if let Some(info) = face.find_variation_axis_info(b"wght") {
                        variations.insert(hb::Tag(info.0.tag), weight);
                    } else {
                        let embolden = (weight - data.weight.computed.1).max(0.0) * (1.0 / 14400.0);
                        font.set_synthetic_bold(embolden, embolden, false);
                    }
                    let width = drawing_state.font_stretch.modernize().0;
                    if let Some(info) = face.find_variation_axis_info(b"wdth") {
                        variations.insert(hb::Tag(info.0.tag), width * 100.0);
                    }
                    match drawing_state.font_style {
                        ComputedFontStyle::Normal => {}
                        ComputedFontStyle::Italic => {
                            if let Some(info) = face.find_variation_axis_info(b"ital") {
                                variations.insert(hb::Tag(info.0.tag), 1.0);
                            } else if data.style.computed == ComputedFontStyleRange::Normal {
                                font.set_synthetic_slant(0.25);
                            }
                        }
                        ComputedFontStyle::Oblique(angle) => {
                            if let Some(info) = face.find_variation_axis_info(b"slnt") {
                                variations.insert(hb::Tag(info.0.tag), -angle.deg);
                            } else if data.style.computed == ComputedFontStyleRange::Normal {
                                font.set_synthetic_slant(angle.radians().tan());
                            }
                        }
                    }
                    if let Some(ref list) = data.variation_settings.variation_value_list {
                        for entry in list.iter() {
                            if let Some(info) = face.find_variation_axis_info(&entry.tag) {
                                variations.insert(hb::Tag(info.0.tag), entry.value);
                            }
                        }
                    }
                    let variations = variations
                        .into_iter()
                        .map(|(tag, value)| hb::Variation::new(tag, value))
                        .collect::<Box<[_]>>();
                    font.set_variations(&variations);
                    Some((font, features))
                }
                _ => None,
            }
        }) else {
            return buf;
        };
        let scale = font_size.px / font.face().upem() as f32;
        let buf = buf
            .add_str_item(&text, &text[run.range])
            .set_direction(run.direction.to_harfbuzz())
            .set_script(script)
            .set_language(lang);
        let buf = hb::shape(font, buf, features);
        let positions = buf.get_glyph_positions();
        let infos = buf.get_glyph_infos();
        let mut cluster_has_nonzero_advance = false;
        for (index, (&position, &info)) in positions.iter().zip(infos).enumerate() {
            let glyph = info.codepoint;
            let advance = vec2(position.x_advance, position.y_advance).cast() * scale;
            let offset = vec2(position.x_offset, position.y_offset).cast() * scale;
            let pos = cursor + offset;
            path_builder.transform = Transform2D::new(scale, 0.0, 0.0, scale, pos.x, pos.y);
            font.draw_glyph(glyph, &mut path_builder);
            if let Some(mut extents) = font.get_glyph_extents(glyph) {
                extents.y_bearing += extents.height;
                extents.height = -extents.height;
                bounds = bounds.union(&Box2D::from_origin_and_size(
                    pos + vec2(extents.x_bearing, extents.y_bearing).cast() * scale,
                    size2(extents.width, extents.height).cast() * scale,
                ));
            }
            cursor += advance;
            if advance != Vector2D::zero() {
                cluster_has_nonzero_advance = true;
            }
            if (index + 1 >= infos.len() || info.cluster != infos[index + 1].cluster)
                && cluster_has_nonzero_advance
            {
                cluster_has_nonzero_advance = false;
                text_has_nonzero_advance = true;
                let c = text[info.cluster as usize..].chars().next().unwrap();
                cursor.x += letter_spacing.px;
                if c == ' ' || c == '\u{a0}' {
                    cursor.x += word_spacing.px;
                }
            }
        }
        buf.clear()
    });
    if text_has_nonzero_advance {
        cursor.x -= letter_spacing.px;
    }
    let width = cursor.x;
    let compression = (max_width / width).min(1.0);
    let width = width * compression;
    let bounds = bounds.scale(compression, 1.0);
    let font_metrics = match fonts.first_available_font(font_family, font_attrs) {
        Some(ref font) => {
            let data = font.data.borrow();
            match data.state {
                FontFaceState::Loaded(ref font) => {
                    let ascent_override = match data.ascent_override.0 {
                        SpecifiedMetricsOverrideValue::Normal => None,
                        SpecifiedMetricsOverrideValue::Percentage(v) => Some(v),
                    };
                    let descent_override = match data.descent_override.0 {
                        SpecifiedMetricsOverrideValue::Normal => None,
                        SpecifiedMetricsOverrideValue::Percentage(v) => Some(v),
                    };
                    FontMetrics::new(font, ascent_override, descent_override).scale(font_size.px)
                }
                _ => FontMetrics::empty(),
            }
        }
        None => FontMetrics::empty(),
    };
    let anchor_x = match physical_alignment {
        PhysicalAlignment::Left => 0.0,
        PhysicalAlignment::Right => width,
        PhysicalAlignment::Center => width * 0.5,
    };
    let anchor_y = match drawing_state.text_baseline {
        CanvasTextBaseline::Top => font_metrics.em_ascent,
        CanvasTextBaseline::Hanging => font_metrics.hanging_baseline,
        CanvasTextBaseline::Middle => (font_metrics.em_ascent + font_metrics.em_descent) * 0.5,
        CanvasTextBaseline::Alphabetic => font_metrics.alphabetic_baseline,
        CanvasTextBaseline::Ideographic => font_metrics.ideographic_baseline,
        CanvasTextBaseline::Bottom => font_metrics.em_descent,
    };
    let path = path_builder
        .path
        .transform(&Transform2D::new(compression, 0.0, 0.0, 1.0, -anchor_x, -anchor_y).cast());
    let text_metrics = TextMetrics {
        width,
        actual_bounding_box_left: anchor_x - bounds.min.x,
        actual_bounding_box_right: bounds.max.x - anchor_x,
        font_bounding_box_ascent: font_metrics.ascent - anchor_y,
        font_bounding_box_descent: anchor_y - font_metrics.descent,
        actual_bounding_box_ascent: bounds.max.y - anchor_y,
        actual_bounding_box_descent: anchor_y - bounds.min.y,
        em_height_ascent: font_metrics.em_ascent - anchor_y,
        em_height_descent: anchor_y - font_metrics.em_descent,
        hanging_baseline: font_metrics.hanging_baseline - anchor_y,
        alphabetic_baseline: font_metrics.alphabetic_baseline - anchor_y,
        ideographic_baseline: font_metrics.ideographic_baseline - anchor_y,
    };
    (path, text_metrics)
}

fn parse_source_or_throw(css: &str) -> Result<SpecifiedFontSources, Canvas2DError> {
    SpecifiedFontSources::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontSource,
        details: css::SyntaxError::from(e),
    })
}

fn parse_family_or_throw(css: &str) -> Result<SpecifiedSpecificFamily, Canvas2DError> {
    SpecifiedSpecificFamily::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontFamilyName,
        details: css::SyntaxError::from(e),
    })
}

fn parse_style_or_throw(css: &str) -> Result<SpecifiedFontStyleRange, Canvas2DError> {
    SpecifiedFontStyleRange::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontStyleRange,
        details: css::SyntaxError::from(e),
    })
}

fn parse_weight_or_throw(css: &str) -> Result<SpecifiedFontWeightRange, Canvas2DError> {
    SpecifiedFontWeightRange::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontWeightRange,
        details: css::SyntaxError::from(e),
    })
}

fn parse_stretch_or_throw(css: &str) -> Result<SpecifiedFontWidthRange, Canvas2DError> {
    SpecifiedFontWidthRange::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontWidthRange,
        details: css::SyntaxError::from(e),
    })
}

fn parse_unicode_range_or_throw(css: &str) -> Result<SpecifiedUnicodeRange, Canvas2DError> {
    SpecifiedUnicodeRange::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::UnicodeRange,
        details: css::SyntaxError::from(e),
    })
}

fn parse_feature_settings_or_throw(
    css: &str,
) -> Result<SpecifiedFontFeatureSettings, Canvas2DError> {
    SpecifiedFontFeatureSettings::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontFeatureSettings,
        details: css::SyntaxError::from(e),
    })
}

fn parse_variation_settings_or_throw(
    css: &str,
) -> Result<SpecifiedFontVariationSettings, Canvas2DError> {
    SpecifiedFontVariationSettings::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontVariationSettings,
        details: css::SyntaxError::from(e),
    })
}

fn parse_display_or_throw(css: &str) -> Result<SpecifiedFontDisplay, Canvas2DError> {
    SpecifiedFontDisplay::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::FontDisplayPolicy,
        details: css::SyntaxError::from(e),
    })
}

fn parse_ascent_override_or_throw(css: &str) -> Result<SpecifiedMetricsOverride, Canvas2DError> {
    SpecifiedMetricsOverride::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::AscentOverride,
        details: css::SyntaxError::from(e),
    })
}

fn parse_descent_override_or_throw(css: &str) -> Result<SpecifiedMetricsOverride, Canvas2DError> {
    SpecifiedMetricsOverride::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::DescentOverride,
        details: css::SyntaxError::from(e),
    })
}

fn parse_line_gap_override_or_throw(css: &str) -> Result<SpecifiedMetricsOverride, Canvas2DError> {
    SpecifiedMetricsOverride::from_css_string(css).map_err(|e| Canvas2DError::ParseCss {
        css: css.to_owned(),
        kind: css::ValueKind::LineGapOverride,
        details: css::SyntaxError::from(e),
    })
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_select_source(
    #[string] source: &str,
) -> Result<String, Canvas2DError> {
    for source in parse_source_or_throw(source)?.font_source_list.iter() {
        match *source {
            SpecifiedFontSource::Url(ref url) => return Ok((**url).to_owned()),
            SpecifiedFontSource::Local(_) => {}
        }
    }
    // TODO implement local font fallback
    Err(Canvas2DError::UnsupportedLocalFontFallback)
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_font_face_new(
    #[string] family: String,
    #[string] style: String,
    #[string] weight: String,
    #[string] stretch: String,
    #[string] unicode_range: String,
    #[string] feature_settings: String,
    #[string] variation_settings: String,
    #[string] display: String,
    #[string] ascent_override: String,
    #[string] descent_override: String,
    #[string] line_gap_override: String,
) -> Result<Wrap<Rc<FontFace>>, Canvas2DError> {
    Ok(Wrap::new(Rc::new(FontFace::new(FontFaceData::new(
        parse_family_or_throw(&family)?,
        parse_style_or_throw(&style)?,
        parse_weight_or_throw(&weight)?,
        parse_stretch_or_throw(&stretch)?,
        parse_unicode_range_or_throw(&unicode_range)?,
        parse_feature_settings_or_throw(&feature_settings)?,
        parse_variation_settings_or_throw(&variation_settings)?,
        parse_display_or_throw(&display)?,
        parse_ascent_override_or_throw(&ascent_override)?,
        parse_descent_override_or_throw(&descent_override)?,
        parse_line_gap_override_or_throw(&line_gap_override)?,
        FontFaceState::Unloaded,
    )))))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_font_face_errored() -> Wrap<Rc<FontFace>> {
    Wrap::new(Rc::new(FontFace::new(FontFaceData::new(
        SpecifiedSpecificFamily { name: "".into() },
        SpecifiedFontStyleRange::default(),
        SpecifiedFontWeightRange::default(),
        SpecifiedFontWidthRange::default(),
        SpecifiedUnicodeRange::default(),
        SpecifiedFontFeatureSettings::default(),
        SpecifiedFontVariationSettings::default(),
        SpecifiedFontDisplay::default(),
        SpecifiedMetricsOverride::default(),
        SpecifiedMetricsOverride::default(),
        SpecifiedMetricsOverride::default(),
        FontFaceState::Errored,
    ))))
}

#[op2(fast)]
#[bigint]
pub fn op_canvas_2d_font_face_id(#[cppgc] this: &Wrap<Rc<FontFace>>) -> u64 {
    this.id().as_u64()
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_family(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.family().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_family(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_family_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_family(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_style(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.style().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_style(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_style_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_style(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_weight(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.weight().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_weight(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_weight_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_weight(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_stretch(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.width().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_stretch(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_stretch_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_width(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_unicode_range(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.unicode_range().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_unicode_range(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_unicode_range_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_unicode_range(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_feature_settings(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.feature_settings().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_feature_settings(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_feature_settings_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_feature_settings(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_variation_settings(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.variation_settings().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_variation_settings(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_variation_settings_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_variation_settings(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_display(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.display().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_display(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_display_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_display(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_ascent_override(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.ascent_override().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_ascent_override(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_ascent_override_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_ascent_override(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_descent_override(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.descent_override().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_descent_override(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_descent_override_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_descent_override(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_line_gap_override(#[cppgc] this: &Wrap<Rc<FontFace>>) -> String {
    let data = this.data().borrow();
    data.line_gap_override().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_line_gap_override(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[string] value: &str,
) -> Result<(), Canvas2DError> {
    let value = parse_line_gap_override_or_throw(value)?;
    let mut data = this.data().borrow_mut();
    data.set_line_gap_override(value);
    Ok(())
}

#[op2]
pub fn op_canvas_2d_font_face_load(
    #[cppgc] this: &Wrap<Rc<FontFace>>,
    #[anybuffer] source: &[u8],
    from_url: bool,
) -> Result<(), Canvas2DError> {
    let mut data = this.data().borrow_mut();
    data.load(source, from_url)
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_insert(
    #[cppgc] this: &Wrap<Rc<RefCell<FontFaceSet>>>,
    #[cppgc] font: &Wrap<Rc<FontFace>>,
) {
    let mut this = this.borrow_mut();
    this.insert((*font).clone())
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_remove(
    #[cppgc] this: &Wrap<Rc<RefCell<FontFaceSet>>>,
    #[cppgc] font: &Wrap<Rc<FontFace>>,
) {
    let mut this = this.borrow_mut();
    this.remove(font.id())
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_clear(#[cppgc] this: &Wrap<Rc<RefCell<FontFaceSet>>>) {
    let mut this = this.borrow_mut();
    this.clear()
}

#[op2]
pub fn op_canvas_2d_font_face_set_match<'a>(
    scope: &mut v8::HandleScope<'a>,
    #[cppgc] this: &Wrap<Rc<RefCell<FontFaceSet>>>,
    #[string] font: &str,
    #[string] text: &str,
) -> Result<v8::Local<'a, v8::Set>, Canvas2DError> {
    let this = this.borrow();
    let font = ComputedFont::from_css_string(font).map_err(|e| Canvas2DError::ParseCss {
        css: font.to_owned(),
        kind: css::ValueKind::Font,
        details: css::SyntaxError::from(e),
    })?;
    let family = font.family.family_list.as_ref();
    let attrs = FontAttributes {
        style: font.style,
        weight: font.weight,
        width: font.stretch.modernize(),
    };
    let set = v8::Set::new(scope);
    for font in this.find_all_fonts_for_str(text, family, attrs) {
        let value = v8::BigInt::new_from_u64(scope, font.id().as_u64()).into();
        set.add(scope, value);
    }
    Ok(set)
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_font_source(state: &OpState) -> Wrap<Rc<RefCell<FontFaceSet>>> {
    Wrap::new(state.borrow::<Rc<RefCell<FontFaceSet>>>().clone())
}

pub fn init(state: &mut OpState) {
    state.put(Rc::new(RefCell::new(FontFaceSet::new())));
}
