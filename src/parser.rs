use crate::key::Key;
use crate::node::Node;
use std::collections::BTreeMap;

use crate::ParseError;

pub(super) fn parse_paragraphs(_input: &str) -> Result<ResultMap, ParseError> {
    todo!();
}

#[derive(Debug)]
pub(super) struct ResultMap {
    value: BTreeMap<Key, Node>, // todo
}
