use std::convert::Infallible;
use std::fmt;
use std::rc::Rc;

use cssparser::{
    match_ignore_ascii_case, serialize_string, BasicParseError, ParseError, Parser, ParserInput,
    ToCss, Token,
};

use super::angle::SpecifiedAngle;
use super::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::{parse_number_with_range, parse_string, CssNumber, CssPercentage, CssValue};

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedFontStyle {
    Normal,
    Italic,
    Oblique(SpecifiedAngle),
}

impl SpecifiedFontStyle {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "italic" => Self::Italic,
            "oblique" => {
                let angle = input
                    .try_parse(|input| SpecifiedAngle::parse_with_range(input, -90.0, 90.0))
                    .unwrap_or(SpecifiedAngle::Deg(14.0));
                Self::Oblique(angle)
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
            Self::Oblique(angle) => {
                dest.write_str("oblique")?;
                if !matches!(angle, SpecifiedAngle::Deg(deg) if deg == 14.0) {
                    write!(dest, " {}", CssValue(&angle))?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedFontVariantCss2 {
    Normal,
    SmallCaps,
}

impl SpecifiedFontVariantCss2 {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "small-caps" => Self::SmallCaps,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for SpecifiedFontVariantCss2 {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::SmallCaps => dest.write_str("small-caps"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedAbsoluteFontWeight {
    Number(f32),
    Normal,
    Bold,
}

impl SpecifiedAbsoluteFontWeight {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input.try_parse(|input| parse_number_with_range(input, 1.0, 1000.0)) {
            return Ok(Self::Number(v));
        }
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "bold" => Self::Bold,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for SpecifiedAbsoluteFontWeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Number(v) => write!(dest, "{}", CssNumber(v)),
            Self::Normal => dest.write_str("normal"),
            Self::Bold => dest.write_str("bold"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedFontWeight {
    Absolute(SpecifiedAbsoluteFontWeight),
    Bolder,
    Lighter,
}

impl SpecifiedFontWeight {
    pub fn normal() -> Self {
        Self::Absolute(SpecifiedAbsoluteFontWeight::Normal)
    }

    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input.try_parse(SpecifiedAbsoluteFontWeight::parse) {
            return Ok(Self::Absolute(v));
        }
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "bolder" => Self::Bolder,
            "lighter" => Self::Lighter,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for SpecifiedFontWeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Absolute(v) => v.to_css(dest),
            Self::Bolder => dest.write_str("bolder"),
            Self::Lighter => dest.write_str("lighter"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedFontStretchCss3 {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl SpecifiedFontStretchCss3 {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "ultra-condensed" => Self::UltraCondensed,
            "extra-condensed" => Self::ExtraCondensed,
            "condensed" => Self::Condensed,
            "semi-condensed" => Self::SemiCondensed,
            "semi-expanded" => Self::SemiExpanded,
            "expanded" => Self::Expanded,
            "extra-expanded" => Self::ExtraExpanded,
            "ultra-expanded" => Self::UltraExpanded,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for SpecifiedFontStretchCss3 {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::UltraCondensed => dest.write_str("ultra-condensed"),
            Self::ExtraCondensed => dest.write_str("extra-condensed"),
            Self::Condensed => dest.write_str("condensed"),
            Self::SemiCondensed => dest.write_str("semi-condensed"),
            Self::SemiExpanded => dest.write_str("semi-expanded"),
            Self::Expanded => dest.write_str("expanded"),
            Self::ExtraExpanded => dest.write_str("extra-expanded"),
            Self::UltraExpanded => dest.write_str("ultra-expanded"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedAbsoluteFontSize {
    Length(SpecifiedAbsoluteLength),
    Percentage(f32),
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
}

impl SpecifiedAbsoluteFontSize {
    pub fn compute(self) -> ComputedLength {
        match self {
            Self::Length(v) => v.compute(),
            Self::Percentage(v) => ComputedLength {
                px: v as f64 * 10.0,
            },
            Self::XxSmall => ComputedLength { px: 9.6 },
            Self::XSmall => ComputedLength { px: 12.0 },
            Self::Small => ComputedLength { px: 128.0 / 9.0 },
            Self::Medium => ComputedLength { px: 16.0 },
            Self::Large => ComputedLength { px: 19.2 },
            Self::XLarge => ComputedLength { px: 24.0 },
            Self::XxLarge => ComputedLength { px: 32.0 },
            Self::XxxLarge => ComputedLength { px: 48.0 },
        }
    }

    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input
            .try_parse(|input| SpecifiedAbsoluteLength::parse_with_range(input, 0.0, f64::INFINITY))
        {
            return Ok(Self::Length(v));
        }
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "xx-small" => Self::XxSmall,
                "x-small" => Self::XSmall,
                "small" => Self::Small,
                "medium" => Self::Medium,
                "large" => Self::Large,
                "x-large" => Self::XLarge,
                "xx-large" => Self::XxLarge,
                "xxx-large" => Self::XxxLarge,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
                Self::Percentage(unit_value)
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for SpecifiedAbsoluteFontSize {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Length(v) => v.to_css(dest),
            Self::Percentage(v) => write!(dest, "{}", CssPercentage(v)),
            Self::XxSmall => dest.write_str("xx-small"),
            Self::XSmall => dest.write_str("x-small"),
            Self::Small => dest.write_str("small"),
            Self::Medium => dest.write_str("medium"),
            Self::Large => dest.write_str("large"),
            Self::XLarge => dest.write_str("x-large"),
            Self::XxLarge => dest.write_str("xx-large"),
            Self::XxxLarge => dest.write_str("xxx-large"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedFontSize {
    Absolute(SpecifiedAbsoluteFontSize),
    Larger,
    Smaller,
    Math,
}

impl SpecifiedFontSize {
    pub fn compute(self) -> ComputedLength {
        match self {
            Self::Absolute(v) => v.compute(),
            Self::Larger => ComputedLength { px: 12.0 },
            Self::Smaller => ComputedLength { px: 25.0 / 3.0 },
            Self::Math => ComputedLength { px: 10.0 },
        }
    }

    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input.try_parse(SpecifiedAbsoluteFontSize::parse) {
            return Ok(Self::Absolute(v));
        }
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "larger" => Self::Larger,
            "smaller" => Self::Smaller,
            "math" => Self::Math,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for SpecifiedFontSize {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Absolute(v) => v.to_css(dest),
            Self::Larger => todo!(),
            Self::Smaller => todo!(),
            Self::Math => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedLineHeight {
    Normal,
    Length(SpecifiedAbsoluteLength),
    Number(f32),
    Percentage(f32),
}

impl SpecifiedLineHeight {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input
            .try_parse(|input| SpecifiedAbsoluteLength::parse_with_range(input, 0.0, f64::INFINITY))
        {
            return Ok(Self::Length(v));
        }
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "normal" => Self::Normal,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Number { value, .. } if value >= 0.0 => Self::Number(value),
            Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
                Self::Percentage(unit_value)
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for SpecifiedLineHeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Length(v) => v.to_css(dest),
            Self::Number(v) => write!(dest, "{}", CssNumber(v)),
            Self::Percentage(v) => write!(dest, "{}", CssPercentage(v)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedGenericFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Math,
    Fangsong,
    Kai,
    Nastaliq,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
}

impl SpecifiedGenericFamily {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "serif" => Self::Serif,
                "sans-serif" => Self::SansSerif,
                "cursive" => Self::Cursive,
                "fantasy" => Self::Fantasy,
                "monospace" => Self::Monospace,
                "system-ui" => Self::SystemUi,
                "math" => Self::Math,
                "ui-serif" => Self::UiSerif,
                "ui-sans-serif" => Self::UiSansSerif,
                "ui-monospace" => Self::UiMonospace,
                "ui-rounded" => Self::UiRounded,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Function(ref name) => {
                if !name.eq_ignore_ascii_case("generic") {
                    return Err(location.new_unexpected_token_error(Token::Ident(name.clone())));
                }
                input.parse_nested_block(|input| {
                    let ident = input.expect_ident()?;
                    Ok(match_ignore_ascii_case! { ident,
                        "fangsong" => Self::Fangsong,
                        "kai" => Self::Kai,
                        "nastaliq" => Self::Nastaliq,
                        _ => {
                            return Err(
                                location.new_unexpected_token_error(Token::Ident(ident.clone()))
                            )
                        }
                    })
                })?
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for SpecifiedGenericFamily {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Serif => dest.write_str("serif"),
            Self::SansSerif => dest.write_str("sans-serif"),
            Self::Cursive => dest.write_str("cursive"),
            Self::Fantasy => dest.write_str("fantasy"),
            Self::Monospace => dest.write_str("monospace"),
            Self::SystemUi => dest.write_str("system-ui"),
            Self::Math => dest.write_str("math"),
            Self::Fangsong => dest.write_str("generic(fangsong)"),
            Self::Kai => dest.write_str("generic(kai)"),
            Self::Nastaliq => dest.write_str("generic(nastaliq)"),
            Self::UiSerif => dest.write_str("ui-serif"),
            Self::UiSansSerif => dest.write_str("ui-sans-serif"),
            Self::UiMonospace => dest.write_str("ui-monospace"),
            Self::UiRounded => dest.write_str("ui-rounded"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SpecifiedFamilyName {
    Specific(Rc<str>),
    Generic(SpecifiedGenericFamily),
}

impl SpecifiedFamilyName {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(name) = input.try_parse(parse_string) {
            return Ok(Self::Specific(name));
        }
        if let Ok(v) = input.try_parse(SpecifiedGenericFamily::parse) {
            return Ok(Self::Generic(v));
        }
        let mut name = input.expect_ident()?.as_ref().to_owned();
        match_ignore_ascii_case! { &name,
            "initial" | "inherit" | "unset" | "revert" | "default" => {
                let ident = input.expect_ident()?;
                name.push(' ');
                name.push_str(ident);
            },
            _ => {},
        }
        while let Ok(ref ident) = input.try_parse(Parser::expect_ident_cloned) {
            name.push(' ');
            name.push_str(ident);
        }
        Ok(Self::Specific(name.into()))
    }
}

impl ToCss for SpecifiedFamilyName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Specific(ref name) => serialize_string(name, dest),
            Self::Generic(v) => v.to_css(dest),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpecifiedFontFamily {
    pub family_list: Box<[SpecifiedFamilyName]>,
}

impl SpecifiedFontFamily {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let family_list = input
            .parse_comma_separated(SpecifiedFamilyName::parse)?
            .into_boxed_slice();
        Ok(Self { family_list })
    }
}

impl ToCss for SpecifiedFontFamily {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.family_list.iter();
        iter.next().unwrap().to_css(dest)?;
        for name in iter {
            write!(dest, ", {}", CssValue(name))?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SpecifiedFont {
    pub font_style: SpecifiedFontStyle,
    pub font_variant: SpecifiedFontVariantCss2,
    pub font_weight: SpecifiedFontWeight,
    pub font_stretch: SpecifiedFontStretchCss3,
    pub font_size: SpecifiedFontSize,
    pub line_height: SpecifiedLineHeight,
    pub font_family: SpecifiedFontFamily,
}

impl SpecifiedFont {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let mut max_attr = 4;
        let mut font_style = None;
        let mut font_variant = None;
        let mut font_weight = None;
        let mut font_stretch = None;
        while max_attr != 0 {
            if input
                .try_parse(|input| input.expect_ident_matching("normal"))
                .is_ok()
            {
                max_attr -= 1;
                continue;
            }
            if font_style.is_none() {
                if let Ok(v) = input.try_parse(SpecifiedFontStyle::parse) {
                    font_style = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            if font_variant.is_none() {
                if let Ok(v) = input.try_parse(SpecifiedFontVariantCss2::parse) {
                    font_variant = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            if font_weight.is_none() {
                if let Ok(v) = input.try_parse(SpecifiedFontWeight::parse) {
                    font_weight = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            if font_stretch.is_none() {
                if let Ok(v) = input.try_parse(SpecifiedFontStretchCss3::parse) {
                    font_stretch = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            break;
        }
        let font_style = font_style.unwrap_or(SpecifiedFontStyle::Normal);
        let font_variant = font_variant.unwrap_or(SpecifiedFontVariantCss2::Normal);
        let font_weight = font_weight.unwrap_or(SpecifiedFontWeight::normal());
        let font_stretch = font_stretch.unwrap_or(SpecifiedFontStretchCss3::Normal);
        let font_size = SpecifiedFontSize::parse(input)?;
        let line_height = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            SpecifiedLineHeight::parse(input)?
        } else {
            SpecifiedLineHeight::Normal
        };
        let font_family = SpecifiedFontFamily::parse(input)?;
        Ok(Self {
            font_style,
            font_variant,
            font_weight,
            font_stretch,
            font_size,
            line_height,
            font_family,
        })
    }
}

impl ToCss for SpecifiedFont {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if !matches!(self.font_style, SpecifiedFontStyle::Normal) {
            write!(dest, "{} ", CssValue(&self.font_style))?;
        }
        if !matches!(self.font_variant, SpecifiedFontVariantCss2::Normal) {
            write!(dest, "{} ", CssValue(&self.font_variant))?;
        }
        if !matches!(
            self.font_weight,
            SpecifiedFontWeight::Absolute(SpecifiedAbsoluteFontWeight::Normal)
        ) {
            write!(dest, "{} ", CssValue(&self.font_weight))?;
        }
        if !matches!(self.font_stretch, SpecifiedFontStretchCss3::Normal) {
            write!(dest, "{} ", CssValue(&self.font_stretch))?;
        }
        write!(dest, "{} ", CssValue(&self.font_size))?;
        if !matches!(self.line_height, SpecifiedLineHeight::Normal) {
            write!(dest, "/ {} ", CssValue(&self.line_height))?;
        }
        self.font_family.to_css(dest)?;
        Ok(())
    }
}

pub fn parse_font(css: &str) -> Result<SpecifiedFont, BasicParseError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    parser
        .parse_entirely(SpecifiedFont::parse)
        .map_err(ParseError::basic)
}
