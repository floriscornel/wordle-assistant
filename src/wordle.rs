use std::collections::HashMap;

#[cfg(feature = "api_code")]
use serde::{Deserialize, Serialize};

use crate::word_list::WordList;
use crate::{Word, WORDLE_LETTER_COUNT};

pub type Guess = [(char, Feedback); WORDLE_LETTER_COUNT];

#[cfg_attr(feature = "api_code", derive(Serialize, Deserialize, Debug))]
pub enum Feedback {
    NotCorrect,
    OtherLocation,
    Correct,
}

pub struct Wordle {
    word_list: WordList,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            word_list: WordList::new(),
        }
    }

    pub fn guess(&mut self, guess: Guess) {
        let mut min_counts = HashMap::new();
        for idx in 0..WORDLE_LETTER_COUNT {
            let (char, feedback): (u8, &Feedback) = (guess[idx].0 as u8, &guess[idx].1);
            let min_count = min_counts.entry(char).or_insert(0);
            match feedback {
                Feedback::NotCorrect => {
                    self.word_list.incorrect(char, idx);
                }
                Feedback::Correct => {
                    self.word_list.correct(char, idx);
                    *min_count += 1;
                }
                Feedback::OtherLocation => {
                    self.word_list.other_location(char, idx);
                    *min_count += 1;
                }
            }
        }
        self.word_list.char_counts(min_counts);
    }

    pub fn permutations(&self) -> Vec<Word> {
        self.word_list.set.iter().map(|&x| x).collect()
    }
}

impl Default for Wordle {
    fn default() -> Self {
        Self::new()
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
            HashSet::<String>::from_iter(
                found
                    .into_iter()
                    .map(|s| String::from_utf8(s.to_vec()).unwrap())
            ),
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
            HashSet::<String>::from_iter(
                found
                    .into_iter()
                    .map(|s| String::from_utf8(s.to_vec()).unwrap())
            ),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }

    #[test]
    fn example_3() {
        let mut wordle = Wordle::new();
        //BEGIN
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
        let expected = vec!["NEXIN", "LEVIN", "BEGIN"];
        assert_eq!(
            HashSet::<String>::from_iter(
                found
                    .into_iter()
                    .map(|s| String::from_utf8(s.to_vec()).unwrap())
            ),
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
        let expected = vec!["NELIA", "ENTIA", "DIANE", "ELAIN", "INANE", "LIANE"];
        assert_eq!(
            HashSet::<String>::from_iter(
                found
                    .into_iter()
                    .map(|s| String::from_utf8(s.to_vec()).unwrap())
            ),
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
            HashSet::<String>::from_iter(
                found
                    .into_iter()
                    .map(|s| String::from_utf8(s.to_vec()).unwrap())
            ),
            HashSet::<String>::from_iter(expected.into_iter().map(|s| s.to_string()))
        );
    }
}
