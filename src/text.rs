use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::num::NonZeroU64;
use std::ops::Range;
use std::process;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use cssparser::{ToCss as _, UnicodeRange};
use deno_core::error::{custom_error, type_error};
use deno_core::{anyhow, op2, v8, OpState};
use euclid::default::{Box2D, Point2D, Transform2D};
use euclid::{point2, size2, vec2};
use harfbuzz_rs as hb;
use hashlink::LinkedHashMap;
use unicode_bidi::{self as bidi, ParagraphBidiInfo};

use super::css::font::{
    ComputedFamilyName, ComputedFontFamily, ComputedFontStyle, ComputedFontVariantCss2,
    ComputedSpecificFamily, ComputedUnicodeRange,
};
use super::css::FromCss as _;
use super::gc::{borrow_v8, into_v8};
use super::harfbuzz_ext::FontExt as _;
use super::path::Path;
use super::state::{CanvasDirection, CanvasTextAlign, CanvasTextBaseline, DrawingState};

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
pub enum FontFaceState {
    Unloaded,
    Loaded(hb::Shared<hb::Font<'static>>),
    Errored,
}

#[derive(Debug)]
pub struct FontFaceData {
    family: ComputedSpecificFamily,
    unicode_range: ComputedUnicodeRange,
    state: FontFaceState,
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

    pub fn family(&self) -> ComputedSpecificFamily {
        let data = self.data.borrow();
        data.family.clone()
    }

    pub fn set_family(&self, value: ComputedSpecificFamily) {
        let mut data = self.data.borrow_mut();
        data.family = value;
    }

    pub fn unicode_range(&self) -> ComputedUnicodeRange {
        let data = self.data.borrow();
        data.unicode_range.clone()
    }

    pub fn set_unicode_range(&self, value: ComputedUnicodeRange) {
        let mut data = self.data.borrow_mut();
        data.unicode_range = value;
    }

