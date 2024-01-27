use std::borrow::Cow;

use euclid::default::{Box2D, Point2D, Transform2D};
use euclid::{point2, size2, vec2};
use harfbuzz_rs as hb;
use unicode_bidi::ParagraphBidiInfo;

use super::css::font::{ComputedFontStyle, ComputedFontVariantCss2};
use super::harfbuzz_ext::FontExt as _;
use super::path::Path;
use super::state::{CanvasDirection, CanvasTextAlign, CanvasTextBaseline, DrawingState};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    fn to_bidi_level(self) -> unicode_bidi::Level {
        match self {
            Self::Ltr => unicode_bidi::LTR_LEVEL,
            Self::Rtl => unicode_bidi::RTL_LEVEL,
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

pub fn prepare_text(
    drawing_state: &DrawingState,
    text: &str,
    max_width: f64,
) -> (Path, TextMetrics) {
    if max_width <= 0.0 {
        return (Path::new(), TextMetrics::empty());
    }
    let text = replace_ascii_whitespace(text);
    let face = hb::Face::from_bytes(
        // TODO do not hardcode font
        include_bytes!("/System/Library/Fonts/Supplemental/Arial.ttf"),
        0,
    );
    let ppem = drawing_state.font.font_size.0.px;
    let scale = ppem / face.upem() as f32;
    let mut font = hb::Font::new(face);
    let mut features = Vec::new();
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
    let mut path_builder = TextPathBuilder {
        path: Path::new(),
        transform: Transform2D::scale(0.0, 0.0),
    };
    let mut cursor = Point2D::zero();
    let mut bounds = Box2D::zero();
    let bidi_info = ParagraphBidiInfo::new(&text, Some(direction.to_bidi_level()));
    let (levels, runs) = bidi_info.visual_runs(0..bidi_info.levels.len());
    runs.into_iter().fold(hb::UnicodeBuffer::new(), |buf, run| {
        let buf = buf
            .add_str_item(bidi_info.text, &bidi_info.text[run.clone()])
            .set_direction(if levels[run.start].is_rtl() {
                hb::Direction::Rtl
            } else {
                hb::Direction::Ltr
            });
        let buf = hb::shape(&font, buf, &features);
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
    let font_metrics = FontMetrics::new(&font);
    let ascent = font_metrics.ascent * scale;
    let descent = font_metrics.descent * scale;
    let em_ascent = font_metrics.em_ascent * scale;
    let em_descent = font_metrics.em_descent * scale;
    let hanging_baseline = font_metrics.hanging_baseline * scale;
    let alphabetic_baseline = font_metrics.alphabetic_baseline * scale;
    let ideographic_baseline = font_metrics.ideographic_baseline * scale;
    let anchor_x = match physical_alignment {
        PhysicalAlignment::Left => 0.0,
        PhysicalAlignment::Right => width,
        PhysicalAlignment::Center => width * 0.5,
    };
    let anchor_y = match drawing_state.text_baseline {
        CanvasTextBaseline::Top => em_ascent,
        CanvasTextBaseline::Hanging => hanging_baseline,
        CanvasTextBaseline::Middle => (em_ascent + em_descent) * 0.5,
        CanvasTextBaseline::Alphabetic => alphabetic_baseline,
        CanvasTextBaseline::Ideographic => ideographic_baseline,
        CanvasTextBaseline::Bottom => em_descent,
    };
    let path = path_builder.path.transform(&Transform2D::translation(
        -anchor_x as f64,
        -anchor_y as f64,
    ));
    let text_metrics = TextMetrics {
        width,
        actual_bounding_box_left: anchor_x - bounds.min.x,
        actual_bounding_box_right: bounds.max.x - anchor_x,
        font_bounding_box_ascent: ascent - anchor_y,
        font_bounding_box_descent: anchor_y - descent,
        actual_bounding_box_ascent: bounds.max.y - anchor_y,
        actual_bounding_box_descent: anchor_y - bounds.min.y,
        em_height_ascent: em_ascent - anchor_y,
        em_height_descent: anchor_y - em_descent,
        hanging_baseline: hanging_baseline - anchor_y,
        alphabetic_baseline: alphabetic_baseline - anchor_y,
        ideographic_baseline: ideographic_baseline - anchor_y,
    };
    (path, text_metrics)
}
