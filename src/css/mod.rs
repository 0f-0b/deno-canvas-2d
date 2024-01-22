use std::convert::Infallible;
use std::fmt::{self, Display};
use std::rc::Rc;

use cssparser::{ParseError, Parser, ToCss, Token};
use cssparser_color::NumberOrPercentage;

pub mod angle;
pub mod color;
pub mod filter;
pub mod font;
pub mod length;
pub mod transform;

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

fn parse_string<'i>(input: &mut Parser<'i, '_>) -> Result<Rc<str>, ParseError<'i, Infallible>> {
    Ok(input.expect_string()?.as_ref().into())
}

fn parse_number<'i>(input: &mut Parser<'i, '_>) -> Result<f32, ParseError<'i, Infallible>> {
    Ok(input.expect_number()?)
}

fn parse_number_with_range<'i>(
    input: &mut Parser<'i, '_>,
    min: f32,
    max: f32,
) -> Result<f32, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if (min..=max).contains(&value) => value,
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
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

fn parse_number_or_percentage_with_range<'i>(
    input: &mut Parser<'i, '_>,
    min: f32,
    max: f32,
) -> Result<NumberOrPercentage, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if (min..=max).contains(&value) => {
            NumberOrPercentage::Number { value }
        }
        Token::Percentage { unit_value, .. } if (min..=max).contains(&unit_value) => {
            NumberOrPercentage::Percentage { unit_value }
        }
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

#[derive(Clone, Copy)]
struct CssValue<'a, T: ?Sized>(&'a T);

#[derive(Clone, Copy)]
struct CssNumber<T: ?Sized>(T);

#[derive(Clone, Copy)]
struct CssPercentage<T: ?Sized>(T);

macro_rules! display_css {
    (impl $([$($gen:tt)*])? for $($t:ty)?) => {
        impl $(<$($gen)*>)? Display for $($t)? where Self: ToCss {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.to_css(f)
            }
        }
    };
}

display_css!(impl ['a, T: ?Sized] for CssValue<'a, T>);
display_css!(impl [T: ?Sized] for CssNumber<T>);
display_css!(impl [T: ?Sized] for CssPercentage<T>);

impl<'a, T: ToCss + ?Sized> ToCss for CssValue<'a, T> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

macro_rules! impl_to_css_for_number {
    ($t:ty) => {
        impl cssparser::ToCss for CssNumber<$t> {
            fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
                match self.0 {
                    v if v == <$t>::INFINITY => dest.write_str("calc(infinity)"),
                    v if v == <$t>::NEG_INFINITY => dest.write_str("calc(-infinity)"),
                    v if v.is_nan() => dest.write_str("calc(NaN)"),
                    v => write!(dest, "{v}"),
                }
            }
        }
    };
}

impl_to_css_for_number!(f32);
impl_to_css_for_number!(f64);

macro_rules! impl_to_css_for_percentage {
    ($t:ty) => {
        impl cssparser::ToCss for CssPercentage<$t> {
            fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
                match self.0 * 100.0 {
                    v if v == <$t>::INFINITY => dest.write_str("calc(infinity * 1%)"),
                    v if v == <$t>::NEG_INFINITY => dest.write_str("calc(-infinity * 1%)"),
                    v if v.is_nan() => dest.write_str("calc(NaN * 1%)"),
                    v => write!(dest, "{v}%"),
                }
            }
        }
    };
}

impl_to_css_for_percentage!(f32);
impl_to_css_for_percentage!(f64);

macro_rules! impl_to_css_for_dimension {
    ($t:ty { Canonical => $canonical_unit:literal; $($variant:ident => $unit:literal),* $(,)? }) => {
        impl cssparser::ToCss for $t {
            fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
                match self.unitless_value() {
                    v if v == f32::INFINITY => {
                        dest.write_str(concat!("calc(infinity * 1", $canonical_unit, ")"))
                    }
                    v if v == f32::NEG_INFINITY => {
                        dest.write_str(concat!("calc(-infinity * 1", $canonical_unit, ")"))
                    }
                    v if v.is_nan() => {
                        dest.write_str(concat!("calc(NaN * 1", $canonical_unit, ")"))
                    }
                    _ => match *self {
                        $(Self::$variant(v) => write!(dest, concat!("{}", $unit), v),)*
                    },
                }
            }
        }
    };
}

use impl_to_css_for_dimension;
