use std::cell::RefCell;
use std::convert::Infallible;
use std::fmt::{self, Debug};
use std::rc::Rc;

use cssparser::ToCss as _;
use deno_core::{op2, v8, GarbageCollected, OpState};
use euclid::default::{Box2D, Point2D, Transform2D, Vector2D};
use euclid::{point2, size2, vec2, Angle};
use strum_macros::FromRepr;

use super::convert::{
    display_p3_to_premultiplied_linear_srgb, pack_rgba8_to_argb32,
    premultiplied_linear_display_p3_to_srgb, premultiplied_linear_srgb_to_display_p3,
    premultiplied_linear_srgb_to_srgb, srgb_to_premultiplied_linear_display_p3,
    srgb_to_premultiplied_linear_srgb, unpack_argb32_to_rgba8,
};
use super::css::color::{AbsoluteColor, ComputedColor};
use super::css::filter::ComputedFilter;
use super::css::font::{
    ComputedFamilyName, ComputedFont, ComputedFontFamily, ComputedFontSize,
    ComputedFontStretchCss3, ComputedFontStyle, ComputedFontVariantCaps, ComputedFontWeight,
    ComputedGenericFamily, ComputedLineHeight,
};
use super::css::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::css::FromCss as _;
use super::error::Canvas2DError;
use super::filter::{compile_filter, BoxedRenderFunction, FilterChain};
use super::gradient::CanvasGradient;
use super::image_bitmap::ImageBitmap;
use super::image_data::{AlignedImageDataView, AlignedImageDataViewMut};
use super::path::{CanvasFillRule, Path};
use super::pattern::CanvasPattern;
use super::text::{prepare_text, FontFaceSet, TextMetrics};
use super::wrap::Wrap;
use super::{
    raqote_ext, resolve_color_for_canvas, serialize_color_for_canvas, to_raqote_color,
    to_raqote_point, to_raqote_size, to_raqote_solid_source, CanvasColorSpace, ARGB32_ALPHA_MASK,
};

