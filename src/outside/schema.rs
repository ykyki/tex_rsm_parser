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
    pub(super) fn new_ok(root: EntryKey, columns: Vec<Entry>) -> Self {
        Self::Ok(ParseResultOk { root, map: columns })
    }

    pub(super) fn new_error(message: String) -> Self {
        Self::Error(ParseResultError { message })
    }
}

#[derive(Debug, Serialize)]
pub struct ParseResultOk {
    root: EntryKey,
    map: Vec<Entry>,
}

#[derive(Debug, Serialize)]
pub struct ParseResultError {
    message: String,
}

#[derive(Debug, Serialize)]
pub(super) struct Entry {
    key: EntryKey,
    #[serde(flatten)]
    value: EntryValue,
}

#[derive(Debug, Serialize)]
pub(super) struct EntryKey(String);

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "value")]
#[serde(rename_all = "snake_case")]
enum EntryValue {
    Paragraphs(Vec<EntryKey>),
    Paragraph(Vec<EntryKey>),
    Text(String),
    InlineCommand(EVInlineCommand),
    InlineMath(EVMath),
    DisplayMath(EVMath),
}

#[derive(Debug, Serialize)]
struct EVInlineCommand(String);

#[derive(Debug, Serialize)]
struct EVMath {
    status: EVMathStatus,
    content: String,
}

#[derive(Debug, Serialize)]
enum EVMathStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "error")]
    Error,
}

pub(super) fn convert_to_column(key: Key, node: Node) -> Entry {
    // let kind = match &node {
    //     Node::ParagraphList(_) => ParseResultColumnKind::Paragraphs,
    //     Node::Paragraph(_) => ParseResultColumnKind::Paragraph,
    //     Node::RawString(_) => ParseResultColumnKind::Text,
    //     Node::InlineCommand(_) => ParseResultColumnKind::InlineCommand,
    //     Node::MathExpr(v) => {
    //         if v.is_inline() {
    //             ParseResultColumnKind::InlineMath
    //         } else if v.is_display() {
    //             ParseResultColumnKind::DisplayMath
    //         } else {
    //             unreachable!()
    //         }
    //     }
    // };

    let value = match node {
        Node::ParagraphList(Some(ks)) => EntryValue::Paragraphs(convert_keys(ks)),
        Node::Paragraph(Some(ks)) => EntryValue::Paragraph(convert_keys(ks)),
        Node::RawString(s) => EntryValue::Text(s),
        Node::InlineCommand(Some(s)) => EntryValue::InlineCommand(EVInlineCommand(s)),
        Node::MathExpr(v) => {
            let status = if v.is_ok() {
                EVMathStatus::Ok
            } else {
                EVMathStatus::Error
            };
            let is_inline = v.is_inline();
            let is_display = v.is_display();
            let content = v.content();

            if is_inline {
                EntryValue::InlineMath(EVMath { status, content })
            } else if is_display {
                EntryValue::DisplayMath(EVMath { status, content })
            } else {
                unreachable!()
            }
        }
        _ => EntryValue::Text(String::new()),
    };

    Entry {
        key: convert_key(key),
        value,
    }
}

pub(super) fn convert_key(key: Key) -> EntryKey {
    EntryKey(format!("K{:04}", key.to_u32()))
}

fn convert_keys(keys: impl IntoIterator<Item = Key>) -> Vec<EntryKey> {
    keys.into_iter().map(convert_key).collect::<Vec<_>>()
}
