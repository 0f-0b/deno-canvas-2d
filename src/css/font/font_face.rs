use std::convert::Infallible;
use std::fmt;
use std::rc::Rc;

use cssparser::{
    match_ignore_ascii_case, serialize_string, ParseError, Parser, ToCss, Token, UnicodeRange,
};

use super::super::angle::{ComputedAngle, SpecifiedAngle};
use super::super::{parse_url, CssValue, FromCss};

#[derive(Clone, Debug)]
pub struct SpecifiedFontFamily {
    pub name: Rc<str>,
}

impl FromCss for SpecifiedFontFamily {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        Ok(Self {
            name: super::ComputedSpecificFamily::from_css(input)?.name,
        })
    }
}

impl ToCss for SpecifiedFontFamily {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_string(&self.name, dest)
    }
}

#[derive(Clone, Debug)]
pub enum SpecifiedFontSource {
    Url(Rc<str>),
    Local(super::ComputedSpecificFamily),
}

impl FromCss for SpecifiedFontSource {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        if let Ok(v) = input.try_parse(|input| {
            input.expect_function_matching("local")?;
            input.parse_nested_block(super::ComputedSpecificFamily::from_css)
        }) {
            return Ok(Self::Local(v));
        }
        let url = parse_url(input)?;
        // TODO `format(<font-format>)`
        // TODO `tech(<font-tech>#)`
        Ok(Self::Url(url))
    }
}

#[derive(Clone, Debug)]
pub struct SpecifiedSource {
    pub font_source_list: Rc<[SpecifiedFontSource]>,
}

impl FromCss for SpecifiedSource {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let font_source_list =
            input.parse_comma_separated_ignoring_errors(SpecifiedFontSource::from_css);
        if font_source_list.is_empty() {
            return Err(input.new_error_for_next_token());
        }
        let font_source_list = font_source_list.into();
        Ok(Self { font_source_list })
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SpecifiedFontStyle {
    #[default]
    Normal,
    Italic,
    Oblique(SpecifiedAngle, SpecifiedAngle),
}

impl SpecifiedFontStyle {
    pub fn compute(self) -> ComputedFontStyle {
        match self {
            Self::Normal => ComputedFontStyle::Normal,
            Self::Italic => ComputedFontStyle::Italic,
            Self::Oblique(angle1, angle2) => {
                let angle1 = angle1.compute();
                let angle2 = angle2.compute();
                if angle1.deg > angle2.deg {
                    ComputedFontStyle::Oblique(angle2, angle1)
                } else {
                    ComputedFontStyle::Oblique(angle1, angle2)
                }
            }
        }
    }
}

impl FromCss for SpecifiedFontStyle {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "italic" => Self::Italic,
            "oblique" => {
                let angle1 = match input
                    .try_parse(|input| SpecifiedAngle::from_css_with_range(input, -90.0..=90.0))
                {
                    Ok(v) => v,
                    Err(_) => SpecifiedAngle::Deg(14.0),
                };
                let angle2 = match input
                    .try_parse(|input| SpecifiedAngle::from_css_with_range(input, -90.0..=90.0))
                {
                    Ok(v) => v,
                    Err(_) => angle1,
                };
                Self::Oblique(angle1, angle2)
            },
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for SpecifiedFontStyle {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(angle1, angle2) => {
                dest.write_str("oblique")?;
                if angle1 != SpecifiedAngle::Deg(14.0) || angle1 != angle2 {
                    write!(dest, " {}", CssValue(&angle1))?;
                    if angle1 != angle2 {
                        write!(dest, " {}", CssValue(&angle2))?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComputedFontStyle {
    Normal,
    Italic,
    Oblique(ComputedAngle, ComputedAngle),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpecifiedFontWeight(
    pub super::SpecifiedAbsoluteFontWeight,
    pub super::SpecifiedAbsoluteFontWeight,
);

impl SpecifiedFontWeight {
    pub fn compute(self) -> ComputedFontWeight {
        let value1 = self.0.compute().0;
        let value2 = self.1.compute().0;
        if value1 > value2 {
            ComputedFontWeight(value2, value1)
        } else {
            ComputedFontWeight(value1, value2)
        }
    }
}

impl FromCss for SpecifiedFontWeight {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let value1 = input
            .try_parse(super::SpecifiedAbsoluteFontWeight::from_css)
            .unwrap_or_default();
        let value2 = input
            .try_parse(super::SpecifiedAbsoluteFontWeight::from_css)
            .unwrap_or(value1);
        Ok(Self(value1, value2))
    }
}

impl ToCss for SpecifiedFontWeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.0 != self.1 {
            write!(dest, " {}", CssValue(&self.1))?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedFontWeight(pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default)]
pub struct SpecifiedFontWidth(pub super::SpecifiedFontWidth, pub super::SpecifiedFontWidth);

impl SpecifiedFontWidth {
    pub fn compute(self) -> ComputedFontWidth {
        let value1 = self.0.compute().0;
        let value2 = self.1.compute().0;
        if value1 > value2 {
            ComputedFontWidth(value2, value1)
        } else {
            ComputedFontWidth(value1, value2)
        }
    }
}

impl FromCss for SpecifiedFontWidth {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let value1 = input
            .try_parse(super::SpecifiedFontWidth::from_css)
            .unwrap_or_default();
        let value2 = input
            .try_parse(super::SpecifiedFontWidth::from_css)
            .unwrap_or(value1);
        Ok(Self(value1, value2))
    }
}

impl ToCss for SpecifiedFontWidth {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.0 != self.1 {
            write!(dest, " {}", CssValue(&self.1))?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedFontWidth(pub f32, pub f32);

#[derive(Clone, Debug)]
pub struct SpecifiedUnicodeRange {
    pub range_list: Rc<[UnicodeRange]>,
}

impl Default for SpecifiedUnicodeRange {
    fn default() -> Self {
        Self {
            range_list: Rc::new([UnicodeRange {
                start: 0,
                end: 0x10ffff,
            }]),
        }
    }
}

impl FromCss for SpecifiedUnicodeRange {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let range_list = input.parse_comma_separated(UnicodeRange::from_css)?.into();
        Ok(Self { range_list })
    }
}

impl ToCss for SpecifiedUnicodeRange {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.range_list.iter();
        iter.next().unwrap().to_css(dest)?;
        for name in iter {
            write!(dest, ", {}", CssValue(name))?;
        }
        Ok(())
    }
}
