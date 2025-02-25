pub mod encoding;

use std::convert::Infallible;
use std::fmt;

use cssparser::color::PredefinedColorSpace;
use cssparser::{ParseError, Parser, ToCss};
use cssparser_color::{parse_color_with, ColorParser, FromParsedColor};
use palette::chromatic_adaptation::AdaptInto as _;
use palette::color_difference::EuclideanDistance as _;
use palette::convert::{FromColorUnclamped, IntoColorUnclamped as _};
use palette::encoding::Linear;
use palette::rgb::Rgb;
use palette::white_point::{D50, D65};
use palette::{
    Clamp as _, FromColor as _, Hsl, Hwb, IntoColor as _, IsWithinBounds as _, Lab, Lch, LinSrgb,
    Oklab, Oklch, Srgb, Xyz,
};

use super::FromCss;

pub type DisplayP3<T = f32> = Rgb<encoding::DisplayP3, T>;
pub type A98Rgb<T = f32> = Rgb<encoding::A98Rgb, T>;
pub type ProphotoRgb<T = f32> = Rgb<encoding::ProphotoRgb, T>;
pub type Rec2020<T = f32> = Rgb<encoding::Rec2020, T>;
pub type LinDisplayP3<T = f32> = Rgb<Linear<encoding::DisplayP3>, T>;

fn gamut_map_oklch_to_rgb<S>(oklch: Oklch) -> Rgb<S>
where
    Rgb<S>: FromColorUnclamped<Oklch>,
    Oklab: FromColorUnclamped<Rgb<S>>,
{
    const JND_SQ: f32 = 0.0004;
    const EPSILON: f32 = 0.0001;
    if oklch.l >= 1.0 {
        return Rgb::new(1.0, 1.0, 1.0);
    }
    if oklch.l <= 0.0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }
    let rgb = Rgb::from_color_unclamped(oklch);
    if rgb.is_within_bounds() {
        return rgb;
    }
    let rgb_clipped = rgb.clamp();
    let dist_sq = Oklab::distance_squared(
        oklch.into_color_unclamped(),
        rgb_clipped.into_color_unclamped(),
    );
    if dist_sq < JND_SQ {
        return rgb_clipped;
    }
    let mut result = rgb_clipped;
    let mut min_chroma = 0.0;
    let mut max_chroma = oklch.chroma;
    while max_chroma - min_chroma > EPSILON {
        let chroma = (min_chroma + max_chroma) * 0.5;
        let oklch = Oklch { chroma, ..oklch };
        let rgb_clipped = Rgb::from_color(oklch);
        let dist_sq = Oklab::distance_squared(
            oklch.into_color_unclamped(),
            rgb_clipped.into_color_unclamped(),
        );
        if dist_sq < JND_SQ {
            min_chroma = chroma;
        } else {
            max_chroma = chroma;
            result = rgb_clipped;
        }
    }
    result
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteColorValue {
    LegacyRgb(Srgb<u8>),
    Lab(Lab<D50>),
    Lch(Lch<D50>),
    Oklab(Oklab),
    Oklch(Oklch),
    Srgb(Srgb),
    SrgbLinear(LinSrgb),
    DisplayP3(DisplayP3),
    A98Rgb(A98Rgb),
    ProphotoRgb(ProphotoRgb),
    Rec2020(Rec2020),
    XyzD50(Xyz<D50>),
    XyzD65(Xyz<D65>),
}

impl AbsoluteColorValue {
    pub const BLACK: Self = Self::LegacyRgb(Rgb::new(0, 0, 0));

