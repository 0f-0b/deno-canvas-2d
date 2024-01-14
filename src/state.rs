use std::array;
use std::convert::Infallible;
use std::ffi::c_void;
use std::fmt::{self, Debug};
use std::ops::{Residual, Try};
use std::rc::Rc;

use deno_runtime::deno_core::{self, anyhow, op2, v8, OpState};
use euclid::default::{Box2D, Point2D, Transform2D, Vector2D};
use euclid::{point2, size2, vec2, Angle};
use strum_macros::FromRepr;

use super::blur::GaussianBlur;
use super::convert::{
    display_p3_to_premultiplied_linear_srgb, pack_rgba8_to_argb32,
    premultiplied_linear_display_p3_to_srgb, premultiplied_linear_srgb_to_display_p3,
    premultiplied_linear_srgb_to_srgb, srgb_to_premultiplied_linear_display_p3,
    srgb_to_premultiplied_linear_srgb, unpack_argb32_to_rgba8,
};
use super::css_color::AbsoluteColor;
use super::gc::{borrow_v8, borrow_v8_mut, from_v8, into_v8};
use super::gradient::CanvasGradient;
use super::image_bitmap::ImageBitmap;
use super::image_data::{BorrowedImageData, BorrowedMutImageData};
use super::path::{CanvasFillRule, Path};
use super::pattern::CanvasPattern;
use super::{
    parse_color_for_canvas, premultiply, raqote_ext, serialize_color_for_canvas, to_raqote_color,
    to_raqote_point, to_raqote_size, to_raqote_solid_source, CanvasColorSpace,
};

const TRANSPARENT_SOLID_SOURCE: raqote::SolidSource = raqote::SolidSource {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub(super) enum CanvasLineCap {
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
pub(super) enum CanvasLineJoin {
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
pub(super) enum BlendOrCompositeMode {
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
    pub fn to_raqote(self) -> raqote::BlendMode {
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
            Self::Clear => raqote::BlendMode::Clear,
            Self::Copy => raqote::BlendMode::Src,
            Self::SourceOver => raqote::BlendMode::SrcOver,
            Self::DestinationOver => raqote::BlendMode::DstOver,
            Self::SourceIn => raqote::BlendMode::SrcIn,
            Self::DestinationIn => raqote::BlendMode::DstIn,
            Self::SourceOut => raqote::BlendMode::SrcOut,
            Self::DestinationOut => raqote::BlendMode::DstOut,
            Self::SourceAtop => raqote::BlendMode::SrcAtop,
            Self::DestinationAtop => raqote::BlendMode::DstAtop,
            Self::Xor => raqote::BlendMode::Xor,
            Self::Lighter => raqote::BlendMode::Add,
            Self::PlusDarker => unimplemented!(),
            Self::PlusLighter => raqote::BlendMode::Add,
        }
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub(super) enum ImageSmoothingQuality {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug)]
pub(super) enum FillOrStrokeStyle {
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
pub(super) struct DrawingState {
    line_width: f64,
    line_cap: CanvasLineCap,
    line_join: CanvasLineJoin,
    miter_limit: f64,
    dash_list: Box<[f64]>,
    line_dash_offset: f64,
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
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            line_width: 1.0,
            line_cap: CanvasLineCap::Butt,
            line_join: CanvasLineJoin::Miter,
            miter_limit: 10.0,
            dash_list: Box::new([]),
            line_dash_offset: 0.0,
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
            dash_array: self.dash_list.iter().map(|&x| x as f32).collect(),
            dash_offset: self.line_dash_offset as f32,
        }
    }
}

pub(super) struct CanvasState {
    draw_target: raqote::DrawTarget,
    color_space: CanvasColorSpace,
    current_drawing_state: DrawingState,
    drawing_state_stack: Vec<DrawingState>,
}

impl Debug for CanvasState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CanvasState")
            .field("color_space", &self.color_space)
            .field("current_drawing_state", &self.current_drawing_state)
            .field("drawing_state_stack", &self.drawing_state_stack)
            .finish_non_exhaustive()
    }
}

