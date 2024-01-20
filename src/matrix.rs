use std::convert::Infallible;
use std::fmt::{self, Debug};

use cssparser::{
    match_ignore_ascii_case, BasicParseError, CowRcStr, ParseError, Parser, ParserInput, Token,
};
use cssparser_color::NumberOrPercentage;
use deno_core::error::custom_error;
use deno_core::{anyhow, op2};
use euclid::default::{Transform2D, Transform3D, Vector3D};
use euclid::Angle;

#[derive(Clone, Copy)]
enum Matrix<T> {
    _2D(Transform2D<T>),
    _3D(Transform3D<T>),
}

impl<T> Debug for Matrix<T>
where
    Transform2D<T>: Debug,
    Transform3D<T>: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::_2D(m) => f.debug_tuple("_2D").field(m).finish(),
            Self::_3D(m) => f.debug_tuple("_3D").field(m).finish(),
        }
    }
}

impl<T> From<Transform2D<T>> for Matrix<T> {
    fn from(value: Transform2D<T>) -> Self {
        Self::_2D(value)
    }
}

impl<T> From<Transform3D<T>> for Matrix<T> {
    fn from(value: Transform3D<T>) -> Self {
        Self::_3D(value)
    }
}

#[derive(Clone, Copy, Debug)]
struct AbsoluteLength {
    px: f64,
}

impl AbsoluteLength {
    pub fn zero() -> Self {
        Self { px: 0.0 }
    }

    pub fn pixels(px: f64) -> Self {
        Self { px }
    }
}

#[derive(Clone, Copy, Debug)]
enum ComputedTransformFunction {
    Matrix([f64; 6]),
    Matrix3D([f64; 16]),
    Translate(AbsoluteLength, AbsoluteLength),
    Translate3D(AbsoluteLength, AbsoluteLength, AbsoluteLength),
    TranslateX(AbsoluteLength),
    TranslateY(AbsoluteLength),
    TranslateZ(AbsoluteLength),
    Scale(f64, f64),
    Scale3D(f64, f64, f64),
    ScaleX(f64),
    ScaleY(f64),
    ScaleZ(f64),
    Rotate(Angle<f64>),
    Rotate3D(f64, f64, f64, Angle<f64>),
    RotateX(Angle<f64>),
    RotateY(Angle<f64>),
    RotateZ(Angle<f64>),
    Skew(Angle<f64>, Angle<f64>),
    SkewX(Angle<f64>),
    SkewY(Angle<f64>),
    Perspective(Option<AbsoluteLength>),
}

impl ComputedTransformFunction {
    fn to_matrix(self) -> Matrix<f64> {
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
            Self::Rotate(t) => Transform2D::rotation(t).into(),
            Self::Rotate3D(x, y, z, t) => match Vector3D::new(x, y, z).try_normalize() {
                Some(n) => Transform3D::rotation(n.x, n.y, n.z, t),
                None => Transform3D::identity(),
            }
            .into(),
            Self::RotateX(t) => Transform3D::rotation(1.0, 0.0, 0.0, t).into(),
            Self::RotateY(t) => Transform3D::rotation(0.0, 1.0, 0.0, t).into(),
            Self::RotateZ(t) => Transform3D::rotation(0.0, 0.0, 1.0, t).into(),
            Self::Skew(a, b) => {
                Transform2D::new(1.0, b.radians.tan(), a.radians.tan(), 1.0, 0.0, 0.0).into()
            }
            Self::SkewX(a) => Transform2D::new(1.0, 0.0, a.radians.tan(), 1.0, 0.0, 0.0).into(),
            Self::SkewY(b) => Transform2D::new(1.0, b.radians.tan(), 0.0, 1.0, 0.0, 0.0).into(),
            Self::Perspective(d) => Transform3D {
                m34: d.map_or(0.0, |d| -d.px.recip()),
                ..Transform3D::identity()
            }
            .into(),
        }
    }
}

#[derive(Clone, Debug)]
struct ComputedTransform {
    transform_list: Vec<ComputedTransformFunction>,
}

impl ComputedTransform {
    fn new(transform_list: Vec<ComputedTransformFunction>) -> Self {
        Self { transform_list }
    }

    fn none() -> Self {
        Self::new(Vec::new())
    }

