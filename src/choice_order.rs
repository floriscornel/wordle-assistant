use std::collections::{HashMap, HashSet};

use crate::{wordle::Wordle, Word, WORDLE_LETTER_COUNT, WORD_LIST};

impl Wordle {
    pub fn ordered_permutations(&self) -> Vec<Word> {
        let mut words = self.permutations();
        order_choices(&mut words);
        words
    }
}

fn get_frequency_map() -> (HashMap<Word, u64>, u64) {
    let mut max = u64::MIN;
    let map = HashMap::from_iter(WORD_LIST.into_iter().map(|(w, c)| {
        max = max.max(*c);
        (*w, *c)
    }));
    (map, max)
}

// Order the choices by the number of times each letter appears in the word
fn order_choices(words: &mut Vec<Word>) {
    let mut letter_counts = HashMap::<u8, i32>::new();
    let (word_frequencies, freq_max) = get_frequency_map();

    for word in words.clone() {
        for letter in HashSet::<&u8>::from_iter(word.iter()) {
            let count = letter_counts.entry(*letter).or_insert(0);
            *count += 1;
        }
    }

    let get_value = |word: &Word| -> u64 {
        let mut letter_score = 0;
        for letter in HashSet::<&u8>::from_iter(word.iter()) {
            letter_score += letter_counts.get(letter).unwrap();
        }
        let letter_score: f64 = (letter_score as f64) / (words.len() * WORDLE_LETTER_COUNT) as f64;
        let frequency_score = *word_frequencies.get(word).unwrap_or(&0) as f64 / freq_max as f64;
        (10_000_000.0 * (10.0 * letter_score + frequency_score)) as _
    };

    let mut mapped: Vec<([u8; WORDLE_LETTER_COUNT], u64)> = words
        .iter()
        .map(|word| (*word, get_value(word)))
        .collect::<_>();
    mapped.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    *words = mapped.iter().map(|(word, _)| *word).collect::<Vec<Word>>();
}

#[cfg(test)]
mod tests {
    use crate::wordle::Wordle;

    #[test]
    fn check_preferred_order() {
        let expected = vec![
            "YEARS", "AROSE", "AEROS", "SOARE", "OTHER", "RATES", "RAISE", "ARISE", "SERIA",
            "REAIS", "SERAI", "AESIR", "STORE", "LASER",
        ];

        let words = Wordle::new()
            .ordered_permutations()
            .into_iter()
            .take(expected.len())
            .map(|x| String::from_utf8(x.to_vec()).unwrap())
            .collect::<Vec<String>>();

        let expected_transformed = expected
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        assert_eq!(words, expected_transformed);
    }
}
