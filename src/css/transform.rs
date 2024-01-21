use std::convert::Infallible;

use cssparser::{
    match_ignore_ascii_case, BasicParseError, CowRcStr, ParseError, Parser, ParserInput, Token,
};
use euclid::default::{Transform2D, Transform3D, Vector3D};

use super::super::matrix::Matrix;
use super::angle::{ComputedAngle, SpecifiedAngle};
use super::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::{parse_number, parse_number_or_percentage, parse_one_or_more};

#[derive(Clone, Copy, Debug)]
pub enum ComputedTransformFunction {
    Matrix([f64; 6]),
    Matrix3D([f64; 16]),
    Translate(ComputedLength, ComputedLength),
    Translate3D(ComputedLength, ComputedLength, ComputedLength),
    TranslateX(ComputedLength),
    TranslateY(ComputedLength),
    TranslateZ(ComputedLength),
    Scale(f64, f64),
    Scale3D(f64, f64, f64),
    ScaleX(f64),
    ScaleY(f64),
    ScaleZ(f64),
    Rotate(ComputedAngle),
    Rotate3D(f64, f64, f64, ComputedAngle),
    RotateX(ComputedAngle),
    RotateY(ComputedAngle),
    RotateZ(ComputedAngle),
    Skew(ComputedAngle, ComputedAngle),
    SkewX(ComputedAngle),
    SkewY(ComputedAngle),
    Perspective(Option<ComputedLength>),
}

