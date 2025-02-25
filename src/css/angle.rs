use std::convert::Infallible;
use std::ops::RangeBounds;

use cssparser::{match_ignore_ascii_case, ParseError, Parser, Token};

use super::{impl_to_css_for_computed_dimension, impl_to_css_for_specified_dimension};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpecifiedAngle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
}

impl SpecifiedAngle {
    pub fn zero() -> Self {
        Self::Deg(0.0)
    }

    pub fn unitless_value(self) -> f32 {
        match self {
            Self::Deg(v) | Self::Grad(v) | Self::Rad(v) | Self::Turn(v) => v,
        }
    }

    pub fn compute(self) -> ComputedAngle {
        let deg = match self {
            Self::Deg(v) => v,
            Self::Grad(v) => v * 0.9,
            Self::Rad(v) => v.to_degrees(),
            Self::Turn(v) => v * 360.0,
        };
        ComputedAngle { deg }
    }

    fn from_dimension(value: f32, unit: &str) -> Option<Self> {
        Some(match_ignore_ascii_case! { unit,
            "deg" => Self::Deg(value),
            "grad" => Self::Grad(value),
            "rad" => Self::Rad(value),
            "turn" => Self::Turn(value),
            _ => return None,
        })
    }

    pub fn from_css_with_range<'i>(
        input: &mut Parser<'i, '_>,
        deg_range: impl RangeBounds<f32>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            ref t @ Token::Dimension {
                value, ref unit, ..
            } => {
                let result = Self::from_dimension(value, unit).ok_or_else(|| {
                    location.new_unexpected_token_error(Token::Ident(unit.clone()))
                })?;
                let ComputedAngle { deg } = result.compute();
                if !deg_range.contains(&deg) {
                    return Err(location.new_unexpected_token_error(t.clone()));
                }
                result
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }

    pub fn from_css_allow_zero<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value: 0.0, .. } => Self::zero(),
            Token::Dimension {
                value, ref unit, ..
            } => Self::from_dimension(value, unit)
                .ok_or_else(|| location.new_unexpected_token_error(Token::Ident(unit.clone())))?,
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

impl_to_css_for_specified_dimension!(SpecifiedAngle {
    Deg => "deg",
    Grad => "grad",
    Rad => "rad",
    Turn => "turn",
    _ => "deg",
});

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedAngle {
    pub deg: f32,
}

impl ComputedAngle {
    pub fn zero() -> Self {
        Self { deg: 0.0 }
    }

    pub fn radians(self) -> f32 {
        self.deg.to_radians()
    }

    pub fn to_euclid(self) -> euclid::Angle<f32> {
        euclid::Angle::degrees(self.deg)
    }
}

impl_to_css_for_computed_dimension!(ComputedAngle { deg => "deg" });
