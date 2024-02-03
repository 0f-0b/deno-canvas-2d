use std::convert::Infallible;
use std::error::Error;
use std::fmt::{self, Display};
use std::ops::RangeBounds;
use std::rc::Rc;

use cssparser::{
    match_ignore_ascii_case, serialize_string, BasicParseError, ParseError, Parser, ParserInput,
    SourceLocation, ToCss, Token, UnicodeRange,
};
use cssparser_color::NumberOrPercentage;
use itertools::Itertools as _;

pub mod angle;
pub mod color;
pub mod filter;
pub mod font;
pub mod length;
pub mod transform;

pub trait FromCss: Sized {
    type Err;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>>;

    fn from_css_string(css: &str) -> Result<Self, ParseError<Self::Err>> {
        let mut input = ParserInput::new(css);
        let mut parser = Parser::new(&mut input);
        parser.parse_entirely(Self::from_css)
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

fn parse_string<'i>(input: &mut Parser<'i, '_>) -> Result<Rc<str>, ParseError<'i, Infallible>> {
    Ok(input.expect_string()?.as_ref().into())
}

fn parse_url<'i>(input: &mut Parser<'i, '_>) -> Result<Rc<str>, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::UnquotedUrl(ref url) => url.as_ref().into(),
        Token::Function(ref name) => {
            let name = name.clone();
            input.parse_nested_block(|input| {
                match_ignore_ascii_case! { &name,
                    "url" | "src" => parse_string(input),
                    _ => Err(location.new_unexpected_token_error(Token::Ident(name))),
                }
            })?
        }
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

fn parse_number<'i>(input: &mut Parser<'i, '_>) -> Result<f32, ParseError<'i, Infallible>> {
    parse_number_with_range(input, ..)
}

fn parse_number_with_range<'i>(
    input: &mut Parser<'i, '_>,
    range: impl RangeBounds<f32>,
) -> Result<f32, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if range.contains(&value) => value,
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

fn parse_integer_with_range<'i>(
    input: &mut Parser<'i, '_>,
    range: impl RangeBounds<i32>,
) -> Result<i32, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number {
            int_value: Some(int_value),
            ..
        } if range.contains(&int_value) => int_value,
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

fn parse_number_or_percentage<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<NumberOrPercentage, ParseError<'i, Infallible>> {
    parse_number_or_percentage_with_range(input, ..)
}

fn parse_number_or_percentage_with_range<'i>(
    input: &mut Parser<'i, '_>,
    range: impl RangeBounds<f32>,
) -> Result<NumberOrPercentage, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if range.contains(&value) => {
            NumberOrPercentage::Number { value }
        }
        Token::Percentage { unit_value, .. } if range.contains(&unit_value) => {
            NumberOrPercentage::Percentage { unit_value }
        }
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}

impl FromCss for UnicodeRange {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        Ok(UnicodeRange::parse(input)?)
    }
}

#[derive(Clone, Debug)]
pub struct UnicodeRangeSet {
    bounds: Box<[u32]>,
}

impl UnicodeRangeSet {
    pub fn new(ranges: impl IntoIterator<Item = UnicodeRange>) -> Self {
        Self {
            bounds: ranges
                .into_iter()
                .map(|range| (range.start, range.end + 1))
                .sorted()
                .coalesce(|a, b| {
                    if b.0 <= a.1 {
                        Ok((a.0, a.1.max(b.1)))
                    } else {
                        Err((a, b))
                    }
                })
                .flat_map(|(start, end)| [start, end])
                .collect(),
        }
    }

    pub fn contains(&self, c: u32) -> bool {
        (self.bounds.partition_point(|&x| x <= c) & 1) != 0
    }
}

#[derive(Clone, Copy)]
struct CssValue<'a, T: ?Sized>(&'a T);

#[derive(Clone, Copy)]
struct CssString<'a>(&'a str);

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
display_css!(impl ['a] for CssString<'a>);
display_css!(impl [T: ?Sized] for CssNumber<T>);
display_css!(impl [T: ?Sized] for CssPercentage<T>);

impl<'a, T: ToCss + ?Sized> ToCss for CssValue<'a, T> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

impl<'a> ToCss for CssString<'a> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_string(self.0, dest)
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

macro_rules! impl_to_css_for_specified_dimension {
    ($t:ty { $($variant:ident => $unit:literal,)* _ => $canonical_unit:literal $(,)? }) => {
        impl cssparser::ToCss for $t {
            fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
                match self.unitless_value() {
                    v if v == f32::INFINITY => {
                        dest.write_str(concat!("calc(infinity * 1", $canonical_unit, ")"))
                    }
                    v if v == f32::NEG_INFINITY => {
                        dest.write_str(concat!("calc(-infinity * 1", $canonical_unit, ")"))
                    }
                    v if f32::is_nan(v) => {
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

use impl_to_css_for_specified_dimension;

macro_rules! impl_to_css_for_computed_dimension {
    ($t:ty { $field:ident => $unit:literal }) => {
        impl cssparser::ToCss for $t {
            fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
                match self.$field {
                    v if v == f32::INFINITY => {
                        dest.write_str(concat!("calc(infinity * 1", $unit, ")"))
                    }
                    v if v == f32::NEG_INFINITY => {
                        dest.write_str(concat!("calc(-infinity * 1", $unit, ")"))
                    }
                    v if f32::is_nan(v) => dest.write_str(concat!("calc(NaN * 1", $unit, ")")),
                    v => write!(dest, concat!("{}", $unit), v),
                }
            }
        }
    };
}

use impl_to_css_for_computed_dimension;

macro_rules! try_match_next_ident_ignore_ascii_case {
    ($input:expr, $($pattern:pat $(if $guard:expr)? => $then:expr),+ $(,)?) => {
        'block: {
            let location = $input.current_source_location();
            let ident = match $input.expect_ident() {
                Ok(ident) => ident,
                Err(err) => break 'block Err(err),
            };
            Ok(cssparser::match_ignore_ascii_case! { ident,
                $($pattern $(if $guard)? => $then,)+
                _ => {
                    break 'block Err(location
                        .new_basic_unexpected_token_error(cssparser::Token::Ident(ident.clone())))
                }
            })
        }
    }
}

use try_match_next_ident_ignore_ascii_case;

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    location: SourceLocation,
}

impl<'i> From<BasicParseError<'i>> for SyntaxError {
    fn from(value: BasicParseError<'i>) -> Self {
        Self {
            message: value.kind.to_string(),
            location: value.location,
        }
    }
}

impl<'i, E: Display> From<ParseError<'i, E>> for SyntaxError {
    fn from(value: ParseError<'i, E>) -> Self {
        Self {
            message: value.kind.to_string(),
            location: value.location,
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}",
            self.message,
            self.location.line + 1,
            self.location.column,
        )
    }
}

impl Error for SyntaxError {}
