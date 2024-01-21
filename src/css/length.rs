use std::convert::Infallible;

use cssparser::{match_ignore_ascii_case, ParseError, Parser, Token};

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

    pub fn parse<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Infallible>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value, .. } if value == 0.0 => Self::zero(),
            Token::Dimension {
                value, ref unit, ..
            } => match_ignore_ascii_case! { unit,
                "cm" => Self::Cm(value),
                "mm" => Self::Mm(value),
                "q" => Self::Q(value),
                "in" => Self::In(value),
                "pc" => Self::Pc(value),
                "pt" => Self::Pt(value),
                "px" => Self::Px(value),
                _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone()))),
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
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
