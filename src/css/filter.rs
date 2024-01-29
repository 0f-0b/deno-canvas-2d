use std::convert::Infallible;
use std::rc::Rc;

use cssparser::{match_ignore_ascii_case, ParseError, Parser, Token};

use super::angle::{ComputedAngle, SpecifiedAngle};
use super::color::ComputedColor;
use super::length::{ComputedLength, SpecifiedAbsoluteLength};
use super::{parse_number_or_percentage_with_range, parse_one_or_more, parse_string, FromCss};

#[derive(Clone, Copy, Debug)]
pub struct Shadow {
    pub color: ComputedColor,
    pub offset_x: ComputedLength,
    pub offset_y: ComputedLength,
    pub blur: ComputedLength,
}

#[derive(Clone, Copy, Debug)]
pub enum ComputedFilterFunction {
    Blur(ComputedLength),
    Brightness(f32),
    Contrast(f32),
    DropShadow(Shadow),
    Grayscale(f32),
    HueRotate(ComputedAngle),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    Sepia(f32),
}

#[derive(Clone, Debug)]
pub enum ComputedFilterValue {
    Url(Rc<str>),
    FilterFunction(ComputedFilterFunction),
}

impl FromCss for ComputedFilterValue {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::UnquotedUrl(ref url) => Self::Url(url.as_ref().into()),
            Token::Function(ref name) => {
                let name = name.clone();
                input.parse_nested_block(|input| {
                    Ok(match_ignore_ascii_case! { &name,
                        "blur" => {
                            let v = match input.try_parse(|input| {
                                SpecifiedAbsoluteLength::from_css_with_range(
                                    input,
                                    0.0,
                                    f32::INFINITY,
                                )
                            }) {
                                Ok(v) => v.compute(),
                                Err(_) => ComputedLength::zero(),
                            };
                            Self::FilterFunction(ComputedFilterFunction::Blur(v))
                        },
                        "brightness" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Brightness(v))
                        },
                        "contrast" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Contrast(v))
                        },
                        "drop-shadow" => {
                            let color = input.try_parse(ComputedColor::from_css).ok();
                            let offset_x = SpecifiedAbsoluteLength::from_css(input)?.compute();
                            let offset_y = SpecifiedAbsoluteLength::from_css(input)?.compute();
                            let blur = match input.try_parse(|input| {
                                SpecifiedAbsoluteLength::from_css_with_range(
                                    input,
                                    0.0,
                                    f32::INFINITY,
                                )
                            }) {
                                Ok(v) => v.compute(),
                                Err(_) => ComputedLength::zero(),
                            };
                            let color = color
                                .or_else(|| input.try_parse(ComputedColor::from_css).ok())
                                .unwrap_or(ComputedColor::CurrentColor);
                            Self::FilterFunction(ComputedFilterFunction::DropShadow(Shadow {
                                color,
                                offset_x,
                                offset_y,
                                blur,
                            }))
                        },
                        "grayscale" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Grayscale(v))
                        },
                        "hue-rotate" => {
                            let v = match input.try_parse(SpecifiedAngle::from_css_allow_zero) {
                                Ok(v) => v.compute(),
                                Err(_) => ComputedAngle::zero(),
                            };
                            Self::FilterFunction(ComputedFilterFunction::HueRotate(v))
                        },
                        "invert" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Invert(v))
                        },
                        "opacity" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Opacity(v))
                        },
                        "sepia" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Sepia(v))
                        },
                        "saturate" => {
                            let v = match input.try_parse(|input| {
                                parse_number_or_percentage_with_range(input, 0.0, f32::INFINITY)
                            }) {
                                Ok(v) => v.unit_value(),
                                Err(_) => 1.0,
                            };
                            Self::FilterFunction(ComputedFilterFunction::Saturate(v))
                        },
                        "url" | "src" => Self::Url(parse_string(input)?),
                        _ => return Err(location.new_unexpected_token_error(Token::Ident(name))),
                    })
                })?
            }
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ComputedFilter {
    pub filter_value_list: Box<[ComputedFilterValue]>,
}

impl ComputedFilter {
    pub fn none() -> Self {
        Self {
            filter_value_list: Box::new([]),
        }
    }
}

impl FromCss for ComputedFilter {
    type Err = Infallible;

    fn from_css<'i>(input: &mut Parser<'i, '_>) -> Result<Self, ParseError<'i, Self::Err>> {
        input.skip_whitespace();
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(Self::none());
        }
        let filter_value_list =
            parse_one_or_more(input, ComputedFilterValue::from_css)?.into_boxed_slice();
        Ok(Self { filter_value_list })
    }
}
