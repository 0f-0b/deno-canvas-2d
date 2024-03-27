use std::array;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Mul};
use std::rc::Rc;

use euclid::default::{Box2D, Point2D, SideOffsets2D, Size2D, Transform2D};
use euclid::{size2, vec2};
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
    pub fn is_identity(&self) -> bool {
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

    pub fn needs_clamp(&self) -> bool {
        self.a00.min(0.0) + self.a01.min(0.0) + self.a02.min(0.0) + self.a03.min(0.0) < 0.0
            || self.a10.min(0.0) + self.a11.min(0.0) + self.a12.min(0.0) + self.a13.min(0.0) < 0.0
            || self.a20.min(0.0) + self.a21.min(0.0) + self.a22.min(0.0) + self.a23.min(0.0) < 0.0
            || self.a30.min(0.0) + self.a31.min(0.0) + self.a32.min(0.0) + self.a33.min(0.0) < 0.0
            || self.a00.max(0.0) + self.a01.max(0.0) + self.a02.max(0.0) + self.a03.max(0.0) > 1.0
            || self.a10.max(0.0) + self.a11.max(0.0) + self.a12.max(0.0) + self.a13.max(0.0) > 1.0
            || self.a20.max(0.0) + self.a21.max(0.0) + self.a22.max(0.0) + self.a23.max(0.0) > 1.0
            || self.a30.max(0.0) + self.a31.max(0.0) + self.a32.max(0.0) + self.a33.max(0.0) > 1.0
    }

    pub fn then(&self, m: &Self) -> Self {
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

    pub fn apply(&self, c: Color) -> Color {
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

fn apply_shadow(
    mut surface: raqote::DrawTarget,
    color: raqote::Color,
    blur: f32,
) -> raqote::DrawTarget {
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
        for pixel in surface.get_data_mut() {
            *pixel = colors[(*pixel >> 24) as usize];
        }
        return surface;
    }
    let kernel = gaussian_kernel(blur);
    let width_with_overflow = surface.width() as usize;
    let height_with_overflow = surface.height() as usize;
    let width = width_with_overflow.checked_sub(overflow * 2).unwrap();
    let height = height_with_overflow.checked_sub(overflow * 2).unwrap();
    let mut buf = vec![0.0; width * height_with_overflow];
    let src = surface.get_data();
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
    drop(surface);
    let mut result = raqote::DrawTarget::new(width as i32, height as i32);
    let dst = result.get_data_mut();
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
    result
}

fn apply_blur(surface: raqote::DrawTarget, blur: f32) -> raqote::DrawTarget {
    let overflow = gaussian_blur_radius(blur);
    if overflow == 0 {
        return surface;
    }
    let kernel = gaussian_kernel(blur);
    let width_with_overflow = surface.width() as usize;
    let height_with_overflow = surface.height() as usize;
    let width = width_with_overflow.checked_sub(overflow * 2).unwrap();
    let height = height_with_overflow.checked_sub(overflow * 2).unwrap();
    let mut buf = vec![Color::TRANSPARENT; width * height_with_overflow];
    let src = surface.get_data();
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
    drop(surface);
    let mut result = raqote::DrawTarget::new(width as i32, height as i32);
    let dst = result.get_data_mut();
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
    result
}

pub type RenderFunction = dyn Fn(&mut raqote::DrawTarget, raqote::DrawOptions);

pub struct BoxedRenderFunction(pub Box<RenderFunction>);

impl Debug for BoxedRenderFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedRenderFunction")
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub enum FilterChain {
    Source {
        render: BoxedRenderFunction,
    },
    Passthrough {
        src: Rc<FilterChain>,
    },
    Transform {
        src: Rc<FilterChain>,
        mat: Transform2D<f32>,
    },
    Overlay {
        on: Rc<FilterChain>,
        src: Rc<FilterChain>,
    },
    ColorMatrix {
        src: Rc<FilterChain>,
        mat: ColorMatrix,
    },
    Blur {
        src: Rc<FilterChain>,
        blur: f32,
    },
    Shadow {
        src: Rc<FilterChain>,
        color: raqote::Color,
        blur: f32,
    },
}

impl FilterChain {
    pub fn render(&self, size: Size2D<usize>, transform: &Transform2D<f32>) -> raqote::DrawTarget {
        match *self {
            Self::Source { ref render } => {
                let mut result = raqote::DrawTarget::new(
                    size.width.try_into().unwrap(),
                    size.height.try_into().unwrap(),
                );
                result.set_transform(transform);
                render.0(
                    &mut result,
                    raqote::DrawOptions {
                        blend_mode: raqote::BlendMode::Src,
                        ..Default::default()
                    },
                );
                result
            }
            Self::Passthrough { ref src } => src.render(size, transform),
            Self::Transform { ref src, ref mat } => src.render(size, &transform.then(mat)),
            Self::Overlay { ref src, ref on } => {
                let mut result = on.render(size, transform);
                let overlay = src.render(size, transform);
                result.blend_surface(
                    &overlay,
                    Box2D::from_size(size2(result.width(), result.height())),
                    Point2D::origin(),
                    raqote::BlendMode::SrcOver,
                );
                result
            }
            Self::ColorMatrix { ref src, ref mat } => {
                let mut result = src.render(size, transform);
                transform_argb32(result.get_data_mut(), |c| {
                    mat.apply(Color::from_u8(c)).to_u8()
                });
                result
            }
            Self::Blur { ref src, blur } => {
                let overflow = SideOffsets2D::new_all_same(gaussian_blur_radius(blur));
                let src = src.render_with_offsets(size, overflow, transform);
                apply_blur(src, blur)
            }
            Self::Shadow {
                ref src,
                color,
                blur,
            } => {
                let overflow = SideOffsets2D::new_all_same(gaussian_blur_radius(blur));
                let src = src.render_with_offsets(size, overflow, transform);
                apply_shadow(src, color, blur)
            }
        }
    }

    pub fn render_with_offsets(
        &self,
        size: Size2D<usize>,
        offsets: SideOffsets2D<usize>,
        transform: &Transform2D<f32>,
    ) -> raqote::DrawTarget {
        self.render(
            size2(
                size.width + offsets.horizontal(),
                size.height + offsets.vertical(),
            ),
            &transform.then_translate(vec2(offsets.left as f32, offsets.top as f32)),
        )
    }

    pub fn new(render: BoxedRenderFunction) -> Rc<Self> {
        Rc::new(Self::Source { render })
    }

    pub fn transform(self: Rc<Self>, mat: &Transform2D<f32>) -> Rc<Self> {
        if *mat == Transform2D::identity() {
            return self;
        }
        Rc::new(Self::Transform {
            src: self,
            mat: *mat,
        })
    }

    pub fn overlay(self: Rc<Self>, on: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Overlay { src: self, on })
    }

    pub fn color_matrix(self: Rc<Self>, mat: &ColorMatrix) -> Rc<Self> {
        if mat.is_identity() {
            return self;
        }
        let mut result = match *self {
            Self::ColorMatrix { ref src, mat: last } => Rc::new(Self::ColorMatrix {
                src: src.clone(),
                mat: last.then(mat),
            }),
            _ => Rc::new(Self::ColorMatrix {
                src: self,
                mat: *mat,
            }),
        };
        if mat.needs_clamp() {
            result = Rc::new(Self::Passthrough { src: result });
        }
        result
    }

    pub fn blur(self: Rc<Self>, blur: f32) -> Rc<Self> {
        if blur == 0.0 {
            return self;
        }
        Rc::new(Self::Blur { src: self, blur })
    }

    pub fn shadow(self: Rc<Self>, color: raqote::Color, blur: f32) -> Rc<Self> {
        Rc::new(Self::Shadow {
            src: self,
            color,
            blur,
        })
    }
}

pub fn compile_filter(
    render: BoxedRenderFunction,
    computed: &ComputedFilter,
    destination_color_space: CanvasColorSpace,
) -> Rc<FilterChain> {
    let source = FilterChain::new(render);
    let Some(ref list) = computed.filter_value_list else {
        return source;
    };
    let mut result = source.clone();
    for value in list.iter() {
        match *value {
            ComputedFilterValue::Url(_) => return source,
            ComputedFilterValue::FilterFunction(f) => match f {
                ComputedFilterFunction::Blur(blur) => {
                    result = result.blur(blur.px);
                }
                ComputedFilterFunction::Brightness(t) => {
                    result = result.color_matrix(&ColorMatrix {
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
                    result = result.color_matrix(&ColorMatrix {
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
                    let color = to_raqote_color(
                        resolve_color_for_canvas(shadow.color),
                        destination_color_space,
                    );
                    let offset_x = shadow.offset_x.px;
                    let offset_y = shadow.offset_y.px;
                    let blur = shadow.blur.px;
                    result = result.clone().overlay(
                        result
                            .transform(&Transform2D::translation(offset_x, offset_y))
                            .shadow(color, blur),
                    );
                }
                ComputedFilterFunction::Grayscale(t) => {
                    let t = t.min(1.0);
                    result = result.color_matrix(&ColorMatrix {
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
                    result = result.color_matrix(&ColorMatrix {
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
                    result = result.color_matrix(&ColorMatrix {
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
                    result = result.color_matrix(&ColorMatrix {
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
                    result = result.color_matrix(&ColorMatrix {
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
                    result = result.color_matrix(&ColorMatrix {
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
