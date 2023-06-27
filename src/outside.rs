use crate::key::Key;
use crate::node::Node;
use crate::parser::parse_paragraphs;
use serde::Serialize;

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

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum ParseResult {
    #[serde(rename = "ok")]
    Ok(ParseResultOk),
    #[serde(rename = "error")]
    Error(ParseResultError),
}

#[derive(Debug, Serialize)]
pub struct ParseResultError {
    pub message: String,
}

pub(crate) const MAX_INPUT_LENGTH: usize = 1_000;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum ParseError {
    #[error("Too long input. The input must be less than {MAX_INPUT_LENGTH} characters.")]
    TooLongInput,
}

#[derive(Debug, Serialize)]
pub struct ParseResultOk {
    root: ParseResultKey,
    columns: Vec<ParseResultColumn>,
}

#[derive(Debug, Serialize)]
pub struct ParseResultKey(String);

#[derive(Debug, Serialize)]
pub struct ParseResultColumn {
    key: ParseResultKey,
    kind: ParseResultColumnKind,
    detail: ParseResultColumnDetail,
}

#[derive(Debug, Serialize)]
pub enum ParseResultColumnDetail {
    Paragraphs(Vec<ParseResultKey>),
    Paragraph(Vec<ParseResultKey>),
    Text(String),
    InlineCommand(String),
    InlineMath(ParseResultMath),
    DisplayMath(ParseResultMath),
}

#[derive(Debug, Serialize)]
pub struct ParseResultMath {
    status: ParseResultMathStatus,
    content: String,
}

#[derive(Debug, Serialize)]
pub enum ParseResultMathStatus {
    Ok,
    Ng,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ParseResultColumnKind {
    Paragraphs,
    Paragraph,
    Text,
    InlineCommand,
    InlineMath,
    DisplayMath,
}

fn convert_to_column(key: Key, node: Node) -> ParseResultColumn {
    let kind = match &node {
        Node::ParagraphList(_) => ParseResultColumnKind::Paragraphs,
        Node::Paragraph(_) => ParseResultColumnKind::Paragraph,
        Node::RawString(_) => ParseResultColumnKind::Text,
        Node::InlineCommand(_) => ParseResultColumnKind::InlineCommand,
        Node::MathExpr(v) => {
            if v.is_inline() {
                ParseResultColumnKind::InlineMath
            } else if v.is_display() {
                ParseResultColumnKind::DisplayMath
            } else {
                unreachable!()
            }
        }
    };

    let detail = match node {
        Node::ParagraphList(Some(ks)) => ParseResultColumnDetail::Paragraphs(convert_keys(ks)),
        Node::Paragraph(Some(ks)) => ParseResultColumnDetail::Paragraph(convert_keys(ks)),
        Node::RawString(s) => ParseResultColumnDetail::Text(s),
        Node::InlineCommand(Some(s)) => ParseResultColumnDetail::InlineCommand(s),
        Node::MathExpr(v) => ParseResultColumnDetail::InlineMath(ParseResultMath {
            status: if v.is_ok() {
                ParseResultMathStatus::Ok
            } else {
                ParseResultMathStatus::Ng
            },
            content: v.content(),
        }),
        _ => ParseResultColumnDetail::Text(String::new()),
    };

    ParseResultColumn {
        key: convert_key(key),
        kind,
        detail,
    }
}

fn convert_key(key: Key) -> ParseResultKey {
    ParseResultKey(format!("K{:04}", key.to_u32()))
}

fn convert_keys(keys: impl IntoIterator<Item = Key>) -> Vec<ParseResultKey> {
    keys.into_iter().map(convert_key).collect::<Vec<_>>()
}
