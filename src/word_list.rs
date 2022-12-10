use std::collections::HashSet;

use crate::{Word, WORD_LIST};

#[derive(Debug)]
pub struct WordList {
    pub set: HashSet<Word>,
}

impl WordList {
    pub fn new() -> Self {
        Self {
            set: HashSet::from_iter(WORD_LIST.into_iter().map(|(&x, _)| x)),
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.set.contains(key.as_bytes())
    }
}

impl Default for WordList {
    fn default() -> Self {
        Self::new()
    }
}
