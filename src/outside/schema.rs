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
    pub(super) fn new_ok(root: EntryKey, entries: Vec<Entry>, char_count: usize) -> Self {
        Self::Ok(ParseResultOk {
            root,
            entries,
            count: char_count,
        })
    }

    pub(super) fn new_error(message: String) -> Self {
        Self::Error(ParseResultError { message })
    }
}

#[derive(Debug, Serialize)]
pub struct ParseResultOk {
    root: EntryKey,
    entries: Vec<Entry>,
    count: usize,
}

#[derive(Debug, Serialize)]
pub struct ParseResultError {
    message: String,
}

#[derive(Debug, Serialize)]
pub(super) struct Entry {
    key: EntryKey,
    // #[serde(flatten)]
    value: EntryValue,
}

#[derive(Debug, Serialize)]
pub(super) struct EntryKey(String);

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
enum EntryValue {
    Paragraphs(EVKeys),
    Paragraph(EVKeys),
    Text(EVText),
    InlineCommand(EVInlineCommand),
    InlineMath(EVMath),
    DisplayMath(EVMath),
}

#[derive(Debug, Serialize)]
struct EVKeys {
    keys: Vec<EntryKey>,
}

impl EVKeys {
    fn new(keys: impl IntoIterator<Item = EntryKey>) -> Self {
        Self {
            keys: keys.into_iter().collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct EVText {
    content: String,
}

impl EVText {
    fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct EVInlineCommand {
    content: String,
}

impl EVInlineCommand {
    fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

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

pub(super) fn convert_to_entry(key: Key, node: Node) -> Entry {
    let value = match node {
        Node::ParagraphList(Some(ks)) => EntryValue::Paragraphs(EVKeys::new(convert_keys(ks))),
        Node::Paragraph(Some(ks)) => EntryValue::Paragraph(EVKeys::new(convert_keys(ks))),
        Node::RawString(s) => EntryValue::Text(EVText::new(s)),
        Node::InlineCommand(Some(s)) => EntryValue::InlineCommand(EVInlineCommand::new(s)),
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
        _ => EntryValue::Text(EVText::new("unknown")), // todo
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
