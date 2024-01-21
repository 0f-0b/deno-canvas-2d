pub mod encoding;

use std::convert::Infallible;

use cssparser::color::PredefinedColorSpace;
use cssparser::{BasicParseError, ParseError, Parser, ParserInput};
use cssparser_color::{parse_color_with, ColorParser, FromParsedColor};
use palette::chromatic_adaptation::AdaptFrom as _;
use palette::encoding::Linear;
use palette::rgb::Rgb;
use palette::white_point::{D50, D65};
use palette::{
    Clamp, FromColor as _, Hsl, Hwb, IntoColor, Lab, Lch, LinSrgb, Oklab, Oklch, Srgb, Xyz,
};

pub type DisplayP3<T = f32> = Rgb<encoding::DisplayP3, T>;
pub type A98Rgb<T = f32> = Rgb<encoding::A98Rgb, T>;
pub type ProphotoRgb<T = f32> = Rgb<encoding::ProphotoRgb, T>;
pub type Rec2020<T = f32> = Rgb<encoding::Rec2020, T>;
pub type LinDisplayP3<T = f32> = Rgb<Linear<encoding::DisplayP3>, T>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteColorValue {
    LegacyRgb(Srgb),
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
    pub fn into_srgb(self) -> Srgb {
        match self {
            AbsoluteColorValue::LegacyRgb(c) => c,
            AbsoluteColorValue::Lab(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Lch(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Oklab(c) => Rgb::from_color(c),
            AbsoluteColorValue::Oklch(c) => Rgb::from_color(c),
            AbsoluteColorValue::Srgb(c) => c,
            AbsoluteColorValue::SrgbLinear(c) => Rgb::from_color(c),
            AbsoluteColorValue::DisplayP3(c) => Rgb::from_color(c),
            AbsoluteColorValue::A98Rgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::ProphotoRgb(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Rec2020(c) => Rgb::from_color(c),
            AbsoluteColorValue::XyzD50(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::XyzD65(c) => Rgb::from_color(c),
        }
    }

    pub fn into_linear_srgb(self) -> LinSrgb {
        match self {
            AbsoluteColorValue::LegacyRgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::Lab(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Lch(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Oklab(c) => Rgb::from_color(c),
            AbsoluteColorValue::Oklch(c) => Rgb::from_color(c),
            AbsoluteColorValue::Srgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::SrgbLinear(c) => c,
            AbsoluteColorValue::DisplayP3(c) => Rgb::from_color(c),
            AbsoluteColorValue::A98Rgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::ProphotoRgb(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Rec2020(c) => Rgb::from_color(c),
            AbsoluteColorValue::XyzD50(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::XyzD65(c) => Rgb::from_color(c),
        }
    }

    pub fn into_linear_display_p3(self) -> LinDisplayP3 {
        match self {
            AbsoluteColorValue::LegacyRgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::Lab(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Lch(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Oklab(c) => Rgb::from_color(c),
            AbsoluteColorValue::Oklch(c) => Rgb::from_color(c),
            AbsoluteColorValue::Srgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::SrgbLinear(c) => Rgb::from_color(c),
            AbsoluteColorValue::DisplayP3(c) => Rgb::from_color(c),
            AbsoluteColorValue::A98Rgb(c) => Rgb::from_color(c),
            AbsoluteColorValue::ProphotoRgb(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::Rec2020(c) => Rgb::from_color(c),
            AbsoluteColorValue::XyzD50(c) => Rgb::adapt_from(c).clamp(),
            AbsoluteColorValue::XyzD65(c) => Rgb::from_color(c),
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
        value: AbsoluteColorValue::LegacyRgb(Rgb::new(0.0, 0.0, 0.0)),
        alpha: 1.0,
    };
    pub const TRANSPARENT_BLACK: Self = Self {
        value: AbsoluteColorValue::LegacyRgb(Rgb::new(0.0, 0.0, 0.0)),
        alpha: 0.0,
    };
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
            value: AbsoluteColorValue::LegacyRgb(Rgb::new(red, green, blue).into_format()),
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
            value: AbsoluteColorValue::LegacyRgb(Hsl::new(hue, saturation, lightness).into_color()),
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
            value: AbsoluteColorValue::LegacyRgb(Hwb::new(hue, whiteness, blackness).into_color()),
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

impl ComputedColor {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        struct BasicColorParser;

        impl<'i> ColorParser<'i> for BasicColorParser {
            type Output = ComputedColor;
            type Error = Infallible;
        }

        parse_color_with(&BasicColorParser, input)
    }
}

pub fn parse_and_compute_color(css: &str) -> Result<ComputedColor, BasicParseError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    parser
        .parse_entirely(ComputedColor::parse_and_compute)
        .map_err(ParseError::basic)
}
