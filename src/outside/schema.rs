use crate::key::Key;
use crate::node::Node;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum ParseResult {
    #[serde(rename = "ok")]
    Ok(ParseResultOk),
    #[serde(rename = "error")]
    Error(ParseResultError),
}

impl ParseResult {
    pub(super) fn new_ok(root: ParseResultKey, columns: Vec<ParseResultColumn>) -> Self {
        Self::Ok(ParseResultOk { root, columns })
    }

    pub(super) fn new_error(message: String) -> Self {
        Self::Error(ParseResultError { message })
    }
}

#[derive(Debug, Serialize)]
pub struct ParseResultOk {
    root: ParseResultKey,
    columns: Vec<ParseResultColumn>,
}

#[derive(Debug, Serialize)]
pub struct ParseResultError {
    message: String,
}

#[derive(Debug, Serialize)]
pub(super) struct ParseResultKey(String);

#[derive(Debug, Serialize)]
pub(super) struct ParseResultColumn {
    key: ParseResultKey,
    kind: ParseResultColumnKind,
    detail: ParseResultColumnDetail,
}

#[derive(Debug, Serialize)]
enum ParseResultColumnDetail {
    Paragraphs(Vec<ParseResultKey>),
    Paragraph(Vec<ParseResultKey>),
    Text(String),
    InlineCommand(String),
    InlineMath(ParseResultMath),
    DisplayMath(ParseResultMath),
}

#[derive(Debug, Serialize)]
struct ParseResultMath {
    status: ParseResultMathStatus,
    content: String,
}

#[derive(Debug, Serialize)]
enum ParseResultMathStatus {
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

pub(super) fn convert_to_column(key: Key, node: Node) -> ParseResultColumn {
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

pub(super) fn convert_key(key: Key) -> ParseResultKey {
    ParseResultKey(format!("K{:04}", key.to_u32()))
}

fn convert_keys(keys: impl IntoIterator<Item = Key>) -> Vec<ParseResultKey> {
    keys.into_iter().map(convert_key).collect::<Vec<_>>()
}
