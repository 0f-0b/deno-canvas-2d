use std::convert::Infallible;

use cssparser::{match_ignore_ascii_case, ParseError, Parser, Token};

#[derive(Clone, Copy, Debug)]
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

    pub fn compute(self) -> ComputedAngle {
        let deg = match self {
            Self::Deg(v) => v as f64,
            Self::Grad(v) => v as f64 * 0.9,
            Self::Rad(v) => (v as f64).to_degrees(),
            Self::Turn(v) => v as f64 * 360.0,
        };
        ComputedAngle { deg }
    }

    pub fn parse_allow_zero<'i>(
        input: &mut Parser<'i, '_>,
    ) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value, .. } if value == 0.0 => Self::zero(),
            Token::Dimension {
                value, ref unit, ..
            } => match_ignore_ascii_case! { unit,
                "deg" => Self::Deg(value),
                "grad" => Self::Grad(value),
                "rad" => Self::Rad(value),
                "turn" => Self::Turn(value),
                _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone()))),
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedAngle {
    pub deg: f64,
}

impl ComputedAngle {
    pub fn zero() -> Self {
        Self { deg: 0.0 }
    }

    pub fn radians(self) -> f64 {
        self.deg.to_radians()
    }

    pub fn to_euclid(self) -> euclid::Angle<f64> {
        euclid::Angle::degrees(self.deg)
    }
}
