use std::collections::{HashMap, HashSet};

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

    pub fn incorrect(&mut self, char: u8, idx: usize) {
        self.set.retain(|word| word[idx] != char);
    }

    pub fn correct(&mut self, char: u8, idx: usize) {
        self.set.retain(|word| word[idx] == char);
    }

    pub fn other_location(&mut self, char: u8, idx: usize) {
        self.set
            .retain(|word| word.contains(&char) && word[idx] != char);
    }

    pub fn char_counts(&mut self, counts: HashMap<u8, usize>) {
        for (char, count) in counts {
            if count == 0 {
                self.set.retain(|word| !word.contains(&char));
            } else {
                self.set
                    .retain(|word| word.iter().filter(|&x| *x == char).count() >= count);
            }
        }
    }
}

impl Default for WordList {
    fn default() -> Self {
        Self::new()
    }
}
