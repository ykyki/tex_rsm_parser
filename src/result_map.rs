use crate::key::Key;
use crate::node::Node;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub(super) struct ResultMap {
    root: Key,
    entries: BTreeMap<Key, Node>,
}

impl ResultMap {
    pub(crate) fn new(key: Key, node: Node) -> Self {
        Self {
            root: key.clone(),
            entries: BTreeMap::from([(key, node)]),
        }
    }

    pub(crate) fn root(&self) -> Key {
        self.root.clone()
    }

    pub(crate) fn merge(&mut self, children: impl IntoIterator<Item = Self>) {
        for child in children {
            self.entries.extend(child.entries);
        }
    }

    pub(crate) fn hash_table(&self) -> HashMap<Key, String> {
        let mut table = HashMap::new();

        self.hash_by_value_at(&self.root, &mut table);

        table
    }

    fn hash_by_value_at(&self, key: &Key, table: &mut HashMap<Key, String>) -> String {
        if let Some(hash) = table.get(key) {
            return hash.to_owned();
        }

        let mut hasher = DefaultHasher::new();

        if let Some(node) = self.entries.get(key) {
            match node {
                Node::ParagraphList(Some(ks)) => {
                    for k in ks {
                        let hash = self.hash_by_value_at(k, table);
                        hash.hash(&mut hasher);
                    }
                }
                Node::Paragraph(Some(ks)) => {
                    for k in ks {
                        let hash = self.hash_by_value_at(k, table);
                        hash.hash(&mut hasher);
                    }
                }
                Node::RawString(s) => {
                    s.hash(&mut hasher);
                }
                Node::InlineCommand(Some(s)) => {
                    s.hash(&mut hasher);
                }
                Node::MathExpr(me) => {
                    me.hash(&mut hasher);
                }
                _ => {
                    // do nothing
                }
            }
        } else {
            // do nothing
            // 基本的には unreachable!() の想定
        }

        let hash = format!("{:x}", hasher.finish());
        table.insert(key.clone(), hash.clone());
        hash
    }
}

impl IntoIterator for ResultMap {
    type Item = (Key, Node);
    type IntoIter = std::collections::btree_map::IntoIter<Key, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
