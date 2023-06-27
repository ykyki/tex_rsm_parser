use crate::key::Key;
use crate::node::Node;
use crate::parser::parse_paragraphs;
use serde::Serialize;

pub fn parse_paragraphs_to_json(input: &str) -> serde_json::Value {
    let result = parse_paragraphs(input);

    match result {
        Err(e) => serde_json::json!({
            "status": "error",
            "message": e.to_string(),
        }),
        Ok(map) => {
            let root = convert_key(map.root());
            let columns = map
                .into_iter()
                .map(|(key, node)| convert_column(key, node))
                .collect::<Vec<_>>();

            serde_json::json!({
                "status": "ok",
                "root": root,
                "columns": columns,
            })
        }
    }
}

pub(crate) const MAX_INPUT_LENGTH: usize = 1_000;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Too long input. The input must be less than {MAX_INPUT_LENGTH} characters.")]
    TooLongInput,
}

#[derive(Debug, Serialize)]
struct ColumnSchema {
    key: String,
    kind: NodeKind,
    detail: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum NodeKind {
    Paragraphs,
    Paragraph,
    Text,
    InlineCommand,
    InlineMath,
    DisplayMath,
}

fn convert_key(key: Key) -> String {
    format!("k{}", key.to_u32())
}

fn convert_keys(keys: impl IntoIterator<Item = Key>) -> serde_json::Value {
    serde_json::json!(keys.into_iter().map(convert_key).collect::<Vec<_>>())
}

fn convert_column(key: Key, node: Node) -> ColumnSchema {
    let kind = match &node {
        Node::ParagraphList(_) => NodeKind::Paragraphs,
        Node::Paragraph(_) => NodeKind::Paragraph,
        Node::RawString(_) => NodeKind::Text,
        Node::InlineCommand(_) => NodeKind::InlineCommand,
        Node::MathExpr(v) => {
            if v.is_inline() {
                NodeKind::InlineMath
            } else if v.is_display() {
                NodeKind::DisplayMath
            } else {
                unreachable!()
            }
        }
    };

    let detail = match node {
        Node::ParagraphList(Some(ks)) => convert_keys(ks),
        Node::Paragraph(Some(ks)) => convert_keys(ks),
        Node::RawString(s) => serde_json::Value::String(s),
        Node::InlineCommand(Some(s)) => serde_json::Value::String(s),
        Node::MathExpr(v) => serde_json::json!({
            "status": if v.is_ok() { "ok" } else { "ng" },
            "convent": v.content(),
        }),
        _ => serde_json::json!({}),
    };

    ColumnSchema {
        key: convert_key(key),
        kind,
        detail,
    }
}
