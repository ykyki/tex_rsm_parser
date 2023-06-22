use crate::key::Key;
use crate::node::Node;
use std::collections::BTreeMap;

#[derive(Debug)]
pub(super) struct ResultMap {
    root: Key,
    columns: BTreeMap<Key, Node>,
}

impl ResultMap {
    pub(crate) fn new(key: Key, node: Node) -> Self {
        Self {
            root: key.clone(),
            columns: BTreeMap::from([(key, node)]),
        }
    }

    pub(crate) fn root(&self) -> Key {
        self.root.clone()
    }

    pub(crate) fn merge(&mut self, children: impl IntoIterator<Item = Self>) {
        for child in children {
            self.columns.extend(child.columns);
        }
    }
}

impl IntoIterator for ResultMap {
    type Item = (Key, Node);
    type IntoIter = std::collections::btree_map::IntoIter<Key, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.columns.into_iter()
    }
}
