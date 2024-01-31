use std::convert::Infallible;
use std::ops::RangeBounds;

use cssparser::{match_ignore_ascii_case, ParseError, Parser, Token};

use super::{impl_to_css_for_computed_dimension, impl_to_css_for_specified_dimension, FromCss};

#[derive(Clone, Copy, Debug, PartialEq)]
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
            Self::Cm(v) => v * (4800.0 / 127.0),
            Self::Mm(v) => v * (480.0 / 127.0),
            Self::Q(v) => v * (120.0 / 127.0),
            Self::In(v) => v * 96.0,
            Self::Pc(v) => v * 16.0,
            Self::Pt(v) => v * (4.0 / 3.0),
            Self::Px(v) => v,
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

    pub fn from_css_with_range<'i>(
        input: &mut Parser<'i, '_>,
        px_range: impl RangeBounds<f32>,
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
                if !px_range.contains(&px) {
                    return Err(location.new_unexpected_token_error(t.clone()));
                }
                result
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl FromCss for SpecifiedAbsoluteLength {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
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
}

impl_to_css_for_specified_dimension!(SpecifiedAbsoluteLength {
    Cm => "cm",
    Mm => "mm",
    Q => "q",
    In => "in",
    Pc => "pc",
    Pt => "pt",
    Px => "px",
    _ => "px",
});

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedLength {
    pub px: f32,
}

impl ComputedLength {
    pub fn zero() -> Self {
        Self { px: 0.0 }
    }
}

impl_to_css_for_computed_dimension!(ComputedLength { px => "px" });
