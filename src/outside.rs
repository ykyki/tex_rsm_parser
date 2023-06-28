use crate::outside::schema::{
    convert_key, convert_to_column, ParseResult, ParseResultError, ParseResultOk,
};
use crate::parser::parse_paragraphs;

pub fn parse_paragraphs_to_json(input: &str) -> ParseResult {
    let result = parse_paragraphs(input);

    match result {
        Err(e) => ParseResult::Error(ParseResultError {
            message: e.to_string(),
        }),
        Ok(map) => {
            let root = convert_key(map.root());
            let columns = map
                .into_iter()
                .map(|(key, node)| convert_to_column(key, node))
                .collect::<Vec<_>>();

            ParseResult::Ok(ParseResultOk { root, columns })
        }
    }
}

pub(crate) const MAX_INPUT_LENGTH: usize = 1_000;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Too long input. The input must be less than {MAX_INPUT_LENGTH} characters.")]
    TooLongInput,
}

mod schema;