impl ComputedTransformFunction {
    fn to_matrix(self) -> Matrix {
        match self {
            Self::Matrix(m) => Transform2D::from_array(m).into(),
            Self::Matrix3D(m) => Transform3D::from_array(m).into(),
            Self::Translate(x, y) => Transform2D::translation(x.px, y.px).into(),
            Self::Translate3D(x, y, z) => Transform3D::translation(x.px, y.px, z.px).into(),
            Self::TranslateX(x) => Transform2D::translation(x.px, 0.0).into(),
            Self::TranslateY(y) => Transform2D::translation(0.0, y.px).into(),
            Self::TranslateZ(z) => Transform3D::translation(0.0, 0.0, z.px).into(),
            Self::Scale(x, y) => Transform2D::scale(x, y).into(),
            Self::Scale3D(x, y, z) => Transform3D::scale(x, y, z).into(),
            Self::ScaleX(x) => Transform2D::scale(x, 0.0).into(),
            Self::ScaleY(y) => Transform2D::scale(0.0, y).into(),
            Self::ScaleZ(z) => Transform3D::scale(1.0, 1.0, z).into(),
            Self::Rotate(t) => Transform2D::rotation(t.to_euclid()).into(),
            Self::Rotate3D(x, y, z, t) => match Vector3D::new(x, y, z).try_normalize() {
                Some(n) => Transform3D::rotation(n.x, n.y, n.z, t.to_euclid()),
                None => Transform3D::identity(),
            }
            .into(),
            Self::RotateX(t) => Transform3D::rotation(1.0, 0.0, 0.0, t.to_euclid()).into(),
            Self::RotateY(t) => Transform3D::rotation(0.0, 1.0, 0.0, t.to_euclid()).into(),
            Self::RotateZ(t) => Transform3D::rotation(0.0, 0.0, 1.0, t.to_euclid()).into(),
            Self::Skew(a, b) => {
                Transform2D::new(1.0, b.radians().tan(), a.radians().tan(), 1.0, 0.0, 0.0).into()
            }
            Self::SkewX(a) => Transform2D::new(1.0, 0.0, a.radians().tan(), 1.0, 0.0, 0.0).into(),
            Self::SkewY(b) => Transform2D::new(1.0, b.radians().tan(), 0.0, 1.0, 0.0, 0.0).into(),
            Self::Perspective(d) => Transform3D {
                m34: d.map_or(0.0, |d| -d.px.recip()),
                ..Transform3D::identity()
            }
            .into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComputedTransform {
    pub transform_list: Vec<ComputedTransformFunction>,
}

impl ComputedTransform {
    pub fn new(transform_list: Vec<ComputedTransformFunction>) -> Self {
        Self { transform_list }
    }

    pub fn none() -> Self {
        Self::new(Vec::new())
    }

    pub fn to_matrix(&self) -> Matrix {
        self.transform_list.iter().map(|f| f.to_matrix()).fold(
            Matrix::_2D(Transform2D::identity()),
            |a, b| match (a, b) {
                (Matrix::_2D(a), Matrix::_2D(b)) => b.then(&a).into(),
                (Matrix::_2D(a), Matrix::_3D(b)) => b.then(&a.to_3d()).into(),
                (Matrix::_3D(a), Matrix::_2D(b)) => b.to_3d().then(&a).into(),
                (Matrix::_3D(a), Matrix::_3D(b)) => b.then(&a).into(),
            },
        )
    }
}

fn parse_transform_function<'i>(
    name: CowRcStr<'i>,
    input: &mut Parser<'i, '_>,
) -> Result<ComputedTransformFunction, ParseError<'i, Infallible>> {
    Ok(match_ignore_ascii_case! { &name,
        "matrix" => {
            let a = parse_number(input)? as f64;
            input.expect_comma()?;
            let b = parse_number(input)? as f64;
            input.expect_comma()?;
            let c = parse_number(input)? as f64;
            input.expect_comma()?;
            let d = parse_number(input)? as f64;
            input.expect_comma()?;
            let e = parse_number(input)? as f64;
            input.expect_comma()?;
            let f = parse_number(input)? as f64;
            ComputedTransformFunction::Matrix([a, b, c, d, e, f])
        },
        "matrix3d" => {
            let m11 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m12 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m13 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m14 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m21 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m22 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m23 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m24 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m31 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m32 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m33 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m34 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m41 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m42 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m43 = parse_number(input)? as f64;
            input.expect_comma()?;
            let m44 = parse_number(input)? as f64;
            ComputedTransformFunction::Matrix3D([
                m11, m12, m13, m14,
                m21, m22, m23, m24,
                m31, m32, m33, m34,
                m41, m42, m43, m44,
            ])
        },
        "translate" => {
            let x = SpecifiedAbsoluteLength::parse(input)?.compute();
            if input.try_parse(Parser::expect_comma).is_ok() {
                let y = SpecifiedAbsoluteLength::parse(input)?.compute();
                ComputedTransformFunction::Translate(x, y)
            } else {
                ComputedTransformFunction::Translate(x, ComputedLength::zero())
            }
        },
        "translate3d" => {
            let x = SpecifiedAbsoluteLength::parse(input)?.compute();
            input.expect_comma()?;
            let y = SpecifiedAbsoluteLength::parse(input)?.compute();
            input.expect_comma()?;
            let z = SpecifiedAbsoluteLength::parse(input)?.compute();
            ComputedTransformFunction::Translate3D(x, y, z)
        },
        "translatex" => {
            let x = SpecifiedAbsoluteLength::parse(input)?.compute();
            ComputedTransformFunction::TranslateX(x)
        },
        "translatey" => {
            let y = SpecifiedAbsoluteLength::parse(input)?.compute();
            ComputedTransformFunction::TranslateY(y)
        },
        "translatez" => {
            let z = SpecifiedAbsoluteLength::parse(input)?.compute();
            ComputedTransformFunction::TranslateZ(z)
        },
        "scale" => {
            let x = parse_number_or_percentage(input)?.unit_value() as f64;
            if input.try_parse(Parser::expect_comma).is_ok() {
                let y = parse_number_or_percentage(input)?.unit_value() as f64;
                ComputedTransformFunction::Scale(x, y)
            } else {
                ComputedTransformFunction::Scale(x, x)
            }
        },
        "scale3d" => {
            let x = parse_number_or_percentage(input)?.unit_value() as f64;
            input.expect_comma()?;
            let y = parse_number_or_percentage(input)?.unit_value() as f64;
            input.expect_comma()?;
            let z = parse_number_or_percentage(input)?.unit_value() as f64;
            ComputedTransformFunction::Scale3D(x, y, z)
        },
        "scalex" => {
            let x = parse_number_or_percentage(input)?.unit_value() as f64;
            ComputedTransformFunction::ScaleX(x)
        },
        "scaley" => {
            let y = parse_number_or_percentage(input)?.unit_value() as f64;
            ComputedTransformFunction::ScaleY(y)
        },
        "scalez" => {
            let z = parse_number_or_percentage(input)?.unit_value() as f64;
            ComputedTransformFunction::ScaleZ(z)
        },
        "rotate" => {
            let t = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::Rotate(t)
        },
        "rotate3d" => {
            let x = parse_number(input)? as f64;
            input.expect_comma()?;
            let y = parse_number(input)? as f64;
            input.expect_comma()?;
            let z = parse_number(input)? as f64;
            input.expect_comma()?;
            let t = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::Rotate3D(x, y, z, t)
        },
        "rotatex" => {
            let t = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::RotateX(t)
        },
        "rotatey" => {
            let t = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::RotateY(t)
        },
        "rotatez" => {
            let t = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::RotateZ(t)
        },
        "skew" => {
            let a = SpecifiedAngle::parse_allow_zero(input)?.compute();
            if input.try_parse(Parser::expect_comma).is_ok() {
                let b = SpecifiedAngle::parse_allow_zero(input)?.compute();
                ComputedTransformFunction::Skew(a, b)
            } else {
                ComputedTransformFunction::Skew(a, ComputedAngle::zero())
            }
        },
        "skewx" => {
            let a = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::SkewX(a)
        },
        "skewy" => {
            let b = SpecifiedAngle::parse_allow_zero(input)?.compute();
            ComputedTransformFunction::SkewY(b)
        },
        "perspective" => {
            let d = match input
                .try_parse(SpecifiedAbsoluteLength::parse)
                .map(SpecifiedAbsoluteLength::compute)
            {
                Ok(d) if d.px >= 0.0 => Some(d),
                _ => {
                    input.expect_ident_matching("none")?;
                    None
                }
            };
            ComputedTransformFunction::Perspective(d)
        },
        _ => return Err(input.new_unexpected_token_error(Token::Ident(name))),
    })
}

fn parse_transform<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<ComputedTransform, ParseError<'i, Infallible>> {
    input.skip_whitespace();
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(ComputedTransform::none());
    }
    parse_one_or_more(input, |input| {
        let function = input.expect_function()?.clone();
        input.parse_nested_block(|input| parse_transform_function(function, input))
    })
    .map(ComputedTransform::new)
}

pub fn parse_and_compute_transform(css: &str) -> Result<ComputedTransform, BasicParseError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    parser
        .parse_entirely(parse_transform)
        .map_err(ParseError::basic)
}
