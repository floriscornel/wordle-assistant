#![allow(dead_code, unused_variables)]

pub const WORDLE_LETTER_COUNT: usize = 5;

use crate::{choice::order_choices, word_list::WordList};

pub type Guess = [(char, Feedback); WORDLE_LETTER_COUNT];

#[derive(Debug)]
pub enum Feedback {
    NotCorrect,
    OtherLocation,
    Correct,
}

#[derive(Debug, Default)]
pub struct Wordle {
    guesses: Vec<Guess>,
    word_list: WordList,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            guesses: Vec::new(),
            word_list: WordList::new(),
        }
    }

    pub fn guess(&mut self, guess: Guess) {
        self.guesses.push(guess);
    }

    pub fn permutations(&self) -> Vec<String> {
        let mut found = Vec::new();
        for word in &self.word_list.set {
            if self.check(word) {
                found.push(*word);
            }
        }
        order_choices(&mut found);
        found
            .iter()
            .map(|arr| String::from_utf8(arr.to_vec()).unwrap())
            .collect()
    }

    fn check(&self, word: &[u8; WORDLE_LETTER_COUNT]) -> bool {
        for guess in &self.guesses {
            let mut allowed = vec![];
            for idx in 0..WORDLE_LETTER_COUNT {
                let (char, feedback): (u8, &Feedback) = (guess[idx].0 as u8, &guess[idx].1);
                match feedback {
                    Feedback::NotCorrect => {
                        if !allowed.contains(&char) && word.contains(&char) {
                            return false;
                        } else if allowed.contains(&char) {
                            let allowed_count = allowed.iter().filter(|&x| *x == char).count();
                            let found_count = word.iter().filter(|&x| *x == char).count();
                            if found_count > allowed_count {
                                return false;
                            }
                        }
                    }
                    Feedback::Correct => {
                        if word[idx] != char {
                            return false;
                        }
                        allowed.push(char);
                    }
                    Feedback::OtherLocation => {
                        if word[idx] == char || !word.contains(&char) {
                            return false;
                        }
                        allowed.push(char);
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Feedback::*, Wordle};

    #[test]
    fn example_1() {
        let mut wordle = Wordle::new();
        wordle.guess([
            ('H', NotCorrect),
            ('A', NotCorrect),
            ('I', OtherLocation),
            ('K', NotCorrect),
            ('U', NotCorrect),
        ]);
        wordle.guess([
            ('P', OtherLocation),
            ('O', NotCorrect),
            ('N', OtherLocation),
            ('E', OtherLocation),
            ('Y', NotCorrect),
        ]);
        wordle.guess([
            ('S', NotCorrect),
            ('P', OtherLocation),
            ('I', OtherLocation),
            ('N', OtherLocation),
            ('E', OtherLocation),
        ]);
        wordle.guess([
            ('R', NotCorrect),
            ('E', OtherLocation),
            ('P', OtherLocation),
            ('I', OtherLocation),
            ('N', OtherLocation),
        ]);
        let found = wordle.permutations();
        let expected = vec!["INEPT"];
        assert_eq!(
            HashSet::<String>::from_iter(found.into_iter()),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }

    #[test]
    fn example_2() {
        let mut wordle = Wordle::new();
        wordle.guess([
            ('H', NotCorrect),
            ('A', OtherLocation),
            ('I', NotCorrect),
            ('K', NotCorrect),
            ('U', OtherLocation),
        ]);
        wordle.guess([
            ('P', NotCorrect),
            ('O', OtherLocation),
            ('N', NotCorrect),
            ('E', NotCorrect),
            ('Y', NotCorrect),
        ]);
        wordle.guess([
            ('A', Correct),
            ('B', NotCorrect),
            ('O', Correct),
            ('U', Correct),
            ('T', NotCorrect),
        ]);
        let found = wordle.permutations();
        let expected = vec!["AFOUL", "ALOUD", "AMOUR"];
        assert_eq!(
            HashSet::<String>::from_iter(found.into_iter()),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }

    #[test]
    fn example_3() {
        let mut wordle = Wordle::new();
        wordle.guess([
            ('H', NotCorrect),
            ('A', NotCorrect),
            ('I', OtherLocation),
            ('K', NotCorrect),
            ('U', NotCorrect),
        ]);
        wordle.guess([
            ('M', NotCorrect),
            ('O', NotCorrect),
            ('N', OtherLocation),
            ('E', OtherLocation),
            ('Y', NotCorrect),
        ]);
        wordle.guess([
            ('S', NotCorrect),
            ('P', NotCorrect),
            ('I', OtherLocation),
            ('N', OtherLocation),
            ('E', OtherLocation),
        ]);
        wordle.guess([
            ('E', OtherLocation),
            ('N', OtherLocation),
            ('T', NotCorrect),
            ('E', NotCorrect),
            ('R', NotCorrect),
        ]);
        let found = wordle.permutations();
        let expected = vec!["BEGIN", "LEVIN"];
        assert_eq!(
            HashSet::<String>::from_iter(found.into_iter()),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }

    #[test]
    fn example_4() {
        let mut wordle = Wordle::new();
        wordle.guess([
            ('H', NotCorrect),
            ('A', OtherLocation),
            ('I', OtherLocation),
            ('K', NotCorrect),
            ('U', NotCorrect),
        ]);
        wordle.guess([
            ('M', NotCorrect),
            ('O', NotCorrect),
            ('N', OtherLocation),
            ('E', OtherLocation),
            ('Y', NotCorrect),
        ]);
        let found = wordle.permutations();
        let expected = vec!["DIANE", "ENTIA", "ELAIN", "INANE", "LIANE"];
        assert_eq!(
            HashSet::<String>::from_iter(found.into_iter()),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }

    #[test]
    fn example_5() {
        let mut wordle = Wordle::new();
        wordle.guess([
            ('T', OtherLocation),
            ('R', NotCorrect),
            ('A', NotCorrect),
            ('I', OtherLocation),
            ('N', NotCorrect),
        ]);
        wordle.guess([
            ('E', NotCorrect),
            ('M', NotCorrect),
            ('I', OtherLocation),
            ('T', OtherLocation),
            ('S', NotCorrect),
        ]);
        wordle.guess([
            ('P', NotCorrect),
            ('I', OtherLocation),
            ('L', NotCorrect),
            ('O', NotCorrect),
            ('T', OtherLocation),
        ]);
        let found = wordle.permutations();
        let expected = vec!["ITCHY"];
        assert_eq!(
            HashSet::<String>::from_iter(found.into_iter()),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }
}
