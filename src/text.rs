use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::num::NonZeroU64;
use std::ops::Range;
use std::process;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use cssparser::ToCss as _;
use deno_core::error::custom_error;
use deno_core::{anyhow, op2, v8, OpState};
use euclid::default::{Box2D, Point2D, Transform2D, Vector2D};
use euclid::{point2, size2, vec2};
use harfbuzz_rs as hb;
use hashlink::LinkedHashMap;
use unicase::UniCase;
use unicode_bidi::{self as bidi, ParagraphBidiInfo};

use super::css::font::{
    font_face, ComputedFamilyName, ComputedFontFamily, ComputedFontStyle, ComputedFontVariantCaps,
    ComputedFontWeight, ComputedFontWidth,
};
use super::css::{FromCss as _, UnicodeRangeSet};
use super::gc::{borrow_v8, into_v8};
use super::harfbuzz_ext::FontExt as _;
use super::path::Path;
use super::state::{
    CanvasDirection, CanvasFontKerning, CanvasTextAlign, CanvasTextBaseline, CanvasTextRendering,
    DrawingState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontFaceId(NonZeroU64);

impl FontFaceId {
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self(
            NonZeroU64::new(NEXT.fetch_add(1, Ordering::Relaxed))
                .unwrap_or_else(|| process::abort()),
        )
    }
}

#[derive(Debug)]
pub struct FontFaceFamily {
    specified: font_face::SpecifiedFontFamily,
    casefolded: String,
}

impl FontFaceFamily {
    pub fn new(specified: font_face::SpecifiedFontFamily) -> Self {
        let casefolded = UniCase::new(specified.name.as_ref()).to_folded_case();
        Self {
            specified,
            casefolded,
        }
    }
}

#[derive(Debug)]
pub struct FontFaceStyle {
    specified: font_face::SpecifiedFontStyle,
    computed: font_face::ComputedFontStyle,
}

impl FontFaceStyle {
    pub fn new(specified: font_face::SpecifiedFontStyle) -> Self {
        let computed = specified.compute();
        Self {
            specified,
            computed,
        }
    }
}

#[derive(Debug)]
pub struct FontFaceWeight {
    specified: font_face::SpecifiedFontWeight,
    computed: font_face::ComputedFontWeight,
}

impl FontFaceWeight {
    pub fn new(specified: font_face::SpecifiedFontWeight) -> Self {
        let computed = specified.compute();
        Self {
            specified,
            computed,
        }
    }
}

#[derive(Debug)]
pub struct FontFaceWidth {
    specified: font_face::SpecifiedFontWidth,
    computed: font_face::ComputedFontWidth,
}

impl FontFaceWidth {
    pub fn new(specified: font_face::SpecifiedFontWidth) -> Self {
        let computed = specified.compute();
        Self {
            specified,
            computed,
        }
    }
}

#[derive(Debug)]
pub struct FontFaceUnicodeRange {
    specified: font_face::SpecifiedUnicodeRange,
    simplified: UnicodeRangeSet,
}

impl FontFaceUnicodeRange {
    pub fn new(specified: font_face::SpecifiedUnicodeRange) -> Self {
        let simplified = UnicodeRangeSet::new(specified.range_list.iter().cloned());
        Self {
            specified,
            simplified,
        }
    }
}

