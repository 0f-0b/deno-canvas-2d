use std::convert::Infallible;

use cssparser::{ParseError, Parser, Token};
use cssparser_color::NumberOrPercentage;

pub mod angle;
pub mod color;
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

fn parse_number<'i>(input: &mut Parser<'i, '_>) -> Result<f32, ParseError<'i, Infallible>> {
    input.expect_number().map_err(Into::into)
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
