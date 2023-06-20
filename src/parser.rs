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
    value: u32,
}

impl Key {
    fn new() -> Self {
        Self { value: 0 }
    }
}

#[derive(Debug)]
enum Node {
    ParagraphList(Option<Vec<Key>>),
    Paragraph(Option<Vec<Key>>),
    RawString(String),
    InlineCommand(Option<String>),
    InlineMath(Option<MathExpression>),
    DisplayMath(Option<MathExpression>),
}

impl Node {
    pub fn is_ok(&self) -> bool {
        use self::Node::*;
        matches!(
            self,
            ParagraphList(Some(_))
                | Paragraph(Some(_))
                | RawString(_)
                | InlineCommand(Some(_))
                | InlineMath(Some(_))
                | DisplayMath(Some(_))
        )
    }
}

#[derive(Debug)]
struct MathExpression {
    content: String,
}
