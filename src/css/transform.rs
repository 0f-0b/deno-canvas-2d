use std::convert::Infallible;

use cssparser::{match_ignore_ascii_case, ParseError, Parser, Token};
use euclid::default::{Transform2D, Transform3D, Vector3D};

use super::super::matrix::Matrix;
use super::angle::{ComputedAngle, SpecifiedAngle};
use super::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::{parse_number, parse_number_or_percentage, parse_one_or_more, FromCss};

#[derive(Clone, Copy, Debug)]
pub enum ComputedTransformFunction {
    Matrix([f32; 6]),
    Matrix3D([f32; 16]),
    Translate(ComputedLength, ComputedLength),
    Translate3D(ComputedLength, ComputedLength, ComputedLength),
    TranslateX(ComputedLength),
    TranslateY(ComputedLength),
    TranslateZ(ComputedLength),
    Scale(f32, f32),
    Scale3D(f32, f32, f32),
    ScaleX(f32),
    ScaleY(f32),
    ScaleZ(f32),
    Rotate(ComputedAngle),
    Rotate3D(f32, f32, f32, ComputedAngle),
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
            Self::Matrix(m) => Transform2D::from_array(m).cast().into(),
            Self::Matrix3D(m) => Transform3D::from_array(m).cast().into(),
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

impl FromCss for ComputedTransformFunction {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let location = input.current_source_location();
        let name = input.expect_function()?.clone();
        input.parse_nested_block(|input| {
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
                    Self::Matrix([a, b, c, d, e, f])
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
                    Self::Matrix3D([
                        m11, m12, m13, m14,
                        m21, m22, m23, m24,
                        m31, m32, m33, m34,
                        m41, m42, m43, m44,
                    ])
                },
                "translate" => {
                    let x = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    if input.try_parse(Parser::expect_comma).is_ok() {
                        let y = SpecifiedAbsoluteLength::from_css(input)?.compute();
                        Self::Translate(x, y)
                    } else {
                        Self::Translate(x, ComputedLength::zero())
                    }
                },
                "translate3d" => {
                    let x = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    input.expect_comma()?;
                    let y = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    input.expect_comma()?;
                    let z = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    Self::Translate3D(x, y, z)
                },
                "translatex" => {
                    let x = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    Self::TranslateX(x)
                },
                "translatey" => {
                    let y = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    Self::TranslateY(y)
                },
                "translatez" => {
                    let z = SpecifiedAbsoluteLength::from_css(input)?.compute();
                    Self::TranslateZ(z)
                },
                "scale" => {
                    let x = parse_number_or_percentage(input)?.unit_value();
                    if input.try_parse(Parser::expect_comma).is_ok() {
                        let y = parse_number_or_percentage(input)?.unit_value();
                        Self::Scale(x, y)
                    } else {
                        Self::Scale(x, x)
                    }
                },
                "scale3d" => {
                    let x = parse_number_or_percentage(input)?.unit_value();
                    input.expect_comma()?;
                    let y = parse_number_or_percentage(input)?.unit_value();
                    input.expect_comma()?;
                    let z = parse_number_or_percentage(input)?.unit_value();
                    Self::Scale3D(x, y, z)
                },
                "scalex" => {
                    let x = parse_number_or_percentage(input)?.unit_value();
                    Self::ScaleX(x)
                },
                "scaley" => {
                    let y = parse_number_or_percentage(input)?.unit_value();
                    Self::ScaleY(y)
                },
                "scalez" => {
                    let z = parse_number_or_percentage(input)?.unit_value();
                    Self::ScaleZ(z)
                },
                "rotate" => {
                    let t = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::Rotate(t)
                },
                "rotate3d" => {
                    let x = parse_number(input)?;
                    input.expect_comma()?;
                    let y = parse_number(input)?;
                    input.expect_comma()?;
                    let z = parse_number(input)?;
                    input.expect_comma()?;
                    let t = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::Rotate3D(x, y, z, t)
                },
                "rotatex" => {
                    let t = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::RotateX(t)
                },
                "rotatey" => {
                    let t = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::RotateY(t)
                },
                "rotatez" => {
                    let t = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::RotateZ(t)
                },
                "skew" => {
                    let a = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    if input.try_parse(Parser::expect_comma).is_ok() {
                        let b = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                        Self::Skew(a, b)
                    } else {
                        Self::Skew(a, ComputedAngle::zero())
                    }
                },
                "skewx" => {
                    let a = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::SkewX(a)
                },
                "skewy" => {
                    let b = SpecifiedAngle::from_css_allow_zero(input)?.compute();
                    Self::SkewY(b)
                },
                "perspective" => {
                    let d = match input.try_parse(|input| {
                        SpecifiedAbsoluteLength::from_css_with_range(input, 0.0, f32::INFINITY)
                    }) {
                        Ok(d) => Some(d.compute()),
                        Err(_) => {
                            input.expect_ident_matching("none")?;
                            None
                        }
                    };
                    Self::Perspective(d)
                },
                _ => return Err(location.new_unexpected_token_error(Token::Ident(name))),
            })
        })
    }
}

#[derive(Clone, Debug)]
pub struct ComputedTransform {
    pub transform_list: Box<[ComputedTransformFunction]>,
}

impl ComputedTransform {
    pub fn none() -> Self {
        Self {
            transform_list: Box::new([]),
        }
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

impl FromCss for ComputedTransform {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        input.skip_whitespace();
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(Self::none());
        }
        let transform_list =
            parse_one_or_more(input, ComputedTransformFunction::from_css)?.into_boxed_slice();
        Ok(Self { transform_list })
    }
}
