pub fn parse_paragraphs(_input: &str) -> Result<ParseResult, ParseError> {
    todo!();
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Too long input. Input must be less than 100_000 characters.")]
    TooLongInput,
}

#[derive(Debug)]
pub struct ParseResult {
    // todo
}