    fn to_matrix(&self) -> Matrix<f64> {
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

fn parse_one_or_more<'i, T, E>(
    input: &mut Parser<'i, '_>,
    mut parse_one: impl FnMut(&mut Parser<'i, '_>) -> Result<T, ParseError<'i, E>>,
) -> Result<Vec<T>, ParseError<'i, E>> {
    let mut results = vec![parse_one(input)?];
    while let Ok(item) = {
        input.skip_whitespace();
        input.try_parse(&mut parse_one)
    } {
        results.push(item);
    }
    Ok(results)
}

fn parse_number<'i>(input: &mut Parser<'i, '_>) -> Result<f64, ParseError<'i, Infallible>> {
    Ok(input.expect_number()? as f64)
}

fn parse_number_or_percentage<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<NumberOrPercentage, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } => NumberOrPercentage::Number { value },
        Token::Percentage { unit_value, .. } => NumberOrPercentage::Percentage { unit_value },
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

fn parse_absolute_length<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<AbsoluteLength, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if value == 0.0 => AbsoluteLength::zero(),
        Token::Dimension {
            value, ref unit, ..
        } => {
            let v = value as f64;
            AbsoluteLength::pixels(match_ignore_ascii_case! { unit,
                "cm" => v * (4800.0 / 127.0),
                "mm" => v * (480.0 / 127.0),
                "q" => v * (120.0 / 127.0),
                "in" => v * 96.0,
                "pc" => v * 16.0,
                "pt" => v * (4.0 / 3.0),
                "px" => v,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone()))),
            })
        }
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

fn parse_angle_or_zero<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<Angle<f64>, ParseError<'i, Infallible>> {
    use std::f64::consts::{PI, TAU};

    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if value == 0.0 => Angle::zero(),
        Token::Dimension {
            value, ref unit, ..
        } => {
            let v = value as f64;
            Angle::radians(match_ignore_ascii_case! { unit,
                "deg" => v.to_radians(),
                "grad" => v * (PI / 200.0),
                "rad" => v,
                "turn" => v * TAU,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone()))),
            })
        }
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