const TRANSPARENT_SOLID_SOURCE: raqote::SolidSource = raqote::SolidSource {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
const OPAQUE_BLACK_SOLID_SOURCE: raqote::SolidSource = raqote::SolidSource {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasLineCap {
    Butt,
    Round,
    Square,
}

impl CanvasLineCap {
    pub fn to_raqote(self) -> raqote::LineCap {
        match self {
            Self::Butt => raqote::LineCap::Butt,
            Self::Round => raqote::LineCap::Round,
            Self::Square => raqote::LineCap::Square,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasLineJoin {
    Round,
    Bevel,
    Miter,
}

impl CanvasLineJoin {
    pub fn to_raqote(self) -> raqote::LineJoin {
        match self {
            Self::Round => raqote::LineJoin::Round,
            Self::Bevel => raqote::LineJoin::Bevel,
            Self::Miter => raqote::LineJoin::Miter,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasTextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasTextBaseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasDirection {
    Ltr,
    Rtl,
    Inherit,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasFontKerning {
    Auto,
    Normal,
    None,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasFontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl From<ComputedFontStretchCss3> for CanvasFontStretch {
    fn from(value: ComputedFontStretchCss3) -> Self {
        match value {
            ComputedFontStretchCss3::Normal => Self::Normal,
            ComputedFontStretchCss3::UltraCondensed => Self::UltraCondensed,
            ComputedFontStretchCss3::ExtraCondensed => Self::ExtraCondensed,
            ComputedFontStretchCss3::Condensed => Self::Condensed,
            ComputedFontStretchCss3::SemiCondensed => Self::SemiCondensed,
            ComputedFontStretchCss3::SemiExpanded => Self::SemiExpanded,
            ComputedFontStretchCss3::Expanded => Self::Expanded,
            ComputedFontStretchCss3::ExtraExpanded => Self::ExtraExpanded,
            ComputedFontStretchCss3::UltraExpanded => Self::UltraExpanded,
        }
    }
}

impl From<CanvasFontStretch> for ComputedFontStretchCss3 {
    fn from(value: CanvasFontStretch) -> ComputedFontStretchCss3 {
        match value {
            CanvasFontStretch::UltraCondensed => Self::UltraCondensed,
            CanvasFontStretch::ExtraCondensed => Self::ExtraCondensed,
            CanvasFontStretch::Condensed => Self::Condensed,
            CanvasFontStretch::SemiCondensed => Self::SemiCondensed,
            CanvasFontStretch::Normal => Self::Normal,
            CanvasFontStretch::SemiExpanded => Self::SemiExpanded,
            CanvasFontStretch::Expanded => Self::Expanded,
            CanvasFontStretch::ExtraExpanded => Self::ExtraExpanded,
            CanvasFontStretch::UltraExpanded => Self::UltraExpanded,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasFontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

impl From<ComputedFontVariantCaps> for CanvasFontVariantCaps {
    fn from(value: ComputedFontVariantCaps) -> Self {
        match value {
            ComputedFontVariantCaps::Normal => Self::Normal,
            ComputedFontVariantCaps::SmallCaps => Self::SmallCaps,
            ComputedFontVariantCaps::AllSmallCaps => Self::AllSmallCaps,
            ComputedFontVariantCaps::PetiteCaps => Self::PetiteCaps,
            ComputedFontVariantCaps::AllPetiteCaps => Self::AllPetiteCaps,
            ComputedFontVariantCaps::Unicase => Self::Unicase,
            ComputedFontVariantCaps::TitlingCaps => Self::TitlingCaps,
        }
    }
}

impl From<CanvasFontVariantCaps> for ComputedFontVariantCaps {
    fn from(value: CanvasFontVariantCaps) -> ComputedFontVariantCaps {
        match value {
            CanvasFontVariantCaps::Normal => Self::Normal,
            CanvasFontVariantCaps::SmallCaps => Self::SmallCaps,
            CanvasFontVariantCaps::AllSmallCaps => Self::AllSmallCaps,
            CanvasFontVariantCaps::PetiteCaps => Self::PetiteCaps,
            CanvasFontVariantCaps::AllPetiteCaps => Self::AllPetiteCaps,
            CanvasFontVariantCaps::Unicase => Self::Unicase,
            CanvasFontVariantCaps::TitlingCaps => Self::TitlingCaps,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasTextRendering {
    Auto,
    OptimizeSpeed,
    OptimizeLegibility,
    GeometricPrecision,
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum BlendOrCompositeMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    Clear,
    Copy,
    SourceOver,
    DestinationOver,
    SourceIn,
    DestinationIn,
    SourceOut,
    DestinationOut,
    SourceAtop,
    DestinationAtop,
    Xor,
    Lighter,
    PlusDarker,
    PlusLighter,
}

impl BlendOrCompositeMode {
    pub fn to_raqote(self, alpha: bool) -> raqote::BlendMode {
        match self {
            Self::Normal => raqote::BlendMode::SrcOver,
            Self::Multiply => raqote::BlendMode::Multiply,
            Self::Screen => raqote::BlendMode::Screen,
            Self::Overlay => raqote::BlendMode::Overlay,
            Self::Darken => raqote::BlendMode::Darken,
            Self::Lighten => raqote::BlendMode::Lighten,
            Self::ColorDodge => raqote::BlendMode::ColorDodge,
            Self::ColorBurn => raqote::BlendMode::ColorBurn,
            Self::HardLight => raqote::BlendMode::HardLight,
            Self::SoftLight => raqote::BlendMode::SoftLight,
            Self::Difference => raqote::BlendMode::Difference,
            Self::Exclusion => raqote::BlendMode::Exclusion,
            Self::Hue => raqote::BlendMode::Hue,
            Self::Saturation => raqote::BlendMode::Saturation,
            Self::Color => raqote::BlendMode::Color,
            Self::Luminosity => raqote::BlendMode::Luminosity,
            Self::Clear => {
                if alpha {
                    raqote::BlendMode::Clear
                } else {
                    raqote::BlendMode::ClearOpaque
                }
            }
            Self::Copy => {
                if alpha {
                    raqote::BlendMode::Src
                } else {
                    raqote::BlendMode::SrcOpaque
                }
            }
            Self::SourceOver => raqote::BlendMode::SrcOver,
            Self::DestinationOver => {
                if alpha {
                    raqote::BlendMode::DstOver
                } else {
                    raqote::BlendMode::Dst
                }
            }
            Self::SourceIn => {
                if alpha {
                    raqote::BlendMode::SrcIn
                } else {
                    raqote::BlendMode::SrcOpaque
                }
            }
            Self::DestinationIn => {
                if alpha {
                    raqote::BlendMode::DstIn
                } else {
                    raqote::BlendMode::DstInOpaque
                }
            }
            Self::SourceOut => {
                if alpha {
                    raqote::BlendMode::SrcOut
                } else {
                    raqote::BlendMode::ClearOpaque
                }
            }
            Self::DestinationOut => {
                if alpha {
                    raqote::BlendMode::DstOut
                } else {
                    raqote::BlendMode::DstOutOpaque
                }
            }
            Self::SourceAtop => {
                if alpha {
                    raqote::BlendMode::SrcAtop
                } else {
                    raqote::BlendMode::SrcOver
                }
            }
            Self::DestinationAtop => {
                if alpha {
                    raqote::BlendMode::DstAtop
                } else {
                    raqote::BlendMode::DstInOpaque
                }
            }
            Self::Xor => {
                if alpha {
                    raqote::BlendMode::Xor
                } else {
                    raqote::BlendMode::DstOutOpaque
                }
            }
            Self::Lighter => raqote::BlendMode::Add,
            Self::PlusDarker => unimplemented!(),
            Self::PlusLighter => raqote::BlendMode::Add,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum ImageSmoothingQuality {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug)]
pub enum FillOrStrokeStyle {
    Color(AbsoluteColor),
    Gradient(Rc<CanvasGradient>),
    Pattern(Rc<CanvasPattern>),
}

impl FillOrStrokeStyle {
    pub fn to_raqote(
        &self,
        destination_color_space: CanvasColorSpace,
        image_smoothing_enabled: bool,
    ) -> Option<raqote_ext::OwnedSource> {
        match *self {
            FillOrStrokeStyle::Color(color) => {
                let color = to_raqote_solid_source(color, destination_color_space);
                (color.a != 0).then_some(raqote_ext::OwnedSource::Solid(color))
            }
            FillOrStrokeStyle::Gradient(ref gradient) => {
                gradient.to_raqote(destination_color_space)
            }
            FillOrStrokeStyle::Pattern(ref pattern) => {
                pattern.to_raqote(destination_color_space, image_smoothing_enabled)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawingState {
    line_width: f64,
    line_cap: CanvasLineCap,
    line_join: CanvasLineJoin,
    miter_limit: f64,
    dash_list: Option<Rc<[f64]>>,
    line_dash_offset: f64,
    pub font_style: ComputedFontStyle,
    pub font_weight: ComputedFontWeight,
    pub font_size: ComputedFontSize,
    pub font_family: ComputedFontFamily,
    pub text_align: CanvasTextAlign,
    pub text_baseline: CanvasTextBaseline,
    pub direction: CanvasDirection,
    pub letter_spacing: SpecifiedAbsoluteLength,
    pub word_spacing: SpecifiedAbsoluteLength,
    pub font_kerning: CanvasFontKerning,
    pub font_stretch: ComputedFontStretchCss3,
    pub font_variant_caps: ComputedFontVariantCaps,
    pub text_rendering: CanvasTextRendering,
    transformation_matrix: Transform2D<f64>,
    fill_style: FillOrStrokeStyle,
    stroke_style: FillOrStrokeStyle,
    clip_depth: usize,
    global_alpha: f64,
    compositing_and_blending_operator: BlendOrCompositeMode,
    image_smoothing_enabled: bool,
    image_smoothing_quality: ImageSmoothingQuality,
    shadow_color: AbsoluteColor,
    shadow_offset: Vector2D<f64>,
    shadow_blur: f64,
    filter: ComputedFilter,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            line_width: 1.0,
            line_cap: CanvasLineCap::Butt,
            line_join: CanvasLineJoin::Miter,
            miter_limit: 10.0,
            dash_list: None,
            line_dash_offset: 0.0,
            font_style: ComputedFontStyle::Normal,
            font_weight: ComputedFontWeight(400.0),
            font_size: ComputedFontSize(ComputedLength { px: 10.0 }),
            font_family: ComputedFontFamily {
                family_list: Rc::new([ComputedFamilyName::Generic(
                    ComputedGenericFamily::SansSerif,
                )]),
            },
            text_align: CanvasTextAlign::Start,
            text_baseline: CanvasTextBaseline::Alphabetic,
            direction: CanvasDirection::Inherit,
            letter_spacing: SpecifiedAbsoluteLength::zero(),
            word_spacing: SpecifiedAbsoluteLength::zero(),
            font_kerning: CanvasFontKerning::Auto,
            font_stretch: ComputedFontStretchCss3::Normal,
            font_variant_caps: ComputedFontVariantCaps::Normal,
            text_rendering: CanvasTextRendering::Auto,
            transformation_matrix: Transform2D::identity(),
            fill_style: FillOrStrokeStyle::Color(AbsoluteColor::OPAQUE_BLACK),
            stroke_style: FillOrStrokeStyle::Color(AbsoluteColor::OPAQUE_BLACK),
            clip_depth: 0,
            global_alpha: 1.0,
            compositing_and_blending_operator: BlendOrCompositeMode::SourceOver,
            image_smoothing_enabled: true,
            image_smoothing_quality: ImageSmoothingQuality::Low,
            shadow_color: AbsoluteColor::TRANSPARENT_BLACK,
            shadow_offset: Vector2D::zero(),
            shadow_blur: 0.0,
            filter: ComputedFilter::default(),
        }
    }
}

impl DrawingState {
    pub fn get_raqote_fill_source(
        &self,
        color_space: CanvasColorSpace,
    ) -> Option<raqote_ext::OwnedSource> {
        self.fill_style
            .to_raqote(color_space, self.image_smoothing_enabled)
    }

    pub fn get_raqote_stroke_source(
        &self,
        color_space: CanvasColorSpace,
    ) -> Option<raqote_ext::OwnedSource> {
        self.stroke_style
            .to_raqote(color_space, self.image_smoothing_enabled)
    }

    pub fn get_raqote_stroke_style(&self) -> raqote::StrokeStyle {
        raqote::StrokeStyle {
            width: self.line_width as f32,
            cap: self.line_cap.to_raqote(),
            join: self.line_join.to_raqote(),
            miter_limit: self.miter_limit as f32,
            dash_array: match self.dash_list {
                Some(ref v) => v.iter().map(|&x| x as f32).collect(),
                None => vec![],
            },
            dash_offset: self.line_dash_offset as f32,
        }
    }
}

pub struct CanvasState {
    draw_target: raqote::DrawTarget,
    alpha: bool,
    color_space: CanvasColorSpace,
    current_drawing_state: DrawingState,
    drawing_state_stack: Vec<DrawingState>,
}

impl Debug for CanvasState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CanvasState")
            .field("color_space", &self.color_space)
            .field("alpha", &self.alpha)
            .field("current_drawing_state", &self.current_drawing_state)
            .field("drawing_state_stack", &self.drawing_state_stack)
            .finish_non_exhaustive()
    }
}

impl CanvasState {
    pub fn new(
        width: u64,
        height: u64,
        alpha: bool,
        color_space: CanvasColorSpace,
    ) -> Result<Self, Canvas2DError> {
        let size = to_raqote_size(width, height)?;
        let mut draw_target = raqote::DrawTarget::new(size.width, size.height);
        if !alpha {
            draw_target.get_data_mut().fill(ARGB32_ALPHA_MASK);
        }
        Ok(CanvasState {
            draw_target,
            alpha,
            color_space,
            current_drawing_state: Default::default(),
            drawing_state_stack: Vec::new(),
        })
    }

    pub fn width(&self) -> u64 {
        self.draw_target.width() as u64
    }

    pub fn height(&self) -> u64 {
        self.draw_target.height() as u64
    }

    pub fn color_space(&self) -> CanvasColorSpace {
        self.color_space
    }

    pub fn as_raqote_image(&self) -> raqote::Image {
        raqote::Image {
            width: self.draw_target.width(),
            height: self.draw_target.height(),
            data: self.draw_target.get_data(),
        }
    }

    pub fn save(&mut self) {
        self.drawing_state_stack
            .push(self.current_drawing_state.clone());
    }

    pub fn restore(&mut self) {
        if let Some(top) = self.drawing_state_stack.pop() {
            for _ in top.clip_depth..self.current_drawing_state.clip_depth {
                self.draw_target.pop_clip();
            }
            self.current_drawing_state = top;
            self.update_transform();
        }
    }

    pub fn reset(&mut self, width: u64, height: u64) -> Result<(), Canvas2DError> {
        *self = Self::new(width, height, self.alpha, self.color_space)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        if self.alpha {
            self.draw_target.get_data_mut().fill(0);
        } else {
            self.draw_target.get_data_mut().fill(ARGB32_ALPHA_MASK);
        }
    }

    pub fn line_width(&self) -> f64 {
        self.current_drawing_state.line_width
    }

    pub fn set_line_width(&mut self, value: f64) {
        self.current_drawing_state.line_width = value;
    }

    pub fn line_cap(&self) -> CanvasLineCap {
        self.current_drawing_state.line_cap
    }

    pub fn set_line_cap(&mut self, value: CanvasLineCap) {
        self.current_drawing_state.line_cap = value;
    }

    pub fn line_join(&self) -> CanvasLineJoin {
        self.current_drawing_state.line_join
    }

    pub fn set_line_join(&mut self, value: CanvasLineJoin) {
        self.current_drawing_state.line_join = value;
    }

    pub fn miter_limit(&self) -> f64 {
        self.current_drawing_state.miter_limit
    }

    pub fn set_miter_limit(&mut self, value: f64) {
        self.current_drawing_state.miter_limit = value;
    }

    pub fn dash_list(&self) -> &[f64] {
        self.current_drawing_state
            .dash_list
            .as_deref()
            .unwrap_or_default()
    }

    pub fn set_dash_list(&mut self, segments: &[f64]) {
        self.current_drawing_state.dash_list = if segments.is_empty() {
            None
        } else {
            Some(segments.into())
        };
    }

    pub fn line_dash_offset(&self) -> f64 {
        self.current_drawing_state.line_dash_offset
    }

    pub fn set_line_dash_offset(&mut self, value: f64) {
        self.current_drawing_state.line_dash_offset = value;
    }

    pub fn font(&self) -> Option<ComputedFont> {
        Some(ComputedFont {
            style: self.current_drawing_state.font_style,
            variant: self.current_drawing_state.font_variant_caps.to_css2()?,
            weight: self.current_drawing_state.font_weight,
            stretch: self.current_drawing_state.font_stretch,
            size: self.current_drawing_state.font_size,
            line_height: ComputedLineHeight::Normal,
            family: self.current_drawing_state.font_family.clone(),
        })
    }

    pub fn set_font(&mut self, value: ComputedFont) {
        self.current_drawing_state.font_style = value.style;
        self.current_drawing_state.font_variant_caps = value.variant.modernize();
        self.current_drawing_state.font_weight = value.weight;
        self.current_drawing_state.font_stretch = value.stretch;
        self.current_drawing_state.font_size = value.size;
        self.current_drawing_state.font_family = value.family;
    }

    pub fn text_align(&self) -> CanvasTextAlign {
        self.current_drawing_state.text_align
    }

    pub fn set_text_align(&mut self, value: CanvasTextAlign) {
        self.current_drawing_state.text_align = value;
    }

    pub fn text_baseline(&self) -> CanvasTextBaseline {
        self.current_drawing_state.text_baseline
    }

    pub fn set_text_baseline(&mut self, value: CanvasTextBaseline) {
        self.current_drawing_state.text_baseline = value;
    }

    pub fn direction(&self) -> CanvasDirection {
        self.current_drawing_state.direction
    }

    pub fn set_direction(&mut self, value: CanvasDirection) {
        self.current_drawing_state.direction = value;
    }

    pub fn letter_spacing(&self) -> SpecifiedAbsoluteLength {
        self.current_drawing_state.letter_spacing
    }

    pub fn set_letter_spacing(&mut self, value: SpecifiedAbsoluteLength) {
        self.current_drawing_state.letter_spacing = value;
    }

    pub fn word_spacing(&self) -> SpecifiedAbsoluteLength {
        self.current_drawing_state.word_spacing
    }

    pub fn set_word_spacing(&mut self, value: SpecifiedAbsoluteLength) {
        self.current_drawing_state.word_spacing = value;
    }

    pub fn font_kerning(&self) -> CanvasFontKerning {
        self.current_drawing_state.font_kerning
    }

    pub fn set_font_kerning(&mut self, value: CanvasFontKerning) {
        self.current_drawing_state.font_kerning = value;
    }

    pub fn font_stretch(&self) -> CanvasFontStretch {
        self.current_drawing_state.font_stretch.into()
    }

    pub fn set_font_stretch(&mut self, value: CanvasFontStretch) {
        self.current_drawing_state.font_stretch = value.into();
    }

    pub fn font_variant_caps(&self) -> CanvasFontVariantCaps {
        self.current_drawing_state.font_variant_caps.into()
    }

    pub fn set_font_variant_caps(&mut self, value: CanvasFontVariantCaps) {
        self.current_drawing_state.font_variant_caps = value.into();
    }

    pub fn text_rendering(&self) -> CanvasTextRendering {
        self.current_drawing_state.text_rendering
    }

    pub fn set_text_rendering(&mut self, value: CanvasTextRendering) {
        self.current_drawing_state.text_rendering = value;
    }

    fn update_transform(&mut self) {
        self.draw_target
            .set_transform(&self.current_drawing_state.transformation_matrix.cast());
    }

    pub fn scale(&mut self, x: f64, y: f64) {
        self.current_drawing_state.transformation_matrix = self
            .current_drawing_state
            .transformation_matrix
            .pre_scale(x, y);
        self.update_transform();
    }

    pub fn rotate(&mut self, radians: f64) {
        self.current_drawing_state.transformation_matrix = self
            .current_drawing_state
            .transformation_matrix
            .pre_rotate(Angle::radians(radians));
        self.update_transform();
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        self.current_drawing_state.transformation_matrix = self
            .current_drawing_state
            .transformation_matrix
            .pre_translate(vec2(x, y));
        self.update_transform();
    }

    pub fn transform(&mut self, mat: Transform2D<f64>) {
        self.current_drawing_state.transformation_matrix =
            mat.then(&self.current_drawing_state.transformation_matrix);
        self.update_transform();
    }

    pub fn get_transform(&self) -> Transform2D<f64> {
        self.current_drawing_state.transformation_matrix
    }

    pub fn set_transform(&mut self, mat: Transform2D<f64>) {
        self.current_drawing_state.transformation_matrix = mat;
        self.update_transform();
    }

    pub fn reset_transform(&mut self) {
        self.current_drawing_state.transformation_matrix = Transform2D::identity();
        self.update_transform();
    }

    pub fn fill_style(&self) -> &FillOrStrokeStyle {
        &self.current_drawing_state.fill_style
    }

    pub fn set_fill_style(&mut self, value: FillOrStrokeStyle) {
        self.current_drawing_state.fill_style = value;
    }

    pub fn stroke_style(&self) -> &FillOrStrokeStyle {
        &self.current_drawing_state.stroke_style
    }

    pub fn set_stroke_style(&mut self, value: FillOrStrokeStyle) {
        self.current_drawing_state.stroke_style = value;
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.draw_target.fill_rect(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
            &raqote::Source::Solid(OPAQUE_BLACK_SOLID_SOURCE),
            &raqote::DrawOptions {
                blend_mode: if self.alpha {
                    raqote::BlendMode::Clear
                } else {
                    raqote::BlendMode::Src
                },
                ..Default::default()
            },
        );
    }

    fn draw_with_filter(&mut self, filter: Rc<FilterChain>, offset: Vector2D<f64>) {
        let width = self.draw_target.width() as usize;
        let height = self.draw_target.height() as usize;
        let transform = *self.draw_target.get_transform();
        let layer = filter.render(
            size2(width, height),
            &transform.then_translate(offset.cast()),
        );
        self.draw_target.set_transform(&Transform2D::identity());
        self.draw_target.fill_rect(
            0.0,
            0.0,
            width as f32,
            height as f32,
            &raqote::Source::Image(
                raqote::Image {
                    width: layer.width(),
                    height: layer.height(),
                    data: layer.get_data(),
                },
                raqote::ExtendMode::Pad,
                raqote::FilterMode::Nearest,
                raqote::Transform::identity(),
                false,
                false,
            ),
            &raqote::DrawOptions {
                blend_mode: self
                    .current_drawing_state
                    .compositing_and_blending_operator
                    .to_raqote(self.alpha),
                ..Default::default()
            },
        );
        self.draw_target.set_transform(&transform);
    }

    fn draw_shadow(&mut self, filter: Rc<FilterChain>) {
        let color = self.current_drawing_state.shadow_color;
        if self.current_drawing_state.shadow_color.alpha == 0.0 {
            return;
        }
        let offset = self.current_drawing_state.shadow_offset;
        let blur = self.current_drawing_state.shadow_blur;
        if offset.x == 0.0 && offset.y == 0.0 && blur == 0.0 {
            return;
        }
        let filter = filter.shadow(
            to_raqote_color(color, self.color_space),
            (blur * 0.5) as f32,
        );
        self.draw_with_filter(filter, offset);
    }

    fn paint<RF>(&mut self, prepare: impl FnOnce(&Self) -> RF)
    where
        RF: Fn(&mut raqote::DrawTarget, raqote::DrawOptions) + 'static,
    {
        self.try_paint(move |this| Ok::<_, Infallible>(prepare(this)))
            .unwrap()
    }

    fn try_paint<RF, E>(&mut self, prepare: impl FnOnce(&Self) -> Result<RF, E>) -> Result<(), E>
    where
        RF: Fn(&mut raqote::DrawTarget, raqote::DrawOptions) + 'static,
    {
        if matches!(
            self.current_drawing_state.compositing_and_blending_operator,
            BlendOrCompositeMode::Clear
        ) {
            if self.alpha {
                self.draw_target.clear(TRANSPARENT_SOLID_SOURCE);
            } else {
                self.draw_target.clear(OPAQUE_BLACK_SOLID_SOURCE);
            }
            return Ok(());
        }
        let filter = compile_filter(
            BoxedRenderFunction(Box::new(prepare(self)?)),
            self.current_drawing_state.global_alpha as f32,
            &self.current_drawing_state.filter,
            self.color_space,
        );
        if let FilterChain::Source { ref render, alpha } = *filter {
            match self.current_drawing_state.compositing_and_blending_operator {
                BlendOrCompositeMode::Copy => {
                    if self.alpha {
                        self.draw_target.clear(TRANSPARENT_SOLID_SOURCE);
                    } else {
                        self.draw_target.clear(OPAQUE_BLACK_SOLID_SOURCE);
                    }
                    render.0(
                        &mut self.draw_target,
                        raqote::DrawOptions {
                            blend_mode: self
                                .current_drawing_state
                                .compositing_and_blending_operator
                                .to_raqote(self.alpha),
                            alpha,
                            ..Default::default()
                        },
                    );
                    return Ok(());
                }
                BlendOrCompositeMode::Normal
                | BlendOrCompositeMode::Multiply
                | BlendOrCompositeMode::Screen
                | BlendOrCompositeMode::Overlay
                | BlendOrCompositeMode::Darken
                | BlendOrCompositeMode::Lighten
                | BlendOrCompositeMode::ColorDodge
                | BlendOrCompositeMode::ColorBurn
                | BlendOrCompositeMode::HardLight
                | BlendOrCompositeMode::SoftLight
                | BlendOrCompositeMode::Difference
                | BlendOrCompositeMode::Exclusion
                | BlendOrCompositeMode::Hue
                | BlendOrCompositeMode::Saturation
                | BlendOrCompositeMode::Color
                | BlendOrCompositeMode::Luminosity
                | BlendOrCompositeMode::SourceOver
                | BlendOrCompositeMode::DestinationOver
                | BlendOrCompositeMode::DestinationOut
                | BlendOrCompositeMode::SourceAtop
                | BlendOrCompositeMode::Xor
                | BlendOrCompositeMode::Lighter
                | BlendOrCompositeMode::PlusDarker
                | BlendOrCompositeMode::PlusLighter => {
                    self.draw_shadow(filter.clone());
                    render.0(
                        &mut self.draw_target,
                        raqote::DrawOptions {
                            blend_mode: self
                                .current_drawing_state
                                .compositing_and_blending_operator
                                .to_raqote(self.alpha),
                            alpha,
                            ..Default::default()
                        },
                    );
                    return Ok(());
                }
                _ => {}
            }
        }
        self.draw_shadow(filter.clone());
        self.draw_with_filter(filter, Vector2D::zero());
        Ok(())
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if width == 0.0 || height == 0.0 {
            return;
        }
        let color_space = self.color_space;
        self.paint(move |this| {
            let source = this
                .current_drawing_state
                .get_raqote_fill_source(color_space);
            move |draw_target, draw_options| {
                if let Some(ref source) = source {
                    draw_target.fill_rect(
                        x as f32,
                        y as f32,
                        width as f32,
                        height as f32,
                        &source.borrow(),
                        &draw_options,
                    );
                }
            }
        });
    }

    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let color_space = self.color_space;
        self.paint(move |this| {
            let path = this.draw_target.trace_path(
                &raqote::Path {
                    ops: {
                        let b =
                            Box2D::from_origin_and_size(point2(x, y), size2(width, height)).cast();
                        match (width == 0.0, height == 0.0) {
                            (true, true) => vec![],
                            (true, false) | (false, true) => {
                                vec![raqote::PathOp::MoveTo(b.min), raqote::PathOp::LineTo(b.max)]
                            }
                            (false, false) => vec![
                                raqote::PathOp::MoveTo(b.min),
                                raqote::PathOp::LineTo(point2(b.max.x, b.min.y)),
                                raqote::PathOp::LineTo(b.max),
                                raqote::PathOp::LineTo(point2(b.min.x, b.max.y)),
                                raqote::PathOp::Close,
                            ],
                        }
                    },
                    winding: raqote::Winding::NonZero,
                },
                &this.current_drawing_state.get_raqote_stroke_style(),
            );
            let source = this
                .current_drawing_state
                .get_raqote_stroke_source(color_space);
            move |draw_target, draw_options| {
                if let Some(ref source) = source {
                    draw_target.fill(&path, &source.borrow(), &draw_options);
                }
            }
        });
    }

    pub fn fill_text(&mut self, fonts: &FontFaceSet, text: &str, x: f64, y: f64, max_width: f64) {
        let color_space = self.color_space;
        self.paint(move |this| {
            let (path, _) =
                prepare_text(fonts, &this.current_drawing_state, text, max_width as f32);
            let path = path.transform(&Transform2D::new(1.0, 0.0, 0.0, -1.0, x, y));
            let path = path.to_raqote(CanvasFillRule::NonZero);
            let source = this
                .current_drawing_state
                .get_raqote_fill_source(color_space);
            move |draw_target, draw_options| {
                if let Some(ref source) = source {
                    draw_target.fill(&path, &source.borrow(), &draw_options);
                }
            }
        });
    }

    pub fn stroke_text(&mut self, fonts: &FontFaceSet, text: &str, x: f64, y: f64, max_width: f64) {
        let color_space = self.color_space;
        self.paint(move |this| {
            let (path, _) =
                prepare_text(fonts, &this.current_drawing_state, text, max_width as f32);
            let path = path.transform(&Transform2D::new(1.0, 0.0, 0.0, -1.0, x, y));
            let path = this.draw_target.trace_path(
                &raqote::Path {
                    ops: path.to_raqote_ops(),
                    winding: raqote::Winding::NonZero,
                },
                &this.current_drawing_state.get_raqote_stroke_style(),
            );
            let source = this
                .current_drawing_state
                .get_raqote_stroke_source(color_space);
            move |draw_target, draw_options| {
                if let Some(ref source) = source {
                    draw_target.fill(&path, &source.borrow(), &draw_options);
                }
            }
        });
    }

    pub fn measure_text(&self, fonts: &FontFaceSet, text: &str) -> TextMetrics {
        let (_, text_metrics) =
            prepare_text(fonts, &self.current_drawing_state, text, f32::INFINITY);
        text_metrics
    }

    pub fn fill(&mut self, path: &Path, fill_rule: CanvasFillRule) {
        let color_space = self.color_space;
        self.paint(move |this| {
            let path = path.to_raqote(fill_rule);
            let source = this
                .current_drawing_state
                .get_raqote_fill_source(color_space);
            move |draw_target, draw_options| {
                if let Some(ref source) = source {
                    draw_target.fill(&path, &source.borrow(), &draw_options);
                }
            }
        });
    }

    pub fn stroke(&mut self, path: &Path) {
        let color_space = self.color_space;
        self.paint(move |this| {
            let path = this.draw_target.trace_path(
                &raqote::Path {
                    ops: path.to_raqote_ops(),
                    winding: raqote::Winding::NonZero,
                },
                &this.current_drawing_state.get_raqote_stroke_style(),
            );
            let source = this
                .current_drawing_state
                .get_raqote_stroke_source(color_space);
            move |draw_target, draw_options| {
                if let Some(ref source) = source {
                    draw_target.fill(&path, &source.borrow(), &draw_options);
                }
            }
        });
    }

    pub fn clip(&mut self, path: &Path, fill_rule: CanvasFillRule) {
        let path = path.to_raqote(fill_rule);
        self.draw_target.push_clip(&path);
        self.current_drawing_state.clip_depth += 1;
    }

    pub fn is_point_in_path(&self, path: &Path, x: f64, y: f64, fill_rule: CanvasFillRule) -> bool {
        let path = path.to_raqote(fill_rule);
        path.transform(self.draw_target.get_transform())
            .contains_point(0.1, x as f32, y as f32)
    }

    pub fn is_point_in_stroke(&self, path: &Path, x: f64, y: f64) -> bool {
        let path = self.draw_target.trace_path(
            &raqote::Path {
                ops: path.to_raqote_ops(),
                winding: raqote::Winding::NonZero,
            },
            &self.current_drawing_state.get_raqote_stroke_style(),
        );
        path.transform(self.draw_target.get_transform())
            .contains_point(0.1, x as f32, y as f32)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_image(
        &mut self,
        image: ImageBitmap,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), Canvas2DError> {
        if sw == 0.0 || sh == 0.0 {
            return Ok(());
        }
        let color_space = self.color_space;
        self.try_paint(move |this| {
            let source = image
                .into_color_space(color_space)
                .into_raqote_image()?
                .map(|image| {
                    raqote_ext::OwnedSource::Image(
                        image,
                        raqote::ExtendMode::Pad,
                        if this.current_drawing_state.image_smoothing_enabled {
                            raqote::FilterMode::Bilinear
                        } else {
                            raqote::FilterMode::Nearest
                        },
                        Transform2D::translation(-dx, -dy)
                            .then(&Transform2D::new(sw / dw, 0.0, 0.0, sh / dh, sx, sy))
                            .cast(),
                        false,
                        false,
                    )
                });
            Ok(move |draw_target: &mut raqote::DrawTarget, draw_options| {
                if let Some(ref image) = source {
                    draw_target.fill_rect(
                        dx as f32,
                        dy as f32,
                        dw as f32,
                        dh as f32,
                        &image.borrow(),
                        &draw_options,
                    );
                }
            })
        })
    }

    pub fn get_image_data(
        &self,
        mut dst: AlignedImageDataViewMut,
        x: i64,
        y: i64,
    ) -> Result<(), Canvas2DError> {
        let dst_color_space = dst.color_space;
        let mut dst = dst.as_raqote_surface_rgba8()?;
        let src_origin = to_raqote_point(x, y)?;
        dst.composite_surface(
            &self.draw_target,
            Box2D::from_origin_and_size(src_origin, size2(dst.width(), dst.height())),
            Point2D::origin(),
            |src, dst| {
                dst.copy_from_slice(src);
                match (self.color_space, dst_color_space) {
                    (CanvasColorSpace::Srgb, CanvasColorSpace::Srgb)
                    | (CanvasColorSpace::DisplayP3, CanvasColorSpace::DisplayP3) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_srgb_to_srgb);
                    }
                    (CanvasColorSpace::Srgb, CanvasColorSpace::DisplayP3) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_srgb_to_display_p3);
                    }
                    (CanvasColorSpace::DisplayP3, CanvasColorSpace::Srgb) => {
                        unpack_argb32_to_rgba8(dst, premultiplied_linear_display_p3_to_srgb);
                    }
                }
            },
        );
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn put_image_data(
        &mut self,
        src: AlignedImageDataView,
        sx: i64,
        sy: i64,
        sw: u64,
        sh: u64,
        dx: i64,
        dy: i64,
    ) -> Result<(), Canvas2DError> {
        let src_color_space = src.color_space;
        let src = src.as_raqote_surface_rgba8()?;
        let src_origin = to_raqote_point(sx, sy)?;
        let src_size = to_raqote_size(sw, sh)?;
        let dst_origin = to_raqote_point(dx, dy)?;
        self.draw_target.composite_surface(
            &src,
            Box2D::from_origin_and_size(src_origin, src_size),
            dst_origin,
            |src, dst| {
                dst.copy_from_slice(src);
                match (src_color_space, self.color_space) {
                    (CanvasColorSpace::Srgb, CanvasColorSpace::Srgb)
                    | (CanvasColorSpace::DisplayP3, CanvasColorSpace::DisplayP3) => {
                        pack_rgba8_to_argb32(dst, srgb_to_premultiplied_linear_srgb);
                    }
                    (CanvasColorSpace::Srgb, CanvasColorSpace::DisplayP3) => {
                        pack_rgba8_to_argb32(dst, srgb_to_premultiplied_linear_display_p3);
                    }
                    (CanvasColorSpace::DisplayP3, CanvasColorSpace::Srgb) => {
                        pack_rgba8_to_argb32(dst, display_p3_to_premultiplied_linear_srgb);
                    }
                }
                if !self.alpha {
                    for pixel in dst {
                        *pixel |= ARGB32_ALPHA_MASK;
                    }
                }
            },
        );
        Ok(())
    }

    pub fn global_alpha(&self) -> f64 {
        self.current_drawing_state.global_alpha
    }

    pub fn set_global_alpha(&mut self, value: f64) {
        self.current_drawing_state.global_alpha = value;
    }

    pub fn global_composite_operation(&self) -> BlendOrCompositeMode {
        self.current_drawing_state.compositing_and_blending_operator
    }

    pub fn set_global_compositing_operator(&mut self, value: BlendOrCompositeMode) {
        self.current_drawing_state.compositing_and_blending_operator = value;
    }

    pub fn image_smoothing_enabled(&self) -> bool {
        self.current_drawing_state.image_smoothing_enabled
    }

    pub fn set_image_smoothing_enabled(&mut self, value: bool) {
        self.current_drawing_state.image_smoothing_enabled = value;
    }

    pub fn image_smoothing_quality(&self) -> ImageSmoothingQuality {
        self.current_drawing_state.image_smoothing_quality
    }

    pub fn set_image_smoothing_quality(&mut self, value: ImageSmoothingQuality) {
        self.current_drawing_state.image_smoothing_quality = value;
    }

    pub fn shadow_color(&self) -> AbsoluteColor {
        self.current_drawing_state.shadow_color
    }

    pub fn set_shadow_color(&mut self, value: AbsoluteColor) {
        self.current_drawing_state.shadow_color = value;
    }

    pub fn shadow_offset_x(&self) -> f64 {
        self.current_drawing_state.shadow_offset.x
    }

    pub fn set_shadow_offset_x(&mut self, value: f64) {
        self.current_drawing_state.shadow_offset.x = value;
    }

    pub fn shadow_offset_y(&self) -> f64 {
        self.current_drawing_state.shadow_offset.y
    }

    pub fn set_shadow_offset_y(&mut self, value: f64) {
        self.current_drawing_state.shadow_offset.y = value;
    }

    pub fn shadow_blur(&self) -> f64 {
        self.current_drawing_state.shadow_blur
    }

    pub fn set_shadow_blur(&mut self, value: f64) {
        self.current_drawing_state.shadow_blur = value;
    }

    pub fn set_filter(&mut self, value: ComputedFilter) {
        self.current_drawing_state.filter = value;
    }
}

impl GarbageCollected for Wrap<RefCell<CanvasState>> {}

#[op2]
#[cppgc]
pub fn op_canvas_2d_state_new(
    #[number] width: u64,
    #[number] height: u64,
    alpha: bool,
    color_space: i32,
) -> Result<Wrap<RefCell<CanvasState>>, Canvas2DError> {
    let color_space = CanvasColorSpace::from_repr(color_space).unwrap();
    Ok(Wrap::new(RefCell::new(CanvasState::new(
        width,
        height,
        alpha,
        color_space,
    )?)))
}

#[op2(fast)]
#[number]
pub fn op_canvas_2d_state_width(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> u64 {
    let this = this.borrow();
    this.width()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_width(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[number] width: u64,
) -> Result<(), Canvas2DError> {
    let mut this = this.borrow_mut();
    let height = this.height();
    this.reset(width, height)
}

#[op2(fast)]
#[number]
pub fn op_canvas_2d_state_height(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> u64 {
    let this = this.borrow();
    this.height()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_height(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[number] height: u64,
) -> Result<(), Canvas2DError> {
    let mut this = this.borrow_mut();
    let width = this.width();
    this.reset(width, height)
}

#[op2(fast)]
pub fn op_canvas_2d_state_save(#[cppgc] this: &Wrap<RefCell<CanvasState>>) {
    let mut this = this.borrow_mut();
    this.save()
}

#[op2(fast)]
pub fn op_canvas_2d_state_restore(#[cppgc] this: &Wrap<RefCell<CanvasState>>) {
    let mut this = this.borrow_mut();
    this.restore()
}

#[op2(fast)]
pub fn op_canvas_2d_state_reset(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
) -> Result<(), Canvas2DError> {
    let mut this = this.borrow_mut();
    let width = this.width();
    let height = this.height();
    this.reset(width, height)
}

#[op2(fast)]
pub fn op_canvas_2d_state_clear(#[cppgc] this: &Wrap<RefCell<CanvasState>>) {
    let mut this = this.borrow_mut();
    this.clear()
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_width(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.line_width()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_width(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: f64) {
    let mut this = this.borrow_mut();
    if value.is_finite() && value > 0.0 {
        this.set_line_width(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_cap(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.line_cap() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_cap(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: i32) {
    let mut this = this.borrow_mut();
    let value = CanvasLineCap::from_repr(value).unwrap();
    this.set_line_cap(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_join(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.line_join() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_join(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: i32) {
    let mut this = this.borrow_mut();
    let value = CanvasLineJoin::from_repr(value).unwrap();
    this.set_line_join(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_miter_limit(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.miter_limit()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_miter_limit(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: f64) {
    let mut this = this.borrow_mut();
    if value.is_finite() && value > 0.0 {
        this.set_miter_limit(value);
    }
}

#[op2]
pub fn op_canvas_2d_state_dash_list<'a>(
    scope: &mut v8::HandleScope<'a>,
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
) -> v8::Local<'a, v8::Array> {
    let this = this.borrow();
    let segments = this.dash_list();
    let mut elements = segments
        .iter()
        .map(|&value| v8::Number::new(scope, value).into())
        .collect::<Vec<_>>();
    if (elements.len() & 1) != 0 {
        elements.extend_from_within(..);
    }
    v8::Array::new_with_elements(scope, &elements)
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_dash_list(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[buffer] segments: &[f64],
) {
    let mut this = this.borrow_mut();
    this.set_dash_list(segments)
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_dash_offset(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.line_dash_offset()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_dash_offset(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: f64,
) {
    let mut this = this.borrow_mut();
    if value.is_finite() {
        this.set_line_dash_offset(value);
    }
}

#[op2]
#[string]
pub fn op_canvas_2d_state_font(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> String {
    let this = this.borrow();
    match this.font() {
        Some(v) => v.to_css_string(),
        None => "".to_owned(),
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_font(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(mut value) = ComputedFont::from_css_string(value) {
        value.line_height = ComputedLineHeight::Normal;
        this.set_font(value);
        true
    } else {
        false
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_text_align(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.text_align() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_text_align(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: i32) {
    let mut this = this.borrow_mut();
    let value = CanvasTextAlign::from_repr(value).unwrap();
    this.set_text_align(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_text_baseline(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.text_baseline() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_text_baseline(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: i32,
) {
    let mut this = this.borrow_mut();
    let value = CanvasTextBaseline::from_repr(value).unwrap();
    this.set_text_baseline(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_direction(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.direction() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_direction(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: i32) {
    let mut this = this.borrow_mut();
    let value = CanvasDirection::from_repr(value).unwrap();
    this.set_direction(value)
}

#[op2]
#[string]
pub fn op_canvas_2d_state_letter_spacing(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> String {
    let this = this.borrow();
    this.letter_spacing().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_letter_spacing(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(value) = SpecifiedAbsoluteLength::from_css_string(value) {
        this.set_letter_spacing(value);
        true
    } else {
        false
    }
}

#[op2]
#[string]
pub fn op_canvas_2d_state_word_spacing(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> String {
    let this = this.borrow();
    this.word_spacing().to_css_string()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_word_spacing(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(value) = SpecifiedAbsoluteLength::from_css_string(value) {
        this.set_word_spacing(value);
        true
    } else {
        false
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_font_kerning(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.font_kerning() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_font_kerning(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: i32) {
    let mut this = this.borrow_mut();
    let value = CanvasFontKerning::from_repr(value).unwrap();
    this.set_font_kerning(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_font_stretch(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.font_stretch() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_font_stretch(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: i32) {
    let mut this = this.borrow_mut();
    let value = CanvasFontStretch::from_repr(value).unwrap();
    this.set_font_stretch(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_font_variant_caps(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.font_variant_caps() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_font_variant_caps(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: i32,
) {
    let mut this = this.borrow_mut();
    let value = CanvasFontVariantCaps::from_repr(value).unwrap();
    this.set_font_variant_caps(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_text_rendering(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> i32 {
    let this = this.borrow();
    this.text_rendering() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_text_rendering(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: i32,
) {
    let mut this = this.borrow_mut();
    let value = CanvasTextRendering::from_repr(value).unwrap();
    this.set_text_rendering(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_scale(#[cppgc] this: &Wrap<RefCell<CanvasState>>, x: f64, y: f64) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        this.scale(x, y);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_rotate(#[cppgc] this: &Wrap<RefCell<CanvasState>>, radians: f64) {
    let mut this = this.borrow_mut();
    if radians.is_finite() {
        this.rotate(radians);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_translate(#[cppgc] this: &Wrap<RefCell<CanvasState>>, x: f64, y: f64) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        this.translate(x, y);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_transform(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let mut this = this.borrow_mut();
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.transform(Transform2D::new(a, b, c, d, e, f));
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_get_transform(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[buffer] out: &mut [f64],
) {
    let this = this.borrow();
    out.copy_from_slice(&this.get_transform().to_array())
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_transform(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let mut this = this.borrow_mut();
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.set_transform(Transform2D::new(a, b, c, d, e, f));
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_reset_transform(#[cppgc] this: &Wrap<RefCell<CanvasState>>) {
    let mut this = this.borrow_mut();
    this.reset_transform()
}

#[op2]
#[string]
pub fn op_canvas_2d_state_fill_style(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> Option<String> {
    let this = this.borrow();
    if let FillOrStrokeStyle::Color(color) = *this.fill_style() {
        Some(serialize_color_for_canvas(color))
    } else {
        None
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_fill_style_color(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(value) = ComputedColor::from_css_string(value) {
        this.set_fill_style(FillOrStrokeStyle::Color(resolve_color_for_canvas(value)));
        true
    } else {
        false
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_fill_style_pattern(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] value: &Wrap<Rc<CanvasPattern>>,
) {
    let mut this = this.borrow_mut();
    this.set_fill_style(FillOrStrokeStyle::Pattern((*value).clone()))
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_fill_style_gradient(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] value: &Wrap<Rc<CanvasGradient>>,
) {
    let mut this = this.borrow_mut();
    this.set_fill_style(FillOrStrokeStyle::Gradient((*value).clone()))
}

#[op2]
#[string]
pub fn op_canvas_2d_state_stroke_style(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
) -> Option<String> {
    let this = this.borrow();
    if let FillOrStrokeStyle::Color(color) = *this.stroke_style() {
        Some(serialize_color_for_canvas(color))
    } else {
        None
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_stroke_style_color(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(value) = ComputedColor::from_css_string(value) {
        this.set_stroke_style(FillOrStrokeStyle::Color(resolve_color_for_canvas(value)));
        true
    } else {
        false
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_stroke_style_pattern(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] value: &Wrap<Rc<CanvasPattern>>,
) {
    let mut this = this.borrow_mut();
    this.set_stroke_style(FillOrStrokeStyle::Pattern((*value).clone()))
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_stroke_style_gradient(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] value: &Wrap<Rc<CanvasGradient>>,
) {
    let mut this = this.borrow_mut();
    this.set_stroke_style(FillOrStrokeStyle::Gradient((*value).clone()))
}

#[op2(fast)]
pub fn op_canvas_2d_state_clear_rect(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let mut this = this.borrow_mut();
    if [x, y, width, height].into_iter().all(f64::is_finite) {
        this.clear_rect(x, y, width, height);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_fill_rect(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let mut this = this.borrow_mut();
    if [x, y, width, height].into_iter().all(f64::is_finite) {
        this.fill_rect(x, y, width, height);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_stroke_rect(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let mut this = this.borrow_mut();
    if [x, y, width, height].into_iter().all(f64::is_finite) {
        this.stroke_rect(x, y, width, height);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_fill_text(
    state: &OpState,
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] text: &str,
    x: f64,
    y: f64,
    max_width: f64,
) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        let fonts = state.borrow::<Rc<RefCell<FontFaceSet>>>().borrow();
        this.fill_text(&fonts, text, x, y, max_width);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_stroke_text(
    state: &OpState,
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] text: &str,
    x: f64,
    y: f64,
    max_width: f64,
) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        let fonts = state.borrow::<Rc<RefCell<FontFaceSet>>>().borrow();
        this.stroke_text(&fonts, text, x, y, max_width);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_measure_text(
    state: &OpState,
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] text: &str,
    #[buffer] out: &mut [f64],
) {
    let this = this.borrow();
    let out = &mut out[..12];
    let fonts = state.borrow::<Rc<RefCell<FontFaceSet>>>().borrow();
    let result = this.measure_text(&fonts, text);
    out[0] = result.width as f64;
    out[1] = result.actual_bounding_box_left as f64;
    out[2] = result.actual_bounding_box_right as f64;
    out[3] = result.font_bounding_box_ascent as f64;
    out[4] = result.font_bounding_box_descent as f64;
    out[5] = result.actual_bounding_box_ascent as f64;
    out[6] = result.actual_bounding_box_descent as f64;
    out[7] = result.em_height_ascent as f64;
    out[8] = result.em_height_descent as f64;
    out[9] = result.hanging_baseline as f64;
    out[10] = result.alphabetic_baseline as f64;
    out[11] = result.ideographic_baseline as f64;
}

#[op2(fast)]
pub fn op_canvas_2d_state_fill(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] path: &Wrap<RefCell<Path>>,
    fill_rule: i32,
) {
    let mut this = this.borrow_mut();
    let path = path.borrow();
    let fill_rule = CanvasFillRule::from_repr(fill_rule).unwrap();
    this.fill(&path, fill_rule)
}

#[op2(fast)]
pub fn op_canvas_2d_state_stroke(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] path: &Wrap<RefCell<Path>>,
) {
    let mut this = this.borrow_mut();
    let path = path.borrow();
    this.stroke(&path)
}

#[op2(fast)]
pub fn op_canvas_2d_state_clip(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] path: &Wrap<RefCell<Path>>,
    fill_rule: i32,
) {
    let mut this = this.borrow_mut();
    let path = path.borrow();
    let fill_rule = CanvasFillRule::from_repr(fill_rule).unwrap();
    this.clip(&path, fill_rule)
}

#[op2(fast)]
pub fn op_canvas_2d_state_is_point_in_path(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] path: &Wrap<RefCell<Path>>,
    x: f64,
    y: f64,
    fill_rule: i32,
) -> bool {
    let this = this.borrow();
    let path = path.borrow();
    let fill_rule = CanvasFillRule::from_repr(fill_rule).unwrap();
    [x, y].into_iter().all(f64::is_finite) && this.is_point_in_path(&path, x, y, fill_rule)
}

#[op2(fast)]
pub fn op_canvas_2d_state_is_point_in_stroke(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] path: &Wrap<RefCell<Path>>,
    x: f64,
    y: f64,
) -> bool {
    let this = this.borrow();
    let path = path.borrow();
    [x, y].into_iter().all(f64::is_finite) && this.is_point_in_stroke(&path, x, y)
}

#[op2(fast)]
pub fn op_canvas_2d_state_draw_image(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[cppgc] image: &Wrap<RefCell<ImageBitmap>>,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    dx: f64,
    dy: f64,
    dw: f64,
    dh: f64,
) -> Result<(), Canvas2DError> {
    let mut this = this.borrow_mut();
    let image = image.take();
    if [sx, sy, sw, sh, dx, dy, dw, dh]
        .into_iter()
        .all(f64::is_finite)
    {
        this.draw_image(image, sx, sy, sw, sh, dx, dy, dw, dh)?;
    }
    Ok(())
}

#[op2(fast)]
pub fn op_canvas_2d_state_get_image_data(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[buffer] dst_data: &mut [u32],
    dst_width: u32,
    dst_height: u32,
    dst_color_space: i32,
    #[number] x: i64,
    #[number] y: i64,
) -> Result<(), Canvas2DError> {
    let this = this.borrow();
    let dst = AlignedImageDataViewMut {
        width: dst_width,
        height: dst_height,
        color_space: CanvasColorSpace::from_repr(dst_color_space).unwrap(),
        data: dst_data,
    };
    this.get_image_data(dst, x, y)
}

#[op2(fast)]
pub fn op_canvas_2d_state_put_image_data(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[buffer] src_data: &[u32],
    src_width: u32,
    src_height: u32,
    src_color_space: i32,
    #[number] sx: i64,
    #[number] sy: i64,
    #[number] sw: u64,
    #[number] sh: u64,
    #[number] dx: i64,
    #[number] dy: i64,
) -> Result<(), Canvas2DError> {
    let mut this = this.borrow_mut();
    let src = AlignedImageDataView {
        width: src_width,
        height: src_height,
        color_space: CanvasColorSpace::from_repr(src_color_space).unwrap(),
        data: src_data,
    };
    this.put_image_data(src, sx, sy, sw, sh, dx, dy)
}

#[op2(fast)]
pub fn op_canvas_2d_state_global_alpha(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.global_alpha()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_global_alpha(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: f64) {
    let mut this = this.borrow_mut();
    if (0.0..=1.0).contains(&value) {
        this.set_global_alpha(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_global_composite_operation(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
) -> i32 {
    let this = this.borrow();
    this.global_composite_operation() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_global_composite_operation(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: i32,
) {
    let mut this = this.borrow_mut();
    let value = BlendOrCompositeMode::from_repr(value).unwrap();
    this.set_global_compositing_operator(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_image_smoothing_enabled(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
) -> bool {
    let this = this.borrow();
    this.image_smoothing_enabled()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_image_smoothing_enabled(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: bool,
) {
    let mut this = this.borrow_mut();
    this.set_image_smoothing_enabled(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_image_smoothing_quality(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
) -> i32 {
    let this = this.borrow();
    this.image_smoothing_quality() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_image_smoothing_quality(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: i32,
) {
    let mut this = this.borrow_mut();
    let value = ImageSmoothingQuality::from_repr(value).unwrap();
    this.set_image_smoothing_quality(value)
}

#[op2]
#[string]
pub fn op_canvas_2d_state_shadow_color(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> String {
    let this = this.borrow();
    serialize_color_for_canvas(this.shadow_color())
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_color(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(value) = ComputedColor::from_css_string(value) {
        this.set_shadow_color(resolve_color_for_canvas(value));
        true
    } else {
        false
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_shadow_offset_x(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.shadow_offset_x()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_offset_x(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: f64,
) {
    let mut this = this.borrow_mut();
    if value.is_finite() {
        this.set_shadow_offset_x(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_shadow_offset_y(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.shadow_offset_y()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_offset_y(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    value: f64,
) {
    let mut this = this.borrow_mut();
    if value.is_finite() {
        this.set_shadow_offset_y(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_shadow_blur(#[cppgc] this: &Wrap<RefCell<CanvasState>>) -> f64 {
    let this = this.borrow();
    this.shadow_blur()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_blur(#[cppgc] this: &Wrap<RefCell<CanvasState>>, value: f64) {
    let mut this = this.borrow_mut();
    if value.is_finite() && value >= 0.0 {
        this.set_shadow_blur(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_filter(
    #[cppgc] this: &Wrap<RefCell<CanvasState>>,
    #[string] value: &str,
) -> bool {
    let mut this = this.borrow_mut();
    if let Ok(value) = ComputedFilter::from_css_string(value) {
        this.set_filter(value);
        true
    } else {
        false
    }
}
