use std::convert::Infallible;
use std::rc::Rc;

use cssparser::{ParseError, Parser, Token};
use cssparser_color::NumberOrPercentage;

pub mod angle;
pub mod color;
pub mod filter;
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

fn parse_non_negative_number_or_percentage<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<NumberOrPercentage, ParseError<'i, Infallible>> {
    let location = input.current_source_location();
    Ok(match *input.next()? {
        Token::Number { value, .. } if value >= 0.0 => NumberOrPercentage::Number { value },
        Token::Percentage { unit_value, .. } if unit_value >= 0.0 => {
            NumberOrPercentage::Percentage { unit_value }
        }
        ref t => return Err(location.new_unexpected_token_error(t.clone())),
    })
}
