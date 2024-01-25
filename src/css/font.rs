use std::convert::Infallible;
use std::fmt;
use std::rc::Rc;

use cssparser::{
    match_ignore_ascii_case, serialize_string, BasicParseError, ParseError, Parser, ParserInput,
    ToCss, Token,
};

use super::angle::{ComputedAngle, SpecifiedAngle};
use super::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::{parse_string, CssNumber, CssValue};

#[derive(Clone, Copy, Debug)]
pub enum ComputedFontStyle {
    Normal,
    Italic,
    Oblique(ComputedAngle),
}

impl ComputedFontStyle {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "italic" => Self::Italic,
            "oblique" => {
                let angle = match input
                    .try_parse(|input| SpecifiedAngle::parse_with_range(input, -90.0, 90.0))
                {
                    Ok(v) => v.compute(),
                    Err(_) => ComputedAngle { deg: 14.0 },
                };
                Self::Oblique(angle)
            },
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for ComputedFontStyle {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(angle) => {
                dest.write_str("oblique")?;
                if !matches!(angle, ComputedAngle { deg } if deg == 14.0) {
                    write!(dest, " {}", CssValue(&angle))?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComputedFontVariantCss2 {
    Normal,
    SmallCaps,
}

impl ComputedFontVariantCss2 {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { ident,
            "normal" => Self::Normal,
            "small-caps" => Self::SmallCaps,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        })
    }
}

impl ToCss for ComputedFontVariantCss2 {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::SmallCaps => dest.write_str("small-caps"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedFontWeight(pub f32);

impl ComputedFontWeight {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "normal" => Self(400.0),
                "bold" => Self(700.0),
                "bolder" => Self(700.0),
                "lighter" => Self(100.0),
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Number { value, .. } if (1.0..=1000.0).contains(&value) => Self(value),
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for ComputedFontWeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        CssNumber(self.0).to_css(dest)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComputedFontStretchCss3 {
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

impl ComputedFontStretchCss3 {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
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

impl ToCss for ComputedFontStretchCss3 {
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
pub struct ComputedFontSize(pub ComputedLength);

impl ComputedFontSize {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input
            .try_parse(|input| SpecifiedAbsoluteLength::parse_with_range(input, 0.0, f32::INFINITY))
        {
            return Ok(Self(v.compute()));
        }
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "xx-small" => Self(ComputedLength { px: 9.6 }),
                "x-small" => Self(ComputedLength { px: 12.0 }),
                "small" => Self(ComputedLength { px: 128.0 / 9.0 }),
                "medium" => Self(ComputedLength { px: 16.0 }),
                "large" => Self(ComputedLength { px: 19.2 }),
                "x-large" => Self(ComputedLength { px: 24.0 }),
                "xx-large" => Self(ComputedLength { px: 32.0 }),
                "xxx-large" => Self(ComputedLength { px: 48.0 }),
                "larger" => Self(ComputedLength { px: 12.0 }),
                "smaller" => Self(ComputedLength { px: 25.0 / 3.0 }),
                "math" => Self(ComputedLength { px: 10.0 }),
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Percentage { unit_value, .. } if unit_value >= 0.0 => Self(ComputedLength {
                px: unit_value * 10.0,
            }),
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for ComputedFontSize {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComputedLineHeight {
    Normal,
    Length(ComputedLength),
    Number(f32),
}

impl ComputedLineHeight {
    pub fn parse_and_compute<'i>(
        input: &mut Parser<'i, '_>,
        font_size: ComputedFontSize,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) = input
            .try_parse(|input| SpecifiedAbsoluteLength::parse_with_range(input, 0.0, f32::INFINITY))
        {
            return Ok(Self::Length(v.compute()));
        }
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "normal" => Self::Normal,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Number { value, .. } if value >= 0.0 => Self::Number(value),
            Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
                Self::Length(ComputedLength {
                    px: unit_value * font_size.0.px,
                })
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for ComputedLineHeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Length(v) => v.to_css(dest),
            Self::Number(v) => CssNumber(v).to_css(dest),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComputedGenericFamily {
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

impl ComputedGenericFamily {
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

impl ToCss for ComputedGenericFamily {
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
pub enum ComputedFamilyName {
    Specific(Rc<str>),
    Generic(ComputedGenericFamily),
}

impl ComputedFamilyName {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(name) = input.try_parse(parse_string) {
            return Ok(Self::Specific(name));
        }
        if let Ok(v) = input.try_parse(ComputedGenericFamily::parse) {
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

impl ToCss for ComputedFamilyName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Specific(ref name) => serialize_string(name, dest),
            Self::Generic(v) => v.to_css(dest),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComputedFontFamily {
    pub family_list: Box<[ComputedFamilyName]>,
}

impl ComputedFontFamily {
    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let family_list = input
            .parse_comma_separated(ComputedFamilyName::parse)?
            .into_boxed_slice();
        Ok(Self { family_list })
    }
}

impl ToCss for ComputedFontFamily {
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
pub struct ComputedFont {
    pub font_style: ComputedFontStyle,
    pub font_variant: ComputedFontVariantCss2,
    pub font_weight: ComputedFontWeight,
    pub font_stretch: ComputedFontStretchCss3,
    pub font_size: ComputedFontSize,
    pub line_height: ComputedLineHeight,
    pub font_family: ComputedFontFamily,
}

impl ComputedFont {
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
                if let Ok(v) = input.try_parse(ComputedFontStyle::parse_and_compute) {
                    font_style = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            if font_variant.is_none() {
                if let Ok(v) = input.try_parse(ComputedFontVariantCss2::parse_and_compute) {
                    font_variant = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            if font_weight.is_none() {
                if let Ok(v) = input.try_parse(ComputedFontWeight::parse_and_compute) {
                    font_weight = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            if font_stretch.is_none() {
                if let Ok(v) = input.try_parse(ComputedFontStretchCss3::parse_and_compute) {
                    font_stretch = Some(v);
                    max_attr -= 1;
                    continue;
                }
            }
            break;
        }
        let font_style = font_style.unwrap_or(ComputedFontStyle::Normal);
        let font_variant = font_variant.unwrap_or(ComputedFontVariantCss2::Normal);
        let font_weight = font_weight.unwrap_or(ComputedFontWeight(400.0));
        let font_stretch = font_stretch.unwrap_or(ComputedFontStretchCss3::Normal);
        let font_size = ComputedFontSize::parse_and_compute(input)?;
        let line_height = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            ComputedLineHeight::parse_and_compute(input, font_size)?
        } else {
            ComputedLineHeight::Normal
        };
        let font_family = ComputedFontFamily::parse(input)?;
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

impl ToCss for ComputedFont {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if !matches!(self.font_style, ComputedFontStyle::Normal) {
            write!(dest, "{} ", CssValue(&self.font_style))?;
        }
        if !matches!(self.font_variant, ComputedFontVariantCss2::Normal) {
            write!(dest, "{} ", CssValue(&self.font_variant))?;
        }
        if !matches!(self.font_weight, ComputedFontWeight(v) if v == 400.0) {
            write!(dest, "{} ", CssValue(&self.font_weight))?;
        }
        if !matches!(self.font_stretch, ComputedFontStretchCss3::Normal) {
            write!(dest, "{} ", CssValue(&self.font_stretch))?;
        }
        write!(dest, "{} ", CssValue(&self.font_size))?;
        if !matches!(self.line_height, ComputedLineHeight::Normal) {
            write!(dest, "/ {} ", CssValue(&self.line_height))?;
        }
        self.font_family.to_css(dest)?;
        Ok(())
    }
}

pub fn parse_and_compute_font(css: &str) -> Result<ComputedFont, BasicParseError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    parser
        .parse_entirely(ComputedFont::parse)
        .map_err(ParseError::basic)
}