fn parse_transform_function<'i>(
    name: CowRcStr<'i>,
    input: &mut Parser<'i, '_>,
) -> Result<ComputedTransformFunction, ParseError<'i, Infallible>> {
    Ok(match_ignore_ascii_case! { &name,
        "matrix" => {
            let a = parse_number(input)?;
            input.expect_comma()?;
            let b = parse_number(input)?;
            input.expect_comma()?;
            let c = parse_number(input)?;
            input.expect_comma()?;
            let d = parse_number(input)?;
            input.expect_comma()?;
            let e = parse_number(input)?;
            input.expect_comma()?;
            let f = parse_number(input)?;
            ComputedTransformFunction::Matrix([a, b, c, d, e, f])
        },
        "matrix3d" => {
            let m11 = parse_number(input)?;
            input.expect_comma()?;
            let m12 = parse_number(input)?;
            input.expect_comma()?;
            let m13 = parse_number(input)?;
            input.expect_comma()?;
            let m14 = parse_number(input)?;
            input.expect_comma()?;
            let m21 = parse_number(input)?;
            input.expect_comma()?;
            let m22 = parse_number(input)?;
            input.expect_comma()?;
            let m23 = parse_number(input)?;
            input.expect_comma()?;
            let m24 = parse_number(input)?;
            input.expect_comma()?;
            let m31 = parse_number(input)?;
            input.expect_comma()?;
            let m32 = parse_number(input)?;
            input.expect_comma()?;
            let m33 = parse_number(input)?;
            input.expect_comma()?;
            let m34 = parse_number(input)?;
            input.expect_comma()?;
            let m41 = parse_number(input)?;
            input.expect_comma()?;
            let m42 = parse_number(input)?;
            input.expect_comma()?;
            let m43 = parse_number(input)?;
            input.expect_comma()?;
            let m44 = parse_number(input)?;
            ComputedTransformFunction::Matrix3D([
                m11, m12, m13, m14,
                m21, m22, m23, m24,
                m31, m32, m33, m34,
                m41, m42, m43, m44,
            ])
        },
        "translate" => {
            let x = parse_absolute_length(input)?;
            if input.try_parse(Parser::expect_comma).is_ok() {
                let y = parse_absolute_length(input)?;
                ComputedTransformFunction::Translate(x, y)
            } else {
                ComputedTransformFunction::Translate(x, AbsoluteLength::zero())
            }
        },
        "translate3d" => {
            let x = parse_absolute_length(input)?;
            input.expect_comma()?;
            let y = parse_absolute_length(input)?;
            input.expect_comma()?;
            let z = parse_absolute_length(input)?;
            ComputedTransformFunction::Translate3D(x, y, z)
        },
        "translatex" => {
            let x = parse_absolute_length(input)?;
            ComputedTransformFunction::TranslateX(x)
        },
        "translatey" => {
            let y = parse_absolute_length(input)?;
            ComputedTransformFunction::TranslateY(y)
        },
        "translatez" => {
            let z = parse_absolute_length(input)?;
            ComputedTransformFunction::TranslateZ(z)
        },
        "scale" => {
            let x = parse_number_or_percentage(input)?.unit_value();
            if input.try_parse(Parser::expect_comma).is_ok() {
                let y = parse_number_or_percentage(input)?.unit_value();
                ComputedTransformFunction::Scale(x as f64, y as f64)
            } else {
                ComputedTransformFunction::Scale(x as f64, x as f64)
            }
        },
        "scale3d" => {
            let x = parse_number_or_percentage(input)?.unit_value();
            input.expect_comma()?;
            let y = parse_number_or_percentage(input)?.unit_value();
            input.expect_comma()?;
            let z = parse_number_or_percentage(input)?.unit_value();
            ComputedTransformFunction::Scale3D(x as f64, y as f64, z as f64)
        },
        "scalex" => {
            let x = parse_number_or_percentage(input)?.unit_value();
            ComputedTransformFunction::ScaleX(x as f64)
        },
        "scaley" => {
            let y = parse_number_or_percentage(input)?.unit_value();
            ComputedTransformFunction::ScaleY(y as f64)
        },
        "scalez" => {
            let z = parse_number_or_percentage(input)?.unit_value();
            ComputedTransformFunction::ScaleZ(z as f64)
        },
        "rotate" => {
            let t = parse_angle_or_zero(input)?;
            ComputedTransformFunction::Rotate(t)
        },
        "rotate3d" => {
            let x = parse_number(input)?;
            input.expect_comma()?;
            let y = parse_number(input)?;
            input.expect_comma()?;
            let z = parse_number(input)?;
            input.expect_comma()?;
            let t = parse_angle_or_zero(input)?;
            ComputedTransformFunction::Rotate3D(x, y, z, t)
        },
        "rotatex" => {
            let t = parse_angle_or_zero(input)?;
            ComputedTransformFunction::RotateX(t)
        },
        "rotatey" => {
            let t = parse_angle_or_zero(input)?;
            ComputedTransformFunction::RotateY(t)
        },
        "rotatez" => {
            let t = parse_angle_or_zero(input)?;
            ComputedTransformFunction::RotateZ(t)
        },
        "skew" => {
            let a = parse_angle_or_zero(input)?;
            if input.try_parse(Parser::expect_comma).is_ok() {
                let b = parse_angle_or_zero(input)?;
                ComputedTransformFunction::Skew(a, b)
            } else {
                ComputedTransformFunction::Skew(a, Angle::zero())
            }
        },
        "skewx" => {
            let a = parse_angle_or_zero(input)?;
            ComputedTransformFunction::SkewX(a)
        },
        "skewy" => {
            let b = parse_angle_or_zero(input)?;
            ComputedTransformFunction::SkewY(b)
        },
        "perspective" => {
            let d = match input.try_parse(parse_absolute_length) {
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

fn parse_and_compute_transform(css: &str) -> Result<ComputedTransform, BasicParseError> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    parser
        .parse_entirely(parse_transform)
        .map_err(ParseError::basic)
}

#[op2(fast)]
pub fn op_canvas_2d_parse_matrix(
    #[string] transform_list: &str,
    #[buffer] out: &mut [f64],
) -> anyhow::Result<bool> {
    let transform = if transform_list.is_empty() {
        ComputedTransform::none()
    } else {
        parse_and_compute_transform(transform_list).map_err(|err| {
            custom_error(
                "DOMExceptionSyntaxError",
                format!(
                    "Invalid CSS transform list '{transform_list}': {} at {}:{}",
                    err.kind,
                    err.location.line + 1,
                    err.location.column
                ),
            )
        })?
    };
    Ok(match transform.to_matrix() {
        Matrix::_2D(m) => {
            out[..6].copy_from_slice(&m.to_array());
            true
        }
        Matrix::_3D(m) => {
            out.copy_from_slice(&m.to_array());
            false
        }
    })
}
