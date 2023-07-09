#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub(super) struct Key {
    value: u32,
}

impl Key {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}

#[derive(Debug)]
pub(super) struct KeyCounter {
    next: Key,
}

impl KeyCounter {
    pub fn new() -> Self {
        Self { next: Key::new() }
    }

    pub fn count(&mut self) -> Key {
        let k = self.next.clone();
        self.next.increment();
        k
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn KeyCounter() {
        let mut kc = KeyCounter::new();
        assert_eq!(kc.count(), Key { value: 0 });
        assert_eq!(kc.count(), Key { value: 1 });
        assert_eq!(kc.count(), Key { value: 2 });
    }
}