    pub fn load_binary_data(&self, blob: Vec<u8>) -> anyhow::Result<()> {
        // TODO validate blob
        let mut data = self.data.borrow_mut();
        data.state = FontFaceState::Loaded(hb::Font::new(hb::Face::new(blob, 0)).into());
        Ok(())
    }
}

#[derive(Debug)]
pub struct FontSelector {
    family: ComputedFontFamily,
}

#[derive(Debug, Default)]
pub struct FontFaceSet {
    entries: LinkedHashMap<FontFaceId, Rc<RefCell<FontFaceData>>>,
}

impl FontFaceSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, font: &FontFace) {
        self.entries.insert(font.id, font.data.clone());
    }

    fn resolve_family_name(name: &ComputedFamilyName) -> &str {
        match *name {
            ComputedFamilyName::Specific(ref v) => &v.0,
            ComputedFamilyName::Generic(_) => "Arial", // TODO actually select a family
        }
    }

    pub fn first_available_font(&self, font_family: &ComputedFontFamily) -> Option<FontFace> {
        font_family.family_list.iter().find_map(|family_name| {
            let family_name = Self::resolve_family_name(family_name);
            for (&id, data_rc) in self.entries.iter().rev() {
                let data = data_rc.borrow();
                if unicase::eq(family_name, &data.family.0)
                    && data.unicode_range.contains(' ' as u32)
                {
                    return Some(FontFace {
                        id,
                        data: data_rc.clone(),
                    });
                }
            }
            None
        })
    }

    pub fn match_char(&self, c: char, selector: &FontSelector) -> Option<FontFace> {
        fn is_better_match(cur: &FontFaceData, prev: &FontFaceData) -> bool {
            // TODO compare font stretch
            // TODO compare font style
            // TODO compare font weight
            false
        }

        selector.family.family_list.iter().find_map(|family_name| {
            let family_name = Self::resolve_family_name(family_name);
            let mut best_match = None::<FontFace>;
            for (&id, data_rc) in self.entries.iter().rev() {
                let data = data_rc.borrow();
                if unicase::eq(family_name, &data.family.0)
                    && match best_match {
                        Some(ref prev) => is_better_match(&data, &prev.data.borrow()),
                        None => true,
                    }
                    && data.unicode_range.contains(c as u32)
                    && match data.state {
                        FontFaceState::Loaded(ref font) => font.get_nominal_glyph(c).is_some(),
                        _ => false,
                    }
                {
                    best_match = Some(FontFace {
                        id,
                        data: data_rc.clone(),
                    });
                }
            }
            best_match
        })
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
    font_selector: &FontSelector,
    base_direction: Direction,
    text: &str,
) -> Vec<TextRun> {
    let bidi_info = ParagraphBidiInfo::new(text, Some(base_direction.to_bidi_level()));
    let (levels, runs) = bidi_info.visual_runs(0..bidi_info.levels.len());
    runs.into_iter()
        .flat_map(|range| {
            let direction = Direction::from_bidi_level(levels[range.start]);
            let mut runs = Vec::new();
            let mut font_mapping = text[range.clone()]
                .char_indices()
                .map(|(i, c)| (range.start + i, fonts.match_char(c, font_selector)));
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
    let ppem = drawing_state.font.font_size.0.px;
    let font_selector = FontSelector {
        family: drawing_state.font.font_family.clone(),
    };
    let mut features = Vec::new();
    match drawing_state.font.font_variant {
        // TODO figure out what to do with `fontVariantCaps`
        ComputedFontVariantCss2::Normal => {}
        ComputedFontVariantCss2::SmallCaps => {
            // TODO synthesis if `smcp` feature is unavailable
            features.push(hb::Feature::new(b"smcp", 1, ..));
        }
    }
    // TODO <font-weight>
    // TODO <font-width> and `fontStretch`
    // TODO `fontKerning`
    // TODO `textRendering`
    // TODO `letterSpacing`
    // TODO `wordSpacing`
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
    let runs = split_text_to_runs(fonts, &font_selector, direction, &text);
    let mut path_builder = TextPathBuilder {
        path: Path::new(),
        transform: Transform2D::scale(0.0, 0.0),
    };
    let mut cursor = Point2D::zero();
    let mut bounds = Box2D::zero();
    let mut font_cache = HashMap::new();
    runs.into_iter().fold(hb::UnicodeBuffer::new(), |buf, run| {
        let Some(font) = run.font else {
            return buf;
        };
        let font = font_cache.entry(font.id).or_insert_with(|| {
            let data = font.data.borrow();
            match data.state {
                FontFaceState::Loaded(ref font) => {
                    let mut font = hb::Font::create_sub_font(font.clone());
                    match drawing_state.font.font_style {
                        ComputedFontStyle::Normal => {}
                        ComputedFontStyle::Italic => {
                            // TODO use italic face or `ital` feature
                            font.set_synthetic_slant(0.25);
                        }
                        ComputedFontStyle::Oblique(angle) => {
                            // TODO use oblique face or `slnt` feature
                            font.set_synthetic_slant(angle.radians().tan())
                        }
                    }
                    font
                }
                _ => hb::Font::empty(),
            }
        });
        let scale = ppem / font.face().upem() as f32;
        let buf = buf
            .add_str_item(&text, &text[run.range])
            .set_direction(run.direction.to_harfbuzz());
        let buf = hb::shape(font, buf, &features);
        let positions = buf.get_glyph_positions();
        let infos = buf.get_glyph_infos();
        for (position, info) in positions.iter().zip(infos) {
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
        }
        buf.clear()
    });
    let width = cursor.x;
    let compression = (max_width / width).min(1.0);
    let width = width * compression;
    let bounds = bounds * compression;
    let font_metrics = match fonts.first_available_font(&drawing_state.font.font_family) {
        Some(ref font) => {
            let data = font.data.borrow();
            match data.state {
                FontFaceState::Loaded(ref font) => {
                    let scale = ppem / font.face().upem() as f32;
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

fn parse_family_or_throw(css: &str) -> anyhow::Result<ComputedSpecificFamily> {
    ComputedSpecificFamily::from_css_string(css).map_err(|err| {
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

fn parse_unicode_range_or_throw(css: &str) -> anyhow::Result<ComputedUnicodeRange> {
    ComputedUnicodeRange::from_css_string(css).map_err(|err| {
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
    #[string] source: Option<String>,
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    if source.is_some() {
        // TODO implement font loading from url
        return Err(type_error("Cannot load font from URL"));
    }
    let family = parse_family_or_throw(&family)?;
    // TODO parse `style`
    // TODO parse `weight`
    // TODO parse `stretch`
    let unicode_range = parse_unicode_range_or_throw(&unicode_range)?;
    // TODO parse `feature_settings`
    // TODO parse `variation_settings`
    // TODO parse `display`
    // TODO parse `ascent_override`
    // TODO parse `descent_override`
    // TODO parse `line_gap_override`
    let result = FontFace::new(FontFaceData {
        family,
        unicode_range,
        state: FontFaceState::Unloaded,
    });
    Ok(into_v8(state, scope, result))
}

#[op2]
pub fn op_canvas_2d_font_face_errored<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
) -> v8::Local<'a, v8::External> {
    let result = FontFace::new(FontFaceData {
        family: ComputedSpecificFamily("".into()),
        unicode_range: ComputedUnicodeRange {
            range_list: Rc::new([UnicodeRange {
                start: 0,
                end: 0x10ffff,
            }]),
        },
        state: FontFaceState::Errored,
    });
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
pub fn op_canvas_2d_font_face_load_binary_data(
    state: &OpState,
    this: *const c_void,
    #[anybuffer(copy)] source: Vec<u8>,
) -> anyhow::Result<()> {
    let this = borrow_v8::<FontFace>(state, this);
    this.load_binary_data(source)
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
pub fn op_canvas_2d_font_face_set_add(state: &OpState, this: *const c_void, font: *const c_void) {
    let this = borrow_v8::<Rc<RefCell<FontFaceSet>>>(state, this);
    let font = borrow_v8::<FontFace>(state, font);
    let mut this = this.borrow_mut();
    this.add(&font)
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