impl CanvasState {
    pub fn new(width: u64, height: u64, color_space: CanvasColorSpace) -> anyhow::Result<Self> {
        let size = to_raqote_size(width, height)?;
        Ok(CanvasState {
            draw_target: raqote::DrawTarget::new(size.width, size.height),
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

    pub fn reset(&mut self, width: u64, height: u64) -> anyhow::Result<()> {
        *self = Self::new(width, height, self.color_space)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.draw_target.get_data_mut().fill(0);
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
        &self.current_drawing_state.dash_list
    }

    pub fn set_dash_list(&mut self, segments: Box<[f64]>) {
        self.current_drawing_state.dash_list = segments;
    }

    pub fn line_dash_offset(&self) -> f64 {
        self.current_drawing_state.line_dash_offset
    }

    pub fn set_line_dash_offset(&mut self, value: f64) {
        self.current_drawing_state.line_dash_offset = value;
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

    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.current_drawing_state.transformation_matrix = Transform2D::new(a, b, c, d, e, f)
            .then(&self.current_drawing_state.transformation_matrix);
        self.update_transform();
    }

    pub fn get_transform(&self) -> Transform2D<f64> {
        self.current_drawing_state.transformation_matrix
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.current_drawing_state.transformation_matrix = Transform2D::new(a, b, c, d, e, f);
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
        const DUMMY_SOURCE: raqote::Source = raqote::Source::Solid(TRANSPARENT_SOLID_SOURCE);
        self.draw_target.fill_rect(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
            &DUMMY_SOURCE,
            &raqote::DrawOptions {
                blend_mode: raqote::BlendMode::Clear,
                ..Default::default()
            },
        );
    }

    fn paint<RF>(&mut self, prepare: impl FnOnce(&Self) -> RF)
    where
        RF: Fn(&mut raqote::DrawTarget, raqote::DrawOptions),
    {
        self.try_paint(move |this| Ok::<_, Infallible>(prepare(this)))
            .unwrap()
    }

    fn try_paint<R: Try>(
        &mut self,
        prepare: impl FnOnce(&Self) -> R,
    ) -> <R::Residual as Residual<()>>::TryType
    where
        R::Output: Fn(&mut raqote::DrawTarget, raqote::DrawOptions),
        R::Residual: Residual<()>,
    {
        let draw_shadow = |this: &mut Self, render: &R::Output| {
            let offset = this.current_drawing_state.shadow_offset;
            let blur = GaussianBlur::new((this.current_drawing_state.shadow_blur * 0.5) as f32);
            if offset.x == 0.0 && offset.y == 0.0 && blur.is_none() {
                return;
            }
            let colors: [u32; 256] = {
                let c = to_raqote_color(this.current_drawing_state.shadow_color, this.color_space);
                if c.a() == 0 {
                    return;
                }
                array::from_fn(|a| {
                    let a = a as u8;
                    u32::from_be_bytes([
                        a,
                        premultiply(c.r(), a),
                        premultiply(c.g(), a),
                        premultiply(c.b(), a),
                    ])
                })
            };
            let width = this.draw_target.width();
            let height = this.draw_target.height();
            let blur_radius = match blur {
                Some(ref blur) => blur.radius(),
                None => 0,
            };
            let (shadow_width, shadow_height) = {
                let extend_len = (blur_radius * 2).try_into().unwrap();
                (
                    width.checked_add(extend_len).unwrap(),
                    height.checked_add(extend_len).unwrap(),
                )
            };
            let mut shadow_target = raqote::DrawTarget::new(shadow_width, shadow_height);
            shadow_target.set_transform(&this.draw_target.get_transform().then_translate(vec2(
                offset.x as f32 + blur_radius as f32,
                offset.y as f32 + blur_radius as f32,
            )));
            render(
                &mut shadow_target,
                raqote::DrawOptions {
                    blend_mode: raqote::BlendMode::Src,
                    alpha: this.current_drawing_state.global_alpha as f32
                        * this.current_drawing_state.shadow_color.alpha,
                    ..Default::default()
                },
            );
            match blur {
                Some(blur) => {
                    let mut data = shadow_target
                        .into_inner()
                        .into_iter()
                        .map(|pixel| (pixel >> 24) as f32)
                        .collect::<Vec<_>>();
                    blur.apply(&mut data, shadow_width as usize);
                    shadow_target = raqote::DrawTarget::from_backing(
                        shadow_width,
                        shadow_height,
                        data.into_iter()
                            .map(|a| colors[a.round() as u8 as usize])
                            .collect::<Vec<_>>(),
                    );
                }
                None => {
                    for pixel in shadow_target.get_data_mut() {
                        *pixel = colors[(*pixel >> 24) as usize];
                    }
                }
            }
            this.draw_target.blend_surface(
                &shadow_target,
                Box2D::from_size(size2(width, height)),
                point2(0, 0),
                this.current_drawing_state
                    .compositing_and_blending_operator
                    .to_raqote(),
            );
        };
        match self.current_drawing_state.compositing_and_blending_operator {
            BlendOrCompositeMode::Clear => self.draw_target.clear(TRANSPARENT_SOLID_SOURCE),
            BlendOrCompositeMode::Copy => {
                let render = prepare(self)?;
                self.draw_target.clear(TRANSPARENT_SOLID_SOURCE);
                render(
                    &mut self.draw_target,
                    raqote::DrawOptions {
                        blend_mode: self
                            .current_drawing_state
                            .compositing_and_blending_operator
                            .to_raqote(),
                        alpha: self.current_drawing_state.global_alpha as f32,
                        ..Default::default()
                    },
                );
            }
            BlendOrCompositeMode::SourceIn
            | BlendOrCompositeMode::DestinationIn
            | BlendOrCompositeMode::SourceOut
            | BlendOrCompositeMode::DestinationAtop => {
                let render = prepare(self)?;
                draw_shadow(self, &render);
                self.draw_target.push_layer_with_blend(
                    1.0,
                    self.current_drawing_state
                        .compositing_and_blending_operator
                        .to_raqote(),
                );
                render(
                    &mut self.draw_target,
                    raqote::DrawOptions {
                        blend_mode: raqote::BlendMode::Src,
                        alpha: self.current_drawing_state.global_alpha as f32,
                        ..Default::default()
                    },
                );
                self.draw_target.pop_layer();
            }
            _ => {
                let render = prepare(self)?;
                draw_shadow(self, &render);
                render(
                    &mut self.draw_target,
                    raqote::DrawOptions {
                        blend_mode: self
                            .current_drawing_state
                            .compositing_and_blending_operator
                            .to_raqote(),
                        alpha: self.current_drawing_state.global_alpha as f32,
                        ..Default::default()
                    },
                );
            }
        }
        Try::from_output(())
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
    ) -> anyhow::Result<()> {
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
        mut dst: BorrowedMutImageData,
        x: i64,
        y: i64,
    ) -> anyhow::Result<()> {
        let dst_color_space = dst.color_space;
        let mut dst = dst.as_raqote_surface_rgba8()?;
        let origin = to_raqote_point(x, y)?;
        dst.composite_surface(
            &self.draw_target,
            Box2D::from_origin_and_size(origin, size2(dst.width(), dst.height())),
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
        src: BorrowedImageData,
        sx: i64,
        sy: i64,
        sw: u64,
        sh: u64,
        dx: i64,
        dy: i64,
    ) -> anyhow::Result<()> {
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
}

#[op2]
pub fn op_canvas_2d_state_new<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    #[number] width: u64,
    #[number] height: u64,
    color_space: i32,
) -> anyhow::Result<v8::Local<'a, v8::External>> {
    let color_space = CanvasColorSpace::from_repr(color_space).unwrap();
    let result = CanvasState::new(width, height, color_space)?;
    Ok(into_v8(state, scope, result))
}

#[op2(fast)]
#[number]
pub fn op_canvas_2d_state_width(state: &OpState, this: *const c_void) -> u64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.width()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_width(
    state: &OpState,
    this: *const c_void,
    #[number] width: u64,
) -> anyhow::Result<()> {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let height = this.height();
    this.reset(width, height)
}

#[op2(fast)]
#[number]
pub fn op_canvas_2d_state_height(state: &OpState, this: *const c_void) -> u64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.height()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_height(
    state: &OpState,
    this: *const c_void,
    #[number] height: u64,
) -> anyhow::Result<()> {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let width = this.width();
    this.reset(width, height)
}

#[op2(fast)]
pub fn op_canvas_2d_state_save(state: &OpState, this: *const c_void) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    this.save()
}

#[op2(fast)]
pub fn op_canvas_2d_state_restore(state: &OpState, this: *const c_void) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    this.restore()
}

#[op2(fast)]
pub fn op_canvas_2d_state_reset(state: &OpState, this: *const c_void) -> anyhow::Result<()> {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let width = this.width();
    let height = this.height();
    this.reset(width, height)
}

#[op2(fast)]
pub fn op_canvas_2d_state_clear(state: &OpState, this: *const c_void) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    this.clear()
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_width(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.line_width()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_width(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if value.is_finite() && value > 0.0 {
        this.set_line_width(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_cap(state: &OpState, this: *const c_void) -> i32 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.line_cap() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_cap(state: &OpState, this: *const c_void, value: i32) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = CanvasLineCap::from_repr(value).unwrap();
    this.set_line_cap(value);
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_join(state: &OpState, this: *const c_void) -> i32 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.line_join() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_join(state: &OpState, this: *const c_void, value: i32) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = CanvasLineJoin::from_repr(value).unwrap();
    this.set_line_join(value);
}

#[op2(fast)]
pub fn op_canvas_2d_state_miter_limit(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.miter_limit()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_miter_limit(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if value.is_finite() && value > 0.0 {
        this.set_miter_limit(value);
    }
}

#[op2]
pub fn op_canvas_2d_state_dash_list<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    this: *const c_void,
) -> v8::Local<'a, v8::Array> {
    let this = borrow_v8::<CanvasState>(state, this);
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
    state: &OpState,
    this: *const c_void,
    #[buffer] segments: &[f64],
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    this.set_dash_list(segments.into())
}

#[op2(fast)]
pub fn op_canvas_2d_state_line_dash_offset(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.line_dash_offset()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_line_dash_offset(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if value.is_finite() {
        this.set_line_dash_offset(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_scale(state: &OpState, this: *const c_void, x: f64, y: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [x, y].into_iter().all(f64::is_finite) {
        this.scale(x, y);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_rotate(state: &OpState, this: *const c_void, radians: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if radians.is_finite() {
        this.rotate(radians);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_translate(state: &OpState, this: *const c_void, x: f64, y: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [x, y].into_iter().all(f64::is_finite) {
        this.translate(x, y);
    }
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_state_transform(
    state: &OpState,
    this: *const c_void,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.transform(a, b, c, d, e, f);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_get_transform(
    state: &OpState,
    this: *const c_void,
    #[buffer] out: &mut [f64],
) {
    let this = borrow_v8::<CanvasState>(state, this);
    out.copy_from_slice(&this.get_transform().to_array())
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_state_set_transform(
    state: &OpState,
    this: *const c_void,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.set_transform(a, b, c, d, e, f);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_reset_transform(state: &OpState, this: *const c_void) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    this.reset_transform()
}

#[op2]
#[string]
pub fn op_canvas_2d_state_fill_style(state: &OpState, this: *const c_void) -> Option<String> {
    let this = borrow_v8::<CanvasState>(state, this);
    if let FillOrStrokeStyle::Color(color) = *this.fill_style() {
        Some(serialize_color_for_canvas(color))
    } else {
        None
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_fill_style_color(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if let Ok(value) = parse_color_for_canvas(value) {
        this.set_fill_style(FillOrStrokeStyle::Color(value));
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_fill_style_pattern(
    state: &OpState,
    this: *const c_void,
    value: *const c_void,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = borrow_v8::<Rc<CanvasPattern>>(state, value);
    this.set_fill_style(FillOrStrokeStyle::Pattern(value.clone()))
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_fill_style_gradient(
    state: &OpState,
    this: *const c_void,
    value: *const c_void,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = borrow_v8::<Rc<CanvasGradient>>(state, value);
    this.set_fill_style(FillOrStrokeStyle::Gradient(value.clone()))
}

#[op2]
#[string]
pub fn op_canvas_2d_state_stroke_style(state: &OpState, this: *const c_void) -> Option<String> {
    let this = borrow_v8::<CanvasState>(state, this);
    if let FillOrStrokeStyle::Color(color) = *this.stroke_style() {
        Some(serialize_color_for_canvas(color))
    } else {
        None
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_stroke_style_color(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if let Ok(value) = parse_color_for_canvas(value) {
        this.set_stroke_style(FillOrStrokeStyle::Color(value));
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_stroke_style_pattern(
    state: &OpState,
    this: *const c_void,
    value: *const c_void,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = borrow_v8::<Rc<CanvasPattern>>(state, value);
    this.set_stroke_style(FillOrStrokeStyle::Pattern(value.clone()))
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_stroke_style_gradient(
    state: &OpState,
    this: *const c_void,
    value: *const c_void,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = borrow_v8::<Rc<CanvasGradient>>(state, value);
    this.set_stroke_style(FillOrStrokeStyle::Gradient(value.clone()))
}

#[op2(fast)]
pub fn op_canvas_2d_state_clear_rect(
    state: &OpState,
    this: *const c_void,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [x, y, width, height].into_iter().all(f64::is_finite) {
        this.clear_rect(x, y, width, height);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_fill_rect(
    state: &OpState,
    this: *const c_void,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [x, y, width, height].into_iter().all(f64::is_finite) {
        this.fill_rect(x, y, width, height);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_stroke_rect(
    state: &OpState,
    this: *const c_void,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if [x, y, width, height].into_iter().all(f64::is_finite) {
        this.stroke_rect(x, y, width, height);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_fill(
    state: &OpState,
    this: *const c_void,
    path: *const c_void,
    fill_rule: i32,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let path = borrow_v8::<Path>(state, path);
    let fill_rule = CanvasFillRule::from_repr(fill_rule).unwrap();
    this.fill(&path, fill_rule);
}

#[op2(fast)]
pub fn op_canvas_2d_state_stroke(state: &OpState, this: *const c_void, path: *const c_void) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let path = borrow_v8::<Path>(state, path);
    this.stroke(&path);
}

#[op2(fast)]
pub fn op_canvas_2d_state_clip(
    state: &OpState,
    this: *const c_void,
    path: *const c_void,
    fill_rule: i32,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let path = borrow_v8::<Path>(state, path);
    let fill_rule = CanvasFillRule::from_repr(fill_rule).unwrap();
    this.clip(&path, fill_rule);
}

#[op2(fast)]
pub fn op_canvas_2d_state_is_point_in_path(
    state: &OpState,
    this: *const c_void,
    path: *const c_void,
    x: f64,
    y: f64,
    fill_rule: i32,
) -> bool {
    let this = borrow_v8::<CanvasState>(state, this);
    let path = borrow_v8::<Path>(state, path);
    let fill_rule = CanvasFillRule::from_repr(fill_rule).unwrap();
    [x, y].into_iter().all(f64::is_finite) && this.is_point_in_path(&path, x, y, fill_rule)
}

#[op2(fast)]
pub fn op_canvas_2d_state_is_point_in_stroke(
    state: &OpState,
    this: *const c_void,
    path: *const c_void,
    x: f64,
    y: f64,
) -> bool {
    let this = borrow_v8::<CanvasState>(state, this);
    let path = borrow_v8::<Path>(state, path);
    [x, y].into_iter().all(f64::is_finite) && this.is_point_in_stroke(&path, x, y)
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_state_draw_image(
    state: &OpState,
    this: *const c_void,
    image: *const c_void,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    dx: f64,
    dy: f64,
    dw: f64,
    dh: f64,
) -> anyhow::Result<()> {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let image = from_v8::<ImageBitmap>(state, image);
    if [sx, sy, sw, sh, dx, dy, dw, dh]
        .into_iter()
        .all(f64::is_finite)
    {
        this.draw_image(image, sx, sy, sw, sh, dx, dy, dw, dh)?;
    }
    Ok(())
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_state_get_image_data(
    state: &OpState,
    this: *const c_void,
    #[buffer] dst_data: &mut [u32],
    dst_width: u32,
    dst_height: u32,
    dst_color_space: i32,
    #[number] x: i64,
    #[number] y: i64,
) -> anyhow::Result<()> {
    let this = borrow_v8::<CanvasState>(state, this);
    let dst = BorrowedMutImageData {
        width: dst_width,
        height: dst_height,
        color_space: CanvasColorSpace::from_repr(dst_color_space).unwrap(),
        data: dst_data,
    };
    this.get_image_data(dst, x, y)
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_state_put_image_data(
    state: &OpState,
    this: *const c_void,
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
) -> anyhow::Result<()> {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let src = BorrowedImageData {
        width: src_width,
        height: src_height,
        color_space: CanvasColorSpace::from_repr(src_color_space).unwrap(),
        data: src_data,
    };
    this.put_image_data(src, sx, sy, sw, sh, dx, dy)
}

#[op2(fast)]
pub fn op_canvas_2d_state_global_alpha(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.global_alpha()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_global_alpha(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if (0.0..=1.0).contains(&value) {
        this.set_global_alpha(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_global_composite_operation(state: &OpState, this: *const c_void) -> i32 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.global_composite_operation() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_global_composite_operation(
    state: &OpState,
    this: *const c_void,
    value: i32,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = BlendOrCompositeMode::from_repr(value).unwrap();
    this.set_global_compositing_operator(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_image_smoothing_enabled(state: &OpState, this: *const c_void) -> bool {
    let this = borrow_v8::<CanvasState>(state, this);
    this.image_smoothing_enabled()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_image_smoothing_enabled(
    state: &OpState,
    this: *const c_void,
    value: bool,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    this.set_image_smoothing_enabled(value)
}

#[op2(fast)]
pub fn op_canvas_2d_state_image_smoothing_quality(state: &OpState, this: *const c_void) -> i32 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.image_smoothing_quality() as i32
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_image_smoothing_quality(
    state: &OpState,
    this: *const c_void,
    value: i32,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    let value = ImageSmoothingQuality::from_repr(value).unwrap();
    this.set_image_smoothing_quality(value)
}

#[op2]
#[string]
pub fn op_canvas_2d_state_shadow_color(state: &OpState, this: *const c_void) -> String {
    let this = borrow_v8::<CanvasState>(state, this);
    serialize_color_for_canvas(this.shadow_color())
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_color(
    state: &OpState,
    this: *const c_void,
    #[string] value: &str,
) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if let Ok(value) = parse_color_for_canvas(value) {
        this.set_shadow_color(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_shadow_offset_x(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.shadow_offset_x()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_offset_x(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if value.is_finite() {
        this.set_shadow_offset_x(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_shadow_offset_y(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.shadow_offset_y()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_offset_y(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if value.is_finite() {
        this.set_shadow_offset_y(value);
    }
}

#[op2(fast)]
pub fn op_canvas_2d_state_shadow_blur(state: &OpState, this: *const c_void) -> f64 {
    let this = borrow_v8::<CanvasState>(state, this);
    this.shadow_blur()
}

#[op2(fast)]
pub fn op_canvas_2d_state_set_shadow_blur(state: &OpState, this: *const c_void, value: f64) {
    let mut this = borrow_v8_mut::<CanvasState>(state, this);
    if value.is_finite() && value >= 0.0 {
        this.set_shadow_blur(value);
    }
}