#[derive(Debug)]
pub enum FontFaceState {
    Unloaded,
    Loaded(hb::Shared<hb::Font<'static>>),
    Errored,
}

#[derive(Debug)]
pub struct FontFaceData {
    family: FontFaceFamily,
    style: FontFaceStyle,
    weight: FontFaceWeight,
    width: FontFaceWidth,
    unicode_range: FontFaceUnicodeRange,
    state: FontFaceState,
}

impl FontFaceData {
    pub fn new(
        family: font_face::SpecifiedFontFamily,
        style: font_face::SpecifiedFontStyle,
        weight: font_face::SpecifiedFontWeight,
        width: font_face::SpecifiedFontWidth,
        unicode_range: font_face::SpecifiedUnicodeRange,
        state: FontFaceState,
    ) -> Self {
        Self {
            family: FontFaceFamily::new(family),
            style: FontFaceStyle::new(style),
            weight: FontFaceWeight::new(weight),
            width: FontFaceWidth::new(width),
            unicode_range: FontFaceUnicodeRange::new(unicode_range),
            state,
        }
    }
}

#[derive(Debug)]
pub struct FontFace {
    id: FontFaceId,
    data: Rc<RefCell<FontFaceData>>,
}

impl FontFace {
    pub fn new(data: FontFaceData) -> Self {
        Self {
            id: FontFaceId::new(),
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn id(&self) -> FontFaceId {
        self.id
    }

    pub fn family(&self) -> font_face::SpecifiedFontFamily {
        let data = self.data.borrow();
        data.family.specified.clone()
    }

    pub fn set_family(&self, value: font_face::SpecifiedFontFamily) {
        let mut data = self.data.borrow_mut();
        data.family = FontFaceFamily::new(value);
    }

    pub fn style(&self) -> font_face::SpecifiedFontStyle {
        let data = self.data.borrow();
        data.style.specified
    }

    pub fn set_style(&self, value: font_face::SpecifiedFontStyle) {
        let mut data = self.data.borrow_mut();
        data.style = FontFaceStyle::new(value);
    }

    pub fn weight(&self) -> font_face::SpecifiedFontWeight {
        let data = self.data.borrow();
        data.weight.specified
    }

    pub fn set_weight(&self, value: font_face::SpecifiedFontWeight) {
        let mut data = self.data.borrow_mut();
        data.weight = FontFaceWeight::new(value);
    }

    pub fn width(&self) -> font_face::SpecifiedFontWidth {
        let data = self.data.borrow();
        data.width.specified
    }

    pub fn set_width(&self, value: font_face::SpecifiedFontWidth) {
        let mut data = self.data.borrow_mut();
        data.width = FontFaceWidth::new(value);
    }

    pub fn unicode_range(&self) -> font_face::SpecifiedUnicodeRange {
        let data = self.data.borrow();
        data.unicode_range.specified.clone()
    }

    pub fn set_unicode_range(&self, value: font_face::SpecifiedUnicodeRange) {
        let mut data = self.data.borrow_mut();
        data.unicode_range = FontFaceUnicodeRange::new(value);
    }

    pub fn load(&self, blob: Vec<u8>) -> anyhow::Result<()> {
        let mut data = self.data.borrow_mut();
        match data.state {
            FontFaceState::Unloaded => {
                // TODO validate blob
                data.state = FontFaceState::Loaded(hb::Font::new(hb::Face::new(blob, 0)).into());
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FontAttributes {
    pub style: ComputedFontStyle,
    pub weight: ComputedFontWeight,
    pub width: ComputedFontWidth,
}

#[derive(Debug, Default)]
pub struct FontFaceSet {
    entries: LinkedHashMap<FontFaceId, Rc<RefCell<FontFaceData>>>,
}

impl FontFaceSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, font: &FontFace) {
        self.entries.insert(font.id, font.data.clone());
    }

    pub fn remove(&mut self, id: FontFaceId) {
        self.entries.remove(&id);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    fn resolve_family_name(family: &ComputedFamilyName) -> &str {
        match *family {
            ComputedFamilyName::Specific(ref v) => &v.name,
            ComputedFamilyName::Generic(_) => "Arial", // TODO actually select a family
        }
    }

    pub fn match_(&self, family: &ComputedFamilyName, attrs: FontAttributes) -> Vec<FontFace> {
        fn width_distance(
            ComputedFontWidth(desired): ComputedFontWidth,
            font_face::ComputedFontWidth(min, max): font_face::ComputedFontWidth,
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

        fn style_distance(
            desired: ComputedFontStyle,
            range: font_face::ComputedFontStyle,
        ) -> (u8, f32) {
            match (desired, range) {
                (ComputedFontStyle::Normal, font_face::ComputedFontStyle::Normal)
                | (ComputedFontStyle::Italic, font_face::ComputedFontStyle::Italic) => {
                    return (0, 0.0);
                }
                (ComputedFontStyle::Normal, font_face::ComputedFontStyle::Italic) => {
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
                font_face::ComputedFontStyle::Normal => (0.0, 0.0),
                font_face::ComputedFontStyle::Italic => (11.0, 11.0),
                font_face::ComputedFontStyle::Oblique(min, max) => (min.deg, max.deg),
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
            font_face::ComputedFontWeight(min, max): font_face::ComputedFontWeight,
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

        let family = UniCase::new(Self::resolve_family_name(family)).to_folded_case();
        let mut result = Vec::new();
        let mut min_distance = [(u8::MAX, f32::INFINITY); 3];
        for (&id, data_rc) in self.entries.iter().rev() {
            let data = data_rc.borrow();
            if family != data.family.casefolded {
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
            result.push(FontFace {
                id,
                data: data_rc.clone(),
            });
        }
        result
    }

    pub fn first_available_font(
        &self,
        family: &ComputedFontFamily,
        attrs: FontAttributes,
    ) -> Option<FontFace> {
        family.family_list.iter().find_map(|family| {
            self.match_(family, attrs).into_iter().find(|font| {
                let data = font.data.borrow();
                data.unicode_range.simplified.contains(' ' as u32)
            })
        })
    }

    pub fn loaded_font_for_char(
        &self,
        c: char,
        family: &ComputedFontFamily,
        attrs: FontAttributes,
    ) -> Option<FontFace> {
        family.family_list.iter().find_map(|family| {
            self.match_(family, attrs).into_iter().find(|font| {
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

    pub fn fonts_for_str(
        &self,
        s: &str,
        family: &ComputedFontFamily,
        attrs: FontAttributes,
    ) -> Vec<FontFace> {
        family
            .family_list
            .iter()
            .flat_map(|family| {
                self.match_(family, attrs).into_iter().filter(|font| {
                    let data = font.data.borrow();
                    s.chars()
                        .any(|c| data.unicode_range.simplified.contains(c as u32))
                })
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    fn from_bidi_level(level: bidi::Level) -> Self {
        if level.is_rtl() {
            Self::Rtl
        } else {
            Self::Ltr
        }
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

fn replace_ascii_whitespace(s: &str) -> Cow<str> {
    fn should_replace(b: u8) -> bool {
        matches!(b, b'\t' | b'\n' | b'\r' | b'\x0c')
    }

    if s.bytes().any(should_replace) {
        let bytes = s
            .bytes()
            .map(|b| if should_replace(b) { b' ' } else { b })
            .collect();
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
    pub fn new(font: &hb::Font) -> Self {
        let upem = font.face().upem() as f32;
        let ascent = match font.get_position(b"hasc") {
            Some(v) => v as f32,
            None => upem * 0.8,
        };
        let descent = match font.get_position(b"hdsc") {
            Some(v) => v as f32,
            None => upem * -0.2,
        };
        let os2_ascent = match font.get_position(b"Oasc") {
            Some(v) => v as f32,
            None => upem * 0.8,
        };
        let os2_descent = match font.get_position(b"Odsc") {
            Some(v) => v as f32,
            None => upem * -0.2,
        };
        let em_scale = upem / (os2_ascent - os2_descent);
        let em_ascent = os2_ascent * em_scale;
        let em_descent = os2_descent * em_scale;
        let hanging_baseline = match font.get_generic_baseline(b"hang") {
            Some(v) => v as f32,
            None => ascent * 0.8,
        };
        let alphabetic_baseline = match font.get_generic_baseline(b"romn") {
            Some(v) => v as f32,
            None => 0.0,
        };
        let ideographic_baseline = match font.get_generic_baseline(b"ideo") {
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
    font: Option<FontFace>,
}

fn split_text_to_runs(
    fonts: &FontFaceSet,
    font_family: &ComputedFontFamily,
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
                    fonts.loaded_font_for_char(c, font_family, font_attrs),
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
    let font_size = drawing_state.font_size.0;
    let font_family = &drawing_state.font_family;
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
    let mut features = Vec::new();
    match drawing_state.font_variant_caps {
        ComputedFontVariantCaps::Normal => {}
        // TODO synthesize `small-caps` and `all-small-caps` if unsupported by font
        ComputedFontVariantCaps::SmallCaps => {
            features.push(hb::Feature::new(b"smcp", 1, ..));
        }
        ComputedFontVariantCaps::AllSmallCaps => {
            features.extend([
                hb::Feature::new(b"c2sc", 1, ..),
                hb::Feature::new(b"smcp", 1, ..),
            ]);
        }
        ComputedFontVariantCaps::PetiteCaps => {
            features.push(hb::Feature::new(b"pcap", 1, ..));
        }
        ComputedFontVariantCaps::AllPetiteCaps => {
            features.extend([
                hb::Feature::new(b"c2pc", 1, ..),
                hb::Feature::new(b"pcap", 1, ..),
            ]);
        }
        ComputedFontVariantCaps::Unicase => {
            features.push(hb::Feature::new(b"unic", 1, ..));
        }
        ComputedFontVariantCaps::TitlingCaps => {
            features.push(hb::Feature::new(b"titl", 1, ..));
        }
    }
    if match drawing_state.font_kerning {
        CanvasFontKerning::Auto => optimize_speed,
        CanvasFontKerning::Normal => false,
        CanvasFontKerning::None => true,
    } {
        features.push(hb::Feature::new(b"kern", 0, ..));
    }
    if letter_spacing.px != 0.0 || optimize_speed {
        features.extend([
            hb::Feature::new(b"liga", 0, ..),
            hb::Feature::new(b"clig", 0, ..),
            hb::Feature::new(b"calt", 0, ..),
        ])
    }
    // TODO font weight
    // TODO font stretch
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
        let font = font_cache.entry(font.id).or_insert_with(|| {
            let data = font.data.borrow();
            match data.state {
                FontFaceState::Loaded(ref font) => {
                    let mut font = hb::Font::create_sub_font(font.clone());
                    if matches!(data.style.computed, font_face::ComputedFontStyle::Normal) {
                        match drawing_state.font_style {
                            ComputedFontStyle::Normal => {}
                            ComputedFontStyle::Italic => {
                                // TODO use `ital` feature if supported
                                font.set_synthetic_slant(0.25);
                            }
                            ComputedFontStyle::Oblique(angle) => {
                                // TODO use `slnt` feature if supported
                                font.set_synthetic_slant(angle.radians().tan())
                            }
                        }
                    }
                    font
                }
                _ => hb::Font::empty(),
            }
        });
        let scale = font_size.px / font.face().upem() as f32;
        let buf = buf
            .add_str_item(&text, &text[run.range])
            .set_direction(run.direction.to_harfbuzz());
        let buf = hb::shape(font, buf, &features);
        let positions = buf.get_glyph_positions();
        let infos = buf.get_glyph_infos();
        let mut cluster_has_nonzero_advance = false;
        for (index, (&position, &info)) in positions.iter().zip(infos).enumerate() {
            let glyph = info.codepoint;
            let advance = vec2(position.x_advance, position.y_advance).cast() * scale;
            let offset = vec2(position.x_offset, position.y_offset).cast() * scale;
            let pos = cursor + offset;
            path_builder.transform = Transform2D::new(scale, 0.0, 0.0, scale, pos.x, pos.y);
            #[allow(clippy::unnecessary_mut_passed)]
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
    let bounds = bounds * compression;
    let font_metrics = match fonts.first_available_font(font_family, font_attrs) {
        Some(ref font) => {
            let data = font.data.borrow();
            match data.state {
                FontFaceState::Loaded(ref font) => {
                    let scale = font_size.px / font.face().upem() as f32;
                    FontMetrics::new(font).scale(scale * compression)
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
    let path = path_builder.path.transform(
        &Transform2D::new(compression, 0.0, 0.0, compression, -anchor_x, -anchor_y).cast(),
    );
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

fn parse_source_or_throw(css: &str) -> anyhow::Result<font_face::SpecifiedSource> {
    font_face::SpecifiedSource::from_css_string(css).map_err(|err| {
        custom_error(
            "DOMExceptionSyntaxError",
            format!(
                "Invalid font source '{css}': {} at {}:{}",
                err.kind,
                err.location.line + 1,
                err.location.column,
            ),
        )
    })
}

fn parse_family_or_throw(css: &str) -> anyhow::Result<font_face::SpecifiedFontFamily> {
    font_face::SpecifiedFontFamily::from_css_string(css).map_err(|err| {
        custom_error(
            "DOMExceptionSyntaxError",
            format!(
                "Invalid family name '{css}': {} at {}:{}",
                err.kind,
                err.location.line + 1,
                err.location.column,
            ),
        )
    })
}

fn parse_style_or_throw(css: &str) -> anyhow::Result<font_face::SpecifiedFontStyle> {
    font_face::SpecifiedFontStyle::from_css_string(css).map_err(|err| {
        custom_error(
            "DOMExceptionSyntaxError",
            format!(
                "Invalid font style range '{css}': {} at {}:{}",
                err.kind,
                err.location.line + 1,
                err.location.column,
            ),
        )
    })
}

fn parse_weight_or_throw(css: &str) -> anyhow::Result<font_face::SpecifiedFontWeight> {
    font_face::SpecifiedFontWeight::from_css_string(css).map_err(|err| {
        custom_error(
            "DOMExceptionSyntaxError",
            format!(
                "Invalid font weight range '{css}': {} at {}:{}",
                err.kind,
                err.location.line + 1,
                err.location.column,
            ),
        )
    })
}

fn parse_stretch_or_throw(css: &str) -> anyhow::Result<font_face::SpecifiedFontWidth> {
    font_face::SpecifiedFontWidth::from_css_string(css).map_err(|err| {
        custom_error(
            "DOMExceptionSyntaxError",
            format!(
                "Invalid font width range '{css}': {} at {}:{}",
                err.kind,
                err.location.line + 1,
                err.location.column,
            ),
        )
    })
}

fn parse_unicode_range_or_throw(css: &str) -> anyhow::Result<font_face::SpecifiedUnicodeRange> {
    font_face::SpecifiedUnicodeRange::from_css_string(css).map_err(|err| {
        custom_error(
            "DOMExceptionSyntaxError",
            format!(
                "Invalid unicode range '{css}': {} at {}:{}",
                err.kind,
                err.location.line + 1,
                err.location.column,
            ),
        )
    })
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_select_source(#[string] source: &str) -> anyhow::Result<String> {
    for source in parse_source_or_throw(source)?.font_source_list.iter() {
        match *source {
            font_face::SpecifiedFontSource::Url(ref url) => return Ok((**url).to_owned()),
            font_face::SpecifiedFontSource::Local(_) => {}
        }
    }
    // TODO implement local font fallback
    Err(custom_error(
        "DOMExceptionSyntaxError",
        "Local font fallback is not supported",
    ))
}

#[op2]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_font_face_new<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
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
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let result = FontFace::new(FontFaceData::new(
        parse_family_or_throw(&family)?,
        parse_style_or_throw(&style)?,
        parse_weight_or_throw(&weight)?,
        parse_stretch_or_throw(&stretch)?,
        parse_unicode_range_or_throw(&unicode_range)?,
        // TODO parse `feature_settings`
        // TODO parse `variation_settings`
        // TODO parse `display`
        // TODO parse `ascent_override`
        // TODO parse `descent_override`
        // TODO parse `line_gap_override`
        FontFaceState::Unloaded,
    ));
    Ok(into_v8(state, scope, result))
}

#[op2]
pub fn op_canvas_2d_font_face_errored<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
) -> v8::Local<'a, v8::External> {
    let result = FontFace::new(FontFaceData::new(
        font_face::SpecifiedFontFamily { name: "".into() },
        font_face::SpecifiedFontStyle::default(),
        font_face::SpecifiedFontWeight::default(),
        font_face::SpecifiedFontWidth::default(),
        font_face::SpecifiedUnicodeRange::default(),
        FontFaceState::Errored,
    ));
    into_v8(state, scope, result)
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_family(state: &OpState, this: *const c_void) -> String {
    let this = borrow_v8::<FontFace>(state, this);
    this.family().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_family(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    let value = parse_family_or_throw(value)?;
    this.set_family(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_style(state: &OpState, this: *const c_void) -> String {
    let this = borrow_v8::<FontFace>(state, this);
    this.style().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_style(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    let value = parse_style_or_throw(value)?;
    this.set_style(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_weight(state: &OpState, this: *const c_void) -> String {
    let this = borrow_v8::<FontFace>(state, this);
    this.weight().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_weight(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    let value = parse_weight_or_throw(value)?;
    this.set_weight(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_stretch(state: &OpState, this: *const c_void) -> String {
    let this = borrow_v8::<FontFace>(state, this);
    this.width().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_stretch(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    let value = parse_stretch_or_throw(value)?;
    this.set_width(value);
    Ok(())
}

#[op2]
#[string]
pub fn op_canvas_2d_font_face_unicode_range(state: &OpState, this: *const c_void) -> String {
    let this = borrow_v8::<FontFace>(state, this);
    this.unicode_range().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_unicode_range(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    let value = parse_unicode_range_or_throw(value)?;
    this.set_unicode_range(value);
    Ok(())
}

#[op2]
pub fn op_canvas_2d_font_face_load(
    state: &OpState,
    this: *const c_void,
    #[anybuffer(copy)] source: Vec<u8>,
    from_url: bool,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    this.load(source).map_err(|err| {
        custom_error(
            if from_url {
                "DOMExceptionNetworkError"
            } else {
                "DOMExceptionSyntaxError"
            },
            format!("{}", err),
        )
    })
}

#[op2]
pub fn op_canvas_2d_font_face_set_new<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
) -> v8::Local<'a, v8::External> {
    let result = Rc::new(RefCell::new(FontFaceSet::new()));
    into_v8(state, scope, result)
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_insert(
    state: &OpState,
    this: *const c_void,
    font: *const c_void,
) {
    let this = borrow_v8::<Rc<RefCell<FontFaceSet>>>(state, this);
    let font = borrow_v8::<FontFace>(state, font);
    let mut this = this.borrow_mut();
    this.insert(&font)
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_remove(
    state: &OpState,
    this: *const c_void,
    font: *const c_void,
) {
    let this = borrow_v8::<Rc<RefCell<FontFaceSet>>>(state, this);
    let font = borrow_v8::<FontFace>(state, font);
    let mut this = this.borrow_mut();
    this.remove(font.id())
}

#[op2(fast)]
pub fn op_canvas_2d_font_face_set_clear(state: &OpState, this: *const c_void) {
    let this = borrow_v8::<Rc<RefCell<FontFaceSet>>>(state, this);
    let mut this = this.borrow_mut();
    this.clear()
}

#[op2]
pub fn op_canvas_2d_font_source<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
) -> v8::Local<'a, v8::External> {
    let result = state.borrow::<Rc<RefCell<FontFaceSet>>>().clone();
    into_v8(state, scope, result)
}

pub fn init(state: &mut OpState) {
    state.put(Rc::new(RefCell::new(FontFaceSet::new())));
}
