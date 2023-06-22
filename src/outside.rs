use crate::parser::parse_paragraphs;

pub fn parse_paragraphs_to_json(input: &str) -> Result<serde_json::Value, ParseError> {
    let _ = parse_paragraphs(input)?;
    // todo ここでパース結果をJSONに変換する
    todo!();
}

pub(crate) const MAX_INPUT_LENGTH: usize = 100_000;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Too long input. The input must be less than {MAX_INPUT_LENGTH} characters.")]
    TooLongInput,
}
