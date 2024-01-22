use std::convert::Infallible;

use cssparser::{match_ignore_ascii_case, BasicParseError, ParseError, Parser, ParserInput, Token};

use super::impl_to_css_for_dimension;

#[derive(Clone, Copy, Debug)]
pub enum SpecifiedAbsoluteLength {
    Cm(f32),
    Mm(f32),
    Q(f32),
    In(f32),
    Pc(f32),
    Pt(f32),
    Px(f32),
}

impl SpecifiedAbsoluteLength {
    pub fn zero() -> Self {
        Self::Px(0.0)
    }

    pub fn unitless_value(self) -> f32 {
        match self {
            Self::Cm(v)
            | Self::Mm(v)
            | Self::Q(v)
            | Self::In(v)
            | Self::Pc(v)
            | Self::Pt(v)
            | Self::Px(v) => v,
        }
    }

    pub fn compute(self) -> ComputedLength {
        let px = match self {
            Self::Cm(v) => v as f64 * (4800.0 / 127.0),
            Self::Mm(v) => v as f64 * (480.0 / 127.0),
            Self::Q(v) => v as f64 * (120.0 / 127.0),
            Self::In(v) => v as f64 * 96.0,
            Self::Pc(v) => v as f64 * 16.0,
            Self::Pt(v) => v as f64 * (4.0 / 3.0),
            Self::Px(v) => v as f64,
        };
        ComputedLength { px }
    }

    fn from_dimension(value: f32, unit: &str) -> Option<Self> {
        Some(match_ignore_ascii_case! { unit,
            "cm" => Self::Cm(value),
            "mm" => Self::Mm(value),
            "q" => Self::Q(value),
            "in" => Self::In(value),
            "pc" => Self::Pc(value),
            "pt" => Self::Pt(value),
            "px" => Self::Px(value),
            _ => return None,
        })
    }

    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value, .. } if value == 0.0 => Self::zero(),
            Token::Dimension {
                value, ref unit, ..
            } => Self::from_dimension(value, unit)
                .ok_or_else(|| location.new_unexpected_token_error(Token::Ident(unit.clone())))?,
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }

    pub fn parse_with_range<'i>(
        input: &mut Parser<'i, '_>,
        min_px: f64,
        max_px: f64,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            ref t @ Token::Dimension {
                value, ref unit, ..
            } => {
                let result = Self::from_dimension(value, unit).ok_or_else(|| {
                    location.new_unexpected_token_error(Token::Ident(unit.clone()))
                })?;
                let ComputedLength { px } = result.compute();
                if !(min_px..=max_px).contains(&px) {
                    return Err(location.new_unexpected_token_error(t.clone()));
                }
                result
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl_to_css_for_dimension! {
    SpecifiedAbsoluteLength {
        Canonical => "px";
        Cm => "cm",
        Mm => "mm",
        Q => "q",
        In => "in",
        Pc => "pc",
        Pt => "pt",
        Px => "px",
    }
}

pub fn parse_absolute_length(css: &str) -> Result<SpecifiedAbsoluteLength, BasicParseError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    parser
        .parse_entirely(SpecifiedAbsoluteLength::parse)
        .map_err(ParseError::basic)
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedLength {
    pub px: f64,
}

impl ComputedLength {
    pub fn zero() -> Self {
        Self { px: 0.0 }
    }
}
