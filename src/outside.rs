use crate::parser::parse_paragraphs;

pub fn parse_paragraphs_to_json(input: &str) -> Result<serde_json::Value, ParseError> {
    let _ = parse_paragraphs(input)?;
    // todo ここでパース結果をJSONに変換する
    todo!();
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Too long input. Input must be less than 100_000 characters.")]
    TooLongInput,
}
