use std::array;
use std::iter::Sum;
use std::ops::{Add, Mul};

use euclid::default::SideOffsets2D;
use palette::stimulus::IntoStimulus as _;

use super::convert::{transform_argb32, Rgba};
use super::css::filter::{ComputedFilter, ComputedFilterFunction, ComputedFilterValue};
use super::{premultiply, resolve_color_for_canvas, to_raqote_color, CanvasColorSpace};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub fn from_u8((r, g, b, a): Rgba) -> Self {
        Self {
            r: r.into_stimulus(),
            g: g.into_stimulus(),
            b: b.into_stimulus(),
            a: a.into_stimulus(),
        }
    }

    pub fn to_u8(self) -> Rgba {
        (
            self.r.into_stimulus(),
            self.g.into_stimulus(),
            self.b.into_stimulus(),
            self.a.into_stimulus(),
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Mul<&f32> for Color {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self::Output {
        self * *rhs
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::TRANSPARENT, Add::add)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColorMatrix {
    pub a00: f32,
    pub a01: f32,
    pub a02: f32,
    pub a03: f32,
    pub a10: f32,
    pub a11: f32,
    pub a12: f32,
    pub a13: f32,
    pub a20: f32,
    pub a21: f32,
    pub a22: f32,
    pub a23: f32,
    pub a30: f32,
    pub a31: f32,
    pub a32: f32,
    pub a33: f32,
}

impl ColorMatrix {
    pub fn is_identity(self) -> bool {
        self.a00 == 1.0
            && self.a01 == 0.0
            && self.a02 == 0.0
            && self.a03 == 0.0
            && self.a10 == 0.0
            && self.a11 == 1.0
            && self.a12 == 0.0
            && self.a13 == 0.0
            && self.a20 == 0.0
            && self.a21 == 0.0
            && self.a22 == 1.0
            && self.a23 == 0.0
            && self.a30 == 0.0
            && self.a31 == 0.0
            && self.a32 == 0.0
            && self.a33 == 1.0
    }

    pub fn needs_clamp(self) -> bool {
        self.a00.min(0.0) + self.a01.min(0.0) + self.a02.min(0.0) + self.a03.min(0.0) < 0.0
            || self.a10.min(0.0) + self.a11.min(0.0) + self.a12.min(0.0) + self.a13.min(0.0) < 0.0
            || self.a20.min(0.0) + self.a21.min(0.0) + self.a22.min(0.0) + self.a23.min(0.0) < 0.0
            || self.a30.min(0.0) + self.a31.min(0.0) + self.a32.min(0.0) + self.a33.min(0.0) < 0.0
            || self.a00.max(0.0) + self.a01.max(0.0) + self.a02.max(0.0) + self.a03.max(0.0) > 1.0
            || self.a10.max(0.0) + self.a11.max(0.0) + self.a12.max(0.0) + self.a13.max(0.0) > 1.0
            || self.a20.max(0.0) + self.a21.max(0.0) + self.a22.max(0.0) + self.a23.max(0.0) > 1.0
            || self.a30.max(0.0) + self.a31.max(0.0) + self.a32.max(0.0) + self.a33.max(0.0) > 1.0
    }

    pub fn then(self, m: Self) -> Self {
        Self {
            a00: self.a00 * m.a00 + self.a10 * m.a01 + self.a20 * m.a02 + self.a30 * m.a03,
            a01: self.a01 * m.a00 + self.a11 * m.a01 + self.a21 * m.a02 + self.a31 * m.a03,
            a02: self.a02 * m.a00 + self.a12 * m.a01 + self.a22 * m.a02 + self.a32 * m.a03,
            a03: self.a03 * m.a00 + self.a13 * m.a01 + self.a23 * m.a02 + self.a33 * m.a03,
            a10: self.a00 * m.a10 + self.a10 * m.a11 + self.a20 * m.a12 + self.a30 * m.a13,
            a11: self.a01 * m.a10 + self.a11 * m.a11 + self.a21 * m.a12 + self.a31 * m.a13,
            a12: self.a02 * m.a10 + self.a12 * m.a11 + self.a22 * m.a12 + self.a32 * m.a13,
            a13: self.a03 * m.a10 + self.a13 * m.a11 + self.a23 * m.a12 + self.a33 * m.a13,
            a20: self.a00 * m.a20 + self.a10 * m.a21 + self.a20 * m.a22 + self.a30 * m.a23,
            a21: self.a01 * m.a20 + self.a11 * m.a21 + self.a21 * m.a22 + self.a31 * m.a23,
            a22: self.a02 * m.a20 + self.a12 * m.a21 + self.a22 * m.a22 + self.a32 * m.a23,
            a23: self.a03 * m.a20 + self.a13 * m.a21 + self.a23 * m.a22 + self.a33 * m.a23,
            a30: self.a00 * m.a30 + self.a10 * m.a31 + self.a20 * m.a32 + self.a30 * m.a33,
            a31: self.a01 * m.a30 + self.a11 * m.a31 + self.a21 * m.a32 + self.a31 * m.a33,
            a32: self.a02 * m.a30 + self.a12 * m.a31 + self.a22 * m.a32 + self.a32 * m.a33,
            a33: self.a03 * m.a30 + self.a13 * m.a31 + self.a23 * m.a32 + self.a33 * m.a33,
        }
    }

    pub fn apply(self, c: Color) -> Color {
        Color {
            r: c.r * self.a00 + c.g * self.a01 + c.b * self.a02 + c.a * self.a03,
            g: c.r * self.a10 + c.g * self.a11 + c.b * self.a12 + c.a * self.a13,
            b: c.r * self.a20 + c.g * self.a21 + c.b * self.a22 + c.a * self.a23,
            a: c.r * self.a30 + c.g * self.a31 + c.b * self.a32 + c.a * self.a33,
        }
    }
}

fn gaussian(x: f32, sigma: f32) -> f32 {
    let inv_sigma = sigma.recip();
    0.3989423 * inv_sigma * (-0.5 * (x * x) * (inv_sigma * inv_sigma)).exp()
}

fn gaussian_blur_radius(sigma: f32) -> usize {
    (3.0 * sigma).ceil() as usize
}

fn gaussian_kernel(sigma: f32) -> Vec<f32> {
    let m = gaussian_blur_radius(sigma);
    let mut v = vec![0.0; m * 2 + 1];
    for i in 0..=m {
        let w = gaussian(i as f32, sigma);
        v[m + i] = w;
        v[m - i] = w;
    }
    let scale = v.iter().sum::<f32>().recip();
    for w in &mut v {
        *w *= scale;
    }
    v
}

fn make_shadow(
    draw_target: &raqote::DrawTarget,
    color: raqote::Color,
    blur: f32,
) -> (raqote::DrawTarget, usize) {
    let colors: [u32; 256] = array::from_fn(|a| {
        let a = premultiply(a as u8, color.a());
        u32::from_be_bytes([
            a,
            premultiply(color.r(), a),
            premultiply(color.g(), a),
            premultiply(color.b(), a),
        ])
    });
    let overflow = gaussian_blur_radius(blur);
    if overflow == 0 {
        let result = raqote::DrawTarget::from_backing(
            draw_target.width(),
            draw_target.height(),
            draw_target
                .get_data()
                .iter()
                .map(|pixel| colors[(pixel >> 24) as usize])
                .collect(),
        );
        return (result, 0);
    }
    let kernel = gaussian_kernel(blur);
    let width_with_overflow = draw_target.width() as usize;
    let height_with_overflow = draw_target.height() as usize;
    let width = width_with_overflow.checked_sub(overflow * 2).unwrap();
    let height = height_with_overflow.checked_sub(overflow * 2).unwrap();
    let mut result = raqote::DrawTarget::new(width as i32, height as i32);
    let mut buf = vec![0.0; width * height_with_overflow];
    let src = draw_target.get_data();
    let dst = result.get_data_mut();
    for y in 0..height_with_overflow {
        for x in 0..width {
            let alpha = kernel
                .iter()
                .enumerate()
                .map(|(i, w)| (src[y * width_with_overflow + (x + i)] >> 24) as f32 * w)
                .sum::<f32>();
            buf[y * width + x] = alpha;
        }
    }
    for y in 0..height {
        for x in 0..width {
            let alpha = kernel
                .iter()
                .enumerate()
                .map(|(i, w)| buf[(y + i) * width + x] * w)
                .sum::<f32>();
            dst[y * width + x] = colors[alpha.round() as u8 as usize];
        }
    }
    (result, overflow)
}

fn apply_blur(draw_target: &raqote::DrawTarget, blur: f32) -> (raqote::DrawTarget, usize) {
    let overflow = gaussian_blur_radius(blur);
    if overflow == 0 {
        let result = raqote::DrawTarget::from_backing(
            draw_target.width(),
            draw_target.height(),
            draw_target.get_data().to_owned(),
        );
        return (result, 0);
    }
    let kernel = gaussian_kernel(blur);
    let width_with_overflow = draw_target.width() as usize;
    let height_with_overflow = draw_target.height() as usize;
    let width = width_with_overflow.checked_sub(overflow * 2).unwrap();
    let height = height_with_overflow.checked_sub(overflow * 2).unwrap();
    let mut result = raqote::DrawTarget::new(width as i32, height as i32);
    let mut buf = vec![Color::TRANSPARENT; width * height_with_overflow];
    let src = draw_target.get_data();
    let dst = result.get_data_mut();
    for y in 0..height_with_overflow {
        for x in 0..width {
            let alpha = kernel
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    let [a, r, g, b] = src[y * width_with_overflow + (x + i)].to_be_bytes();
                    Color::from_u8((r, g, b, a)) * w
                })
                .sum::<Color>();
            buf[y * width + x] = alpha;
        }
    }
    for y in 0..height {
        for x in 0..width {
            let alpha = kernel
                .iter()
                .enumerate()
                .map(|(i, w)| buf[(y + i) * width + x] * w)
                .sum::<Color>();
            let (r, g, b, a) = alpha.to_u8();
            dst[y * width + x] = u32::from_be_bytes([a, r, g, b]);
        }
    }
    (result, overflow)
}

#[derive(Clone, Copy, Debug)]
pub enum FilterInstruction {
    NoOp,
    ColorMatrix(ColorMatrix),
    Blur(f32),
    MakeShadow {
        color: raqote::Color,
        blur: f32,
    },
    DropShadow {
        color: raqote::Color,
        offset_x: f32,
        offset_y: f32,
        blur: f32,
    },
}

impl FilterInstruction {
    pub fn overflow(&self) -> SideOffsets2D<usize> {
        match *self {
            Self::NoOp | Self::ColorMatrix(_) => SideOffsets2D::zero(),
            Self::Blur(blur) | Self::MakeShadow { blur, .. } | Self::DropShadow { blur, .. } => {
                SideOffsets2D::new_all_same(gaussian_blur_radius(blur))
            }
        }
    }

    pub fn apply(&self, draw_target: &mut raqote::DrawTarget) {
        match *self {
            FilterInstruction::NoOp => {}
            FilterInstruction::ColorMatrix(mat) => {
                transform_argb32(draw_target.get_data_mut(), |c| {
                    mat.apply(Color::from_u8(c)).to_u8()
                });
            }
            FilterInstruction::Blur(blur) => {
                (*draw_target, _) = apply_blur(draw_target, blur);
            }
            FilterInstruction::MakeShadow { color, blur } => {
                (*draw_target, _) = make_shadow(draw_target, color, blur);
            }
            FilterInstruction::DropShadow {
                color,
                offset_x,
                offset_y,
                blur,
            } => {
                let (shadow, overflow) = make_shadow(draw_target, color, blur);
                let mut result = raqote::DrawTarget::new(shadow.width(), shadow.height());
                result.fill_rect(
                    0.0,
                    0.0,
                    result.width() as f32,
                    result.height() as f32,
                    &raqote::Source::Image(
                        raqote::Image {
                            width: shadow.width(),
                            height: shadow.height(),
                            data: shadow.get_data(),
                        },
                        raqote::ExtendMode::Pad,
                        raqote::FilterMode::Nearest,
                        raqote::Transform::translation(-offset_x, -offset_y),
                        true,
                        true,
                    ),
                    &Default::default(),
                );
                result.fill_rect(
                    0.0,
                    0.0,
                    result.width() as f32,
                    result.height() as f32,
                    &raqote::Source::Image(
                        raqote::Image {
                            width: draw_target.width(),
                            height: draw_target.height(),
                            data: draw_target.get_data(),
                        },
                        raqote::ExtendMode::Pad,
                        raqote::FilterMode::Nearest,
                        raqote::Transform::translation(overflow as f32, overflow as f32),
                        true,
                        true,
                    ),
                    &Default::default(),
                );
                *draw_target = result;
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Filter {
    instructions: Vec<FilterInstruction>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn overflow(&self) -> SideOffsets2D<usize> {
        let mut result = SideOffsets2D::zero();
        for instruction in self.instructions.iter() {
            result += instruction.overflow();
        }
        result
    }

    pub fn apply(&self, draw_target: &mut raqote::DrawTarget) {
        for instruction in self.instructions.iter() {
            instruction.apply(draw_target);
        }
    }

    pub fn color_matrix(&mut self, mat: ColorMatrix) {
        if mat.is_identity() {
            return;
        }
        if let Some(FilterInstruction::ColorMatrix(last)) = self.instructions.last_mut() {
            *last = last.then(mat);
        } else {
            self.instructions.push(FilterInstruction::ColorMatrix(mat));
        }
        if mat.needs_clamp() {
            self.instructions.push(FilterInstruction::NoOp);
        }
    }

    pub fn blur(&mut self, blur: f32) {
        if blur != 0.0 {
            self.instructions.push(FilterInstruction::Blur(blur));
        }
    }

    pub fn make_shadow(&mut self, color: raqote::Color, blur: f32) {
        self.instructions
            .push(FilterInstruction::MakeShadow { color, blur });
    }

    pub fn drop_shadow(&mut self, color: raqote::Color, offset_x: f32, offset_y: f32, blur: f32) {
        self.instructions.push(FilterInstruction::DropShadow {
            color,
            offset_x,
            offset_y,
            blur,
        });
    }
}

pub fn compile_filter(
    computed: &ComputedFilter,
    destination_color_space: CanvasColorSpace,
) -> Filter {
    match computed.filter_value_list {
        Some(ref v) => {
            let mut result = Filter::new();
            for value in v.iter() {
                match *value {
                    ComputedFilterValue::Url(_) => return Filter::default(),
                    ComputedFilterValue::FilterFunction(f) => match f {
                        ComputedFilterFunction::Blur(blur) => {
                            result.blur(blur.px);
                        }
                        ComputedFilterFunction::Brightness(t) => {
                            result.color_matrix(ColorMatrix {
                                a00: t,
                                a01: 0.0,
                                a02: 0.0,
                                a03: 0.0,
                                a10: 0.0,
                                a11: t,
                                a12: 0.0,
                                a13: 0.0,
                                a20: 0.0,
                                a21: 0.0,
                                a22: t,
                                a23: 0.0,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                        ComputedFilterFunction::Contrast(t) => {
                            result.color_matrix(ColorMatrix {
                                a00: t,
                                a01: 0.0,
                                a02: 0.0,
                                a03: 0.5 - 0.5 * t,
                                a10: 0.0,
                                a11: t,
                                a12: 0.0,
                                a13: 0.5 - 0.5 * t,
                                a20: 0.0,
                                a21: 0.0,
                                a22: t,
                                a23: 0.5 - 0.5 * t,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                        ComputedFilterFunction::DropShadow(shadow) => {
                            let color = resolve_color_for_canvas(shadow.color);
                            result.drop_shadow(
                                to_raqote_color(color, destination_color_space),
                                shadow.offset_x.px,
                                shadow.offset_y.px,
                                shadow.blur.px,
                            );
                        }
                        ComputedFilterFunction::Grayscale(t) => {
                            let t = t.min(1.0);
                            result.color_matrix(ColorMatrix {
                                a00: 1.0 - 0.787 * t,
                                a01: 0.715 * t,
                                a02: 0.072 * t,
                                a03: 0.0,
                                a10: 0.213 * t,
                                a11: 1.0 - 0.285 * t,
                                a12: 0.072 * t,
                                a13: 0.0,
                                a20: 0.213 * t,
                                a21: 0.715 * t,
                                a22: 1.0 - 0.928 * t,
                                a23: 0.0,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                        ComputedFilterFunction::HueRotate(t) => {
                            let (sin, cos) = t.radians().sin_cos();
                            let inv_cos = 1.0 - cos;
                            result.color_matrix(ColorMatrix {
                                a00: 1.0 - 0.787 * inv_cos - 0.213 * sin,
                                a01: 0.715 * inv_cos - 0.715 * sin,
                                a02: 0.072 * inv_cos + 0.928 * sin,
                                a03: 0.0,
                                a10: 0.213 * inv_cos + 0.143 * sin,
                                a11: 1.0 - 0.285 * inv_cos + 0.140 * sin,
                                a12: 0.072 * inv_cos - 0.283 * sin,
                                a13: 0.0,
                                a20: 0.213 * inv_cos - 0.787 * sin,
                                a21: 0.715 * inv_cos + 0.715 * sin,
                                a22: 1.0 - 0.928 * inv_cos + 0.072 * sin,
                                a23: 0.0,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                        ComputedFilterFunction::Invert(t) => {
                            let t = t.min(1.0);
                            result.color_matrix(ColorMatrix {
                                a00: 1.0 - 2.0 * t,
                                a01: 0.0,
                                a02: 0.0,
                                a03: t,
                                a10: 0.0,
                                a11: 1.0 - 2.0 * t,
                                a12: 0.0,
                                a13: t,
                                a20: 0.0,
                                a21: 0.0,
                                a22: 1.0 - 2.0 * t,
                                a23: t,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                        ComputedFilterFunction::Opacity(t) => {
                            let t = t.min(1.0);
                            result.color_matrix(ColorMatrix {
                                a00: t,
                                a01: 0.0,
                                a02: 0.0,
                                a03: 0.0,
                                a10: 0.0,
                                a11: t,
                                a12: 0.0,
                                a13: 0.0,
                                a20: 0.0,
                                a21: 0.0,
                                a22: t,
                                a23: 0.0,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: t,
                            });
                        }
                        ComputedFilterFunction::Saturate(t) => {
                            let inv_t = 1.0 - t;
                            result.color_matrix(ColorMatrix {
                                a00: 1.0 - 0.787 * inv_t,
                                a01: 0.715 * inv_t,
                                a02: 0.072 * inv_t,
                                a03: 0.0,
                                a10: 0.213 * inv_t,
                                a11: 1.0 - 0.285 * inv_t,
                                a12: 0.072 * inv_t,
                                a13: 0.0,
                                a20: 0.213 * inv_t,
                                a21: 0.715 * inv_t,
                                a22: 1.0 - 0.928 * inv_t,
                                a23: 0.0,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                        ComputedFilterFunction::Sepia(t) => {
                            let t = t.min(1.0);
                            result.color_matrix(ColorMatrix {
                                a00: 1.0 - 0.607 * t,
                                a01: 0.769 * t,
                                a02: 0.189 * t,
                                a03: 0.0,
                                a10: 0.349 * t,
                                a11: 1.0 - 0.314 * t,
                                a12: 0.168 * t,
                                a13: 0.0,
                                a20: 0.272 * t,
                                a21: 0.534 * t,
                                a22: 1.0 - 0.869 * t,
                                a23: 0.0,
                                a30: 0.0,
                                a31: 0.0,
                                a32: 0.0,
                                a33: 1.0,
                            });
                        }
                    },
                }
            }
            result
        }
        None => Filter::default(),
    }
}