    pub fn into_oklch(self) -> Oklch {
        match self {
            Self::LegacyRgb(c) => c.into_linear().into_color_unclamped(),
            Self::Lab(c) => c.adapt_into(),
            Self::Lch(c) => c.adapt_into(),
            Self::Oklab(c) => c.into_color_unclamped(),
            Self::Oklch(c) => c,
            Self::Srgb(c) => c.into_color_unclamped(),
            Self::SrgbLinear(c) => c.into_color_unclamped(),
            Self::DisplayP3(c) => c.into_color_unclamped(),
            Self::A98Rgb(c) => c.into_color_unclamped(),
            Self::ProphotoRgb(c) => c.adapt_into(),
            Self::Rec2020(c) => c.into_color_unclamped(),
            Self::XyzD50(c) => c.adapt_into(),
            Self::XyzD65(c) => c.into_color_unclamped(),
        }
    }

    pub fn into_linear_srgb(self) -> LinSrgb {
        match self {
            Self::LegacyRgb(c) => c.into_linear(),
            _ => gamut_map_oklch_to_rgb(self.into_oklch()),
        }
    }

    pub fn into_linear_display_p3(self) -> LinDisplayP3 {
        match self {
            Self::LegacyRgb(c) => c.into_linear().into_color_unclamped(),
            _ => gamut_map_oklch_to_rgb(self.into_oklch()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbsoluteColor {
    pub value: AbsoluteColorValue,
    pub alpha: f32,
}

impl AbsoluteColor {
    pub const OPAQUE_BLACK: Self = Self {
        value: AbsoluteColorValue::BLACK,
        alpha: 1.0,
    };
    pub const TRANSPARENT_BLACK: Self = Self {
        value: AbsoluteColorValue::BLACK,
        alpha: 0.0,
    };
}

impl ToCss for AbsoluteColor {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use cssparser::color::PredefinedColorSpace;
        use cssparser_color::{ColorFunction, Lab, Lch, Oklab, Oklch, RgbaLegacy};

        match self.value {
            AbsoluteColorValue::LegacyRgb(c) => RgbaLegacy {
                red: c.red,
                green: c.green,
                blue: c.blue,
                alpha: self.alpha,
            }
            .to_css(dest),
            AbsoluteColorValue::Lab(c) => Lab {
                lightness: Some(c.l),
                a: Some(c.a),
                b: Some(c.b),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::Lch(c) => Lch {
                lightness: Some(c.l),
                chroma: Some(c.chroma),
                hue: Some(c.hue.into_raw_degrees()),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::Oklab(c) => Oklab {
                lightness: Some(c.l),
                a: Some(c.a),
                b: Some(c.b),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::Oklch(c) => Oklch {
                lightness: Some(c.l),
                chroma: Some(c.chroma),
                hue: Some(c.hue.into_raw_degrees()),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::Srgb(c) => ColorFunction {
                color_space: PredefinedColorSpace::Srgb,
                c1: Some(c.red),
                c2: Some(c.green),
                c3: Some(c.blue),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::SrgbLinear(c) => ColorFunction {
                color_space: PredefinedColorSpace::SrgbLinear,
                c1: Some(c.red),
                c2: Some(c.green),
                c3: Some(c.blue),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::DisplayP3(c) => ColorFunction {
                color_space: PredefinedColorSpace::DisplayP3,
                c1: Some(c.red),
                c2: Some(c.green),
                c3: Some(c.blue),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::A98Rgb(c) => ColorFunction {
                color_space: PredefinedColorSpace::A98Rgb,
                c1: Some(c.red),
                c2: Some(c.green),
                c3: Some(c.blue),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::ProphotoRgb(c) => ColorFunction {
                color_space: PredefinedColorSpace::ProphotoRgb,
                c1: Some(c.red),
                c2: Some(c.green),
                c3: Some(c.blue),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::Rec2020(c) => ColorFunction {
                color_space: PredefinedColorSpace::Rec2020,
                c1: Some(c.red),
                c2: Some(c.green),
                c3: Some(c.blue),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::XyzD50(c) => ColorFunction {
                color_space: PredefinedColorSpace::XyzD50,
                c1: Some(c.x),
                c2: Some(c.y),
                c3: Some(c.z),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
            AbsoluteColorValue::XyzD65(c) => ColorFunction {
                color_space: PredefinedColorSpace::XyzD65,
                c1: Some(c.x),
                c2: Some(c.y),
                c3: Some(c.z),
                alpha: Some(self.alpha),
            }
            .to_css(dest),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedColor {
    Absolute(AbsoluteColor),
    CurrentColor,
}

impl FromParsedColor for ComputedColor {
    fn from_current_color() -> Self {
        Self::CurrentColor
    }

    fn from_rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::LegacyRgb(Rgb::new(red, green, blue)),
            alpha,
        })
    }

    fn from_hsl(
        hue: Option<f32>,
        saturation: Option<f32>,
        lightness: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let hue = hue.unwrap_or(0.0);
        let saturation = saturation.unwrap_or(0.0);
        let lightness = lightness.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::LegacyRgb(Rgb::from_format(
                Hsl::new(hue, saturation, lightness).into_color(),
            )),
            alpha,
        })
    }

    fn from_hwb(
        hue: Option<f32>,
        whiteness: Option<f32>,
        blackness: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let hue = hue.unwrap_or(0.0);
        let whiteness = whiteness.unwrap_or(0.0);
        let blackness = blackness.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::LegacyRgb(Rgb::from_format(
                Hwb::new(hue, whiteness, blackness).into_color(),
            )),
            alpha,
        })
    }

    fn from_lab(
        lightness: Option<f32>,
        a: Option<f32>,
        b: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let lightness = lightness.unwrap_or(0.0);
        let a = a.unwrap_or(0.0);
        let b = b.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::Lab(Lab::new(lightness, a, b)),
            alpha,
        })
    }

    fn from_lch(
        lightness: Option<f32>,
        chroma: Option<f32>,
        hue: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let lightness = lightness.unwrap_or(0.0);
        let chroma = chroma.unwrap_or(0.0);
        let hue = hue.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::Lch(Lch::new(lightness, chroma, hue)),
            alpha,
        })
    }

    fn from_oklab(
        lightness: Option<f32>,
        a: Option<f32>,
        b: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let lightness = lightness.unwrap_or(0.0);
        let a = a.unwrap_or(0.0);
        let b = b.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::Oklab(Oklab::new(lightness, a, b)),
            alpha,
        })
    }

    fn from_oklch(
        lightness: Option<f32>,
        chroma: Option<f32>,
        hue: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let lightness = lightness.unwrap_or(0.0);
        let chroma = chroma.unwrap_or(0.0);
        let hue = hue.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        Self::Absolute(AbsoluteColor {
            value: AbsoluteColorValue::Oklch(Oklch::new(lightness, chroma, hue)),
            alpha,
        })
    }

    fn from_color_function(
        color_space: PredefinedColorSpace,
        c1: Option<f32>,
        c2: Option<f32>,
        c3: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let c1 = c1.unwrap_or(0.0);
        let c2 = c2.unwrap_or(0.0);
        let c3 = c3.unwrap_or(0.0);
        let alpha = alpha.unwrap_or(0.0);
        match color_space {
            PredefinedColorSpace::Srgb => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::Srgb(Rgb::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::SrgbLinear => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::SrgbLinear(Rgb::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::DisplayP3 => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::DisplayP3(Rgb::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::A98Rgb => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::A98Rgb(Rgb::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::ProphotoRgb => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::ProphotoRgb(Rgb::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::Rec2020 => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::Rec2020(Rgb::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::XyzD50 => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::XyzD50(Xyz::new(c1, c2, c3)),
                alpha,
            }),
            PredefinedColorSpace::XyzD65 => Self::Absolute(AbsoluteColor {
                value: AbsoluteColorValue::XyzD65(Xyz::new(c1, c2, c3)),
                alpha,
            }),
        }
    }
}

impl FromCss for ComputedColor {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        struct BasicColorParser;

        impl ColorParser<'_> for BasicColorParser {
            type Output = ComputedColor;
            type Error = Infallible;
        }

        parse_color_with(&BasicColorParser, input)
    }
}
