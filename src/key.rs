#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Key {
    value: u32,
}

impl Key {
    fn new() -> Self {
        Self { value: 0 }
    }
}
