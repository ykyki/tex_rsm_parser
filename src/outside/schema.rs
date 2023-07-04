use crate::key::Key;
use crate::node::Node;
use serde::Serialize;
use std::collections::HashMap;

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
enum EntryValue {
    #[serde(rename = "paras")]
    Paragraphs(EVKeys),
    #[serde(rename = "para")]
    Paragraph(EVKeys),
    #[serde(rename = "text")]
    Text(EVText),
    #[serde(rename = "il_cmd")]
    InlineCommand(EVInlineCommand),
    #[serde(rename = "il_math")]
    InlineMath(EVMath),
    #[serde(rename = "ds_math")]
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

pub(super) fn convert_to_entry(key: Key, node: Node, hash_table: &HashMap<Key, String>) -> Entry {
    let value = match node {
        Node::ParagraphList(Some(ks)) => {
            EntryValue::Paragraphs(EVKeys::new(convert_keys(ks, hash_table)))
        }
        Node::Paragraph(Some(ks)) => {
            EntryValue::Paragraph(EVKeys::new(convert_keys(ks, hash_table)))
        }
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
        key: convert_key(key, hash_table),
        value,
    }
}

pub(super) fn convert_key(key: Key, hash_table: &HashMap<Key, String>) -> EntryKey {
    EntryKey(hash_table.get(&key).unwrap().to_owned())
}

fn convert_keys(
    keys: impl IntoIterator<Item = Key>,
    hash_table: &HashMap<Key, String>,
) -> Vec<EntryKey> {
    keys.into_iter()
        .map(|k| convert_key(k, hash_table))
        .collect::<Vec<_>>()
}
