pub mod font_face;

use std::convert::Infallible;
use std::fmt;
use std::rc::Rc;

use cssparser::{ParseError, Parser, ToCss, Token, match_ignore_ascii_case, serialize_string};

use super::angle::{ComputedAngle, SpecifiedAngle};
use super::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::{
    CssNumber, CssPercentage, CssValue, FromCss, parse_string,
    try_match_next_ident_ignore_ascii_case,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedFontStyle {
    Normal,
    Italic,
    Oblique(ComputedAngle),
}

impl FromCss for ComputedFontStyle {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        try_match_next_ident_ignore_ascii_case! { input,
            "normal" => Self::Normal,
            "italic" => Self::Italic,
            "oblique" => {
                let angle = match input
                    .try_parse(|input| SpecifiedAngle::from_css_with_range(input, -90.0..=90.0))
                {
                    Ok(v) => v.compute(),
                    Err(_) => ComputedAngle { deg: 14.0 },
                };
                Self::Oblique(angle)
            },
        }
        .map_err(Into::into)
    }
}

impl ToCss for ComputedFontStyle {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(angle) => {
                dest.write_str("oblique")?;
                if angle.deg != 14.0 {
                    write!(dest, " {}", CssValue(&angle))?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputedFontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

impl ComputedFontVariantCaps {
    pub fn to_css2(self) -> Option<ComputedFontVariantCss2> {
        Some(match self {
            Self::Normal => ComputedFontVariantCss2::Normal,
            Self::SmallCaps => ComputedFontVariantCss2::SmallCaps,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputedFontVariantCss2 {
    Normal,
    SmallCaps,
}

impl ComputedFontVariantCss2 {
    pub fn modernize(self) -> ComputedFontVariantCaps {
        match self {
            Self::Normal => ComputedFontVariantCaps::Normal,
            Self::SmallCaps => ComputedFontVariantCaps::SmallCaps,
        }
    }
}

impl FromCss for ComputedFontVariantCss2 {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        try_match_next_ident_ignore_ascii_case! { input,
            "normal" => Self::Normal,
            "small-caps" => Self::SmallCaps,
        }
        .map_err(Into::into)
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpecifiedAbsoluteFontWeight {
    #[default]
    Normal,
    Bold,
    Number(f32),
}

impl SpecifiedAbsoluteFontWeight {
    pub fn compute(self) -> ComputedFontWeight {
        match self {
            Self::Normal => ComputedFontWeight(400.0),
            Self::Bold => ComputedFontWeight(700.0),
            Self::Number(v) => ComputedFontWeight(v),
        }
    }
}

impl FromCss for SpecifiedAbsoluteFontWeight {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
                "normal" => Self::Normal,
                "bold" => Self::Bold,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            },
            Token::Number { value, .. } if (1.0..=1000.0).contains(&value) => Self::Number(value),
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for SpecifiedAbsoluteFontWeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Bold => dest.write_str("bold"),
            Self::Number(value) => CssNumber(value).to_css(dest),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedFontWeight(pub f32);

impl FromCss for ComputedFontWeight {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpecifiedFontWidth {
    #[default]
    Normal,
    Percentage(f32),
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl SpecifiedFontWidth {
    pub fn compute(self) -> ComputedFontWidth {
        match self {
            Self::Normal => ComputedFontWidth(1.0),
            Self::Percentage(v) => ComputedFontWidth(v),
            Self::UltraCondensed => ComputedFontWidth(0.5),
            Self::ExtraCondensed => ComputedFontWidth(0.625),
            Self::Condensed => ComputedFontWidth(0.75),
            Self::SemiCondensed => ComputedFontWidth(0.875),
            Self::SemiExpanded => ComputedFontWidth(1.125),
            Self::Expanded => ComputedFontWidth(1.25),
            Self::ExtraExpanded => ComputedFontWidth(1.5),
            Self::UltraExpanded => ComputedFontWidth(2.0),
        }
    }
}

impl FromCss for SpecifiedFontWidth {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Ident(ref ident) => match_ignore_ascii_case! { ident,
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
            },
            Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
                Self::Percentage(unit_value)
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl ToCss for SpecifiedFontWidth {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Normal => dest.write_str("normal"),
            Self::Percentage(v) => CssPercentage(v).to_css(dest),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedFontWidth(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn modernize(self) -> ComputedFontWidth {
        match self {
            Self::Normal => ComputedFontWidth(1.0),
            Self::UltraCondensed => ComputedFontWidth(0.5),
            Self::ExtraCondensed => ComputedFontWidth(0.625),
            Self::Condensed => ComputedFontWidth(0.75),
            Self::SemiCondensed => ComputedFontWidth(0.875),
            Self::SemiExpanded => ComputedFontWidth(1.125),
            Self::Expanded => ComputedFontWidth(1.25),
            Self::ExtraExpanded => ComputedFontWidth(1.5),
            Self::UltraExpanded => ComputedFontWidth(2.0),
        }
    }
}

impl FromCss for ComputedFontStretchCss3 {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        try_match_next_ident_ignore_ascii_case! { input,
            "normal" => Self::Normal,
            "ultra-condensed" => Self::UltraCondensed,
            "extra-condensed" => Self::ExtraCondensed,
            "condensed" => Self::Condensed,
            "semi-condensed" => Self::SemiCondensed,
            "semi-expanded" => Self::SemiExpanded,
            "expanded" => Self::Expanded,
            "extra-expanded" => Self::ExtraExpanded,
            "ultra-expanded" => Self::UltraExpanded,
        }
        .map_err(Into::into)
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

impl FromCss for ComputedFontSize {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        if let Ok(v) =
            input.try_parse(|input| SpecifiedAbsoluteLength::from_css_with_range(input, 0.0..))
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComputedLineHeight {
    Normal,
    Length(ComputedLength),
    Number(f32),
}

impl ComputedLineHeight {
    pub fn from_css<'i>(
        input: &mut Parser<'i, '_>,
        font_size: ComputedFontSize,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        if let Ok(v) =
            input.try_parse(|input| SpecifiedAbsoluteLength::from_css_with_range(input, 0.0..))
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

#[derive(Clone, Debug)]
pub struct SpecifiedSpecificFamily {
    pub name: Rc<str>,
}

impl SpecifiedSpecificFamily {
    pub fn compute(self) -> ComputedSpecificFamily {
        ComputedSpecificFamily { name: self.name }
    }
}

impl FromCss for SpecifiedSpecificFamily {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        if let Ok(name) = input.try_parse(parse_string) {
            return Ok(Self { name });
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
        Ok(Self { name: name.into() })
    }
}

impl ToCss for SpecifiedSpecificFamily {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_string(&self.name, dest)
    }
}

#[derive(Clone, Debug)]
pub struct ComputedSpecificFamily {
    pub name: Rc<str>,
}

impl FromCss for ComputedSpecificFamily {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        Ok(SpecifiedSpecificFamily::from_css(input)?.compute())
    }
}

impl ToCss for ComputedSpecificFamily {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_string(&self.name, dest)
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

impl FromCss for ComputedGenericFamily {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
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
                    try_match_next_ident_ignore_ascii_case! { input,
                        "fangsong" => Self::Fangsong,
                        "kai" => Self::Kai,
                        "nastaliq" => Self::Nastaliq,
                    }
                    .map_err(Into::into)
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
    Specific(ComputedSpecificFamily),
    Generic(ComputedGenericFamily),
}

impl FromCss for ComputedFamilyName {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        if let Ok(v) = input.try_parse(ComputedGenericFamily::from_css) {
            return Ok(Self::Generic(v));
        }
        Ok(Self::Specific(ComputedSpecificFamily::from_css(input)?))
    }
}

impl ToCss for ComputedFamilyName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Self::Specific(ref v) => v.to_css(dest),
            Self::Generic(v) => v.to_css(dest),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComputedFontFamily {
    pub family_list: Rc<[ComputedFamilyName]>,
}

impl FromCss for ComputedFontFamily {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let family_list = input
            .parse_comma_separated(ComputedFamilyName::from_css)?
            .into();
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
    pub style: ComputedFontStyle,
    pub variant: ComputedFontVariantCss2,
    pub weight: ComputedFontWeight,
    pub stretch: ComputedFontStretchCss3,
    pub size: ComputedFontSize,
    pub line_height: ComputedLineHeight,
    pub family: ComputedFontFamily,
}

impl FromCss for ComputedFont {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let mut max_attrs = 4;
        let mut style = None;
        let mut variant = None;
        let mut weight = None;
        let mut stretch = None;
        while max_attrs != 0 {
            if input
                .try_parse(|input| input.expect_ident_matching("normal"))
                .is_ok()
            {
                max_attrs -= 1;
                continue;
            }
            if style.is_none()
                && let Ok(v) = input.try_parse(ComputedFontStyle::from_css)
            {
                style = Some(v);
                max_attrs -= 1;
                continue;
            }
            if variant.is_none()
                && let Ok(v) = input.try_parse(ComputedFontVariantCss2::from_css)
            {
                variant = Some(v);
                max_attrs -= 1;
                continue;
            }
            if weight.is_none()
                && let Ok(v) = input.try_parse(ComputedFontWeight::from_css)
            {
                weight = Some(v);
                max_attrs -= 1;
                continue;
            }
            if stretch.is_none()
                && let Ok(v) = input.try_parse(|input| {
                    try_match_next_ident_ignore_ascii_case! { input,
                        "normal" => ComputedFontStretchCss3::Normal,
                        "ultra-condensed" => ComputedFontStretchCss3::UltraCondensed,
                        "extra-condensed" => ComputedFontStretchCss3::ExtraCondensed,
                        "condensed" => ComputedFontStretchCss3::Condensed,
                        "semi-condensed" => ComputedFontStretchCss3::SemiCondensed,
                        "semi-expanded" => ComputedFontStretchCss3::SemiExpanded,
                        "expanded" => ComputedFontStretchCss3::Expanded,
                        "extra-expanded" => ComputedFontStretchCss3::ExtraExpanded,
                        "ultra-expanded" => ComputedFontStretchCss3::UltraExpanded,
                    }
                })
            {
                stretch = Some(v);
                max_attrs -= 1;
                continue;
            }
            break;
        }
        let style = style.unwrap_or(ComputedFontStyle::Normal);
        let variant = variant.unwrap_or(ComputedFontVariantCss2::Normal);
        let weight = weight.unwrap_or(ComputedFontWeight(400.0));
        let stretch = stretch.unwrap_or(ComputedFontStretchCss3::Normal);
        let size = ComputedFontSize::from_css(input)?;
        let line_height = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            ComputedLineHeight::from_css(input, size)?
        } else {
            ComputedLineHeight::Normal
        };
        let family = ComputedFontFamily::from_css(input)?;
        Ok(Self {
            style,
            variant,
            weight,
            stretch,
            size,
            line_height,
            family,
        })
    }
}

impl ToCss for ComputedFont {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if self.style != ComputedFontStyle::Normal {
            write!(dest, "{} ", CssValue(&self.style))?;
        }
        if self.variant != ComputedFontVariantCss2::Normal {
            write!(dest, "{} ", CssValue(&self.variant))?;
        }
        if self.weight != ComputedFontWeight(400.0) {
            write!(dest, "{} ", CssValue(&self.weight))?;
        }
        if self.stretch != ComputedFontStretchCss3::Normal {
            write!(dest, "{} ", CssValue(&self.stretch))?;
        }
        write!(dest, "{} ", CssValue(&self.size))?;
        if self.line_height != ComputedLineHeight::Normal {
            write!(dest, "/ {} ", CssValue(&self.line_height))?;
        }
        self.family.to_css(dest)?;
        Ok(())
    }
}
