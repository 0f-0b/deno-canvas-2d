use std::convert::Infallible;
use std::fmt;
use std::rc::Rc;

use cssparser::{
    ParseError, Parser, ToCss, Token, UnicodeRange, match_ignore_ascii_case, serialize_string,
};

use super::super::angle::{ComputedAngle, SpecifiedAngle};
use super::super::{
    CssNumber, CssPercentage, CssString, CssValue, FromCss, parse_integer_with_range, parse_number,
    parse_url, try_match_next_ident_ignore_ascii_case,
};
use super::{ComputedSpecificFamily, SpecifiedAbsoluteFontWeight, SpecifiedFontWidth};

#[derive(Clone, Debug)]
pub enum SpecifiedFontSource {
    Url(Rc<str>),
    Local(#[allow(dead_code)] ComputedSpecificFamily),
}

impl FromCss for SpecifiedFontSource {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        if let Ok(v) = input.try_parse(|input| {
            input.expect_function_matching("local")?;
            input.parse_nested_block(ComputedSpecificFamily::from_css)
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
pub struct SpecifiedFontSources {
    pub font_source_list: Rc<[SpecifiedFontSource]>,
}

impl FromCss for SpecifiedFontSources {
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpecifiedFontStyleRange {
    #[default]
    Normal,
    Italic,
    Oblique(SpecifiedAngle, SpecifiedAngle),
}

impl SpecifiedFontStyleRange {
    pub fn compute(self) -> ComputedFontStyleRange {
        match self {
            Self::Normal => ComputedFontStyleRange::Normal,
            Self::Italic => ComputedFontStyleRange::Italic,
            Self::Oblique(angle1, angle2) => {
                let angle1 = angle1.compute();
                let angle2 = angle2.compute();
                if angle1.deg > angle2.deg {
                    ComputedFontStyleRange::Oblique(angle2, angle1)
                } else {
                    ComputedFontStyleRange::Oblique(angle1, angle2)
                }
            }
        }
    }
}

impl FromCss for SpecifiedFontStyleRange {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        try_match_next_ident_ignore_ascii_case! { input,
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
        }
        .map_err(Into::into)
    }
}

impl ToCss for SpecifiedFontStyleRange {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedFontStyleRange {
    Normal,
    Italic,
    Oblique(ComputedAngle, ComputedAngle),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpecifiedFontWeightRange(
    pub SpecifiedAbsoluteFontWeight,
    pub SpecifiedAbsoluteFontWeight,
);

impl SpecifiedFontWeightRange {
    pub fn compute(self) -> ComputedFontWeightRange {
        let value1 = self.0.compute().0;
        let value2 = self.1.compute().0;
        if value1 > value2 {
            ComputedFontWeightRange(value2, value1)
        } else {
            ComputedFontWeightRange(value1, value2)
        }
    }
}

impl FromCss for SpecifiedFontWeightRange {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let value1 = input
            .try_parse(SpecifiedAbsoluteFontWeight::from_css)
            .unwrap_or_default();
        let value2 = input
            .try_parse(SpecifiedAbsoluteFontWeight::from_css)
            .unwrap_or(value1);
        Ok(Self(value1, value2))
    }
}

impl ToCss for SpecifiedFontWeightRange {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.0 != self.1 {
            write!(dest, " {}", CssValue(&self.1))?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedFontWeightRange(pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpecifiedFontWidthRange(pub SpecifiedFontWidth, pub SpecifiedFontWidth);

impl SpecifiedFontWidthRange {
    pub fn compute(self) -> ComputedFontWidthRange {
        let value1 = self.0.compute().0;
        let value2 = self.1.compute().0;
        if value1 > value2 {
            ComputedFontWidthRange(value2, value1)
        } else {
            ComputedFontWidthRange(value1, value2)
        }
    }
}

impl FromCss for SpecifiedFontWidthRange {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let value1 = input
            .try_parse(SpecifiedFontWidth::from_css)
            .unwrap_or_default();
        let value2 = input
            .try_parse(SpecifiedFontWidth::from_css)
            .unwrap_or(value1);
        Ok(Self(value1, value2))
    }
}

impl ToCss for SpecifiedFontWidthRange {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.0 != self.1 {
            write!(dest, " {}", CssValue(&self.1))?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedFontWidthRange(pub f32, pub f32);

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

fn parse_opentype_tag<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<[u8; 4], ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    let tag = input.expect_string()?;
    if !(tag.len() == 4 && tag.as_bytes().iter().all(|b| (0x20..=0x7e).contains(b))) {
        return Err(location.new_unexpected_token_error(Token::QuotedString(tag.clone())));
    }
    Ok(tag.as_bytes().try_into().unwrap())
}

#[derive(Clone, Copy, Debug)]
pub struct SpecifiedFeatureTagValue {
    pub tag: [u8; 4],
    pub value: u32,
}

impl FromCss for SpecifiedFeatureTagValue {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        fn parse_value(input: &mut Parser) -> u32 {
            if let Ok(v) = input.try_parse(|input| parse_integer_with_range(input, 0..)) {
                return v as u32;
            }
            if let Ok(v) = input.try_parse(|input| {
                try_match_next_ident_ignore_ascii_case! { input,
                    "on" => 1,
                    "off" => 0,
                }
            }) {
                return v;
            }
            1
        }

        let tag = parse_opentype_tag(input)?;
        let value = parse_value(input);
        Ok(Self { tag, value })
    }
}

impl ToCss for SpecifiedFeatureTagValue {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_string(str::from_utf8(&self.tag).unwrap_or_default(), dest)?;
        if self.value != 1 {
            write!(dest, " {}", self.value)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct SpecifiedFontFeatureSettings {
    pub feature_tag_value_list: Option<Rc<[SpecifiedFeatureTagValue]>>,
}

impl FromCss for SpecifiedFontFeatureSettings {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        input.skip_whitespace();
        let feature_tag_value_list =
            match input.try_parse(|input| input.expect_ident_matching("normal")) {
                Ok(_) => None,
                Err(_) => Some(
                    input
                        .parse_comma_separated(SpecifiedFeatureTagValue::from_css)?
                        .into(),
                ),
            };
        Ok(Self {
            feature_tag_value_list,
        })
    }
}

impl ToCss for SpecifiedFontFeatureSettings {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self.feature_tag_value_list {
            Some(ref v) => {
                let mut iter = v.iter();
                iter.next().unwrap().to_css(dest)?;
                for name in iter {
                    write!(dest, ", {}", CssValue(name))?;
                }
                Ok(())
            }
            None => dest.write_str("normal"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SpecifiedVariationValue {
    pub tag: [u8; 4],
    pub value: f32,
}

impl FromCss for SpecifiedVariationValue {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let tag = parse_opentype_tag(input)?;
        let value = parse_number(input)?;
        Ok(Self { tag, value })
    }
}

impl ToCss for SpecifiedVariationValue {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        write!(
            dest,
            "{} {}",
            CssString(str::from_utf8(&self.tag).unwrap_or_default()),
            CssNumber(self.value)
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct SpecifiedFontVariationSettings {
    pub variation_value_list: Option<Rc<[SpecifiedVariationValue]>>,
}

impl FromCss for SpecifiedFontVariationSettings {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        input.skip_whitespace();
        let variation_value_list =
            match input.try_parse(|input| input.expect_ident_matching("normal")) {
                Ok(_) => None,
                Err(_) => Some(
                    input
                        .parse_comma_separated(SpecifiedVariationValue::from_css)?
                        .into(),
                ),
            };
        Ok(Self {
            variation_value_list,
        })
    }
}

impl ToCss for SpecifiedFontVariationSettings {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self.variation_value_list {
            Some(ref v) => {
                let mut iter = v.iter();
                iter.next().unwrap().to_css(dest)?;
                for name in iter {
                    write!(dest, ", {}", CssValue(name))?;
                }
                Ok(())
            }
            None => dest.write_str("normal"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpecifiedFontDisplay {
    #[default]
    Auto,
    Block,
    Swap,
    Fallback,
    Optional,
}

impl FromCss for SpecifiedFontDisplay {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        try_match_next_ident_ignore_ascii_case! { input,
            "auto" => Self::Auto,
            "block" => Self::Block,
            "swap" => Self::Swap,
            "fallback" => Self::Fallback,
            "optional" => Self::Optional,
        }
        .map_err(Into::into)
    }
}

impl ToCss for SpecifiedFontDisplay {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Auto => dest.write_str("auto"),
            Self::Block => dest.write_str("block"),
            Self::Swap => dest.write_str("swap"),
            Self::Fallback => dest.write_str("fallback"),
            Self::Optional => dest.write_str("optional"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpecifiedMetricsOverrideValue {
    #[default]
    Normal,
    Percentage(f32),
}

impl FromCss for SpecifiedMetricsOverrideValue {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "normal" => Self::Normal,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
                Self::Percentage(unit_value)
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for SpecifiedMetricsOverrideValue {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Percentage(v) => CssPercentage(v).to_css(dest),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SpecifiedMetricsOverride(
    pub SpecifiedMetricsOverrideValue,
    pub SpecifiedMetricsOverrideValue,
);

impl FromCss for SpecifiedMetricsOverride {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let value1 = SpecifiedMetricsOverrideValue::from_css(input)?;
        let value2 = input
            .try_parse(SpecifiedMetricsOverrideValue::from_css)
            .unwrap_or_default();
        Ok(Self(value1, value2))
    }
}

impl ToCss for SpecifiedMetricsOverride {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.1 != SpecifiedMetricsOverrideValue::Normal {
            write!(dest, " {}", CssValue(&self.1))?;
        }
        Ok(())
    }
}
