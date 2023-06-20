use std::collections::BTreeMap;

use crate::ParseError;

pub(super) fn parse_paragraphs(_input: &str) -> Result<ResultMap, ParseError> {
    todo!();
}

#[derive(Debug)]
pub(super) struct ResultMap {
    value: BTreeMap<Key, Node>, // todo
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    // todo
}

#[derive(Debug)]
enum Node {
    ParagraphList { children: Vec<Key> },
    Paragraph { children: Vec<Key> },
    ParagraphFailed {}, // todo
    RawString { content: String },
    InlineCommand { content: String },
    InlineMath { content: String },
    DisplayMath { content: String },
}
