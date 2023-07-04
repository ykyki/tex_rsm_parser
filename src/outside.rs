use crate::outside::schema::{convert_key, convert_to_entry, ParseResult};
use crate::parser::{parse_paragraphs, ParseOk};

pub fn parse_paragraphs_to_json(input: &str) -> ParseResult {
    let result = parse_paragraphs(input);

    match result {
        Err(e) => ParseResult::new_error(e.to_string()),
        Ok(ParseOk { rmap, char_count }) => {
            let hash_table = rmap.hash_table();
            let root = convert_key(rmap.root(), &hash_table);
            let entries = rmap
                .into_iter()
                .map(|(key, node)| convert_to_entry(key, node, &hash_table))
                .collect::<Vec<_>>();

            ParseResult::new_ok(root, entries, char_count)
        }
    }
}

pub(crate) const MAX_INPUT_LENGTH: usize = 100_000;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    // #[error("Too long input. The input must be less than {MAX_INPUT_LENGTH} characters.")]
    #[error("Too long input.")]
    TooLongInput,
}

mod schema;
