use std::collections::{HashMap, HashSet};

use crate::{Word, WORDLE_LETTER_COUNT, WORD_LIST};

pub fn get_frequency_map() -> (HashMap<Word, u64>, u64) {
    let mut max = u64::MIN;
    let map = HashMap::from_iter(WORD_LIST.into_iter().map(|(w, c)| {
        max = max.max(*c);
        (*w, *c)
    }));
    (map, max)
}

// Order the choices by the number of times each letter appears in the word
pub fn order_choices(words: &mut Vec<Word>) {
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
        (10_000_000.0 * (letter_score + frequency_score)) as _
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
    use super::{order_choices, Word};

    #[test]
    fn check_preferred_order() {
        let input = vec![
            "AROSE", "SOARE", "AEROS", "SERAI", "REAIS", "ARISE", "RAISE", "AESIR", "ALOES",
            "REALS", "LAERS", "SERAL", "ARLES", "LEARS", "RALES", "LARES", "EARLS", "LASER",
            "TOEAS", "STOAE", "RESAT", "ARETS", "STRAE", "REAST", "STARE", "EARST", "ASTER",
            "TEARS", "STEAR", "TARES", "TASER", "TERAS", "RATES", "AISLE", "AEONS", "NARES",
            "SANER", "EARNS", "REANS", "NEARS", "SNARE", "ANISE", "ISNAE", "SAINE", "URSAE",
            "URASE", "UREAS", "AURES", "ARSED", "SARED",
        ];
        let expected = vec![
            "RATES", "LASER", "TEARS", "STARE", "ASTER", "TASER", "TARES", "STEAR", "TERAS",
            "REAST", "STRAE", "EARST", "RESAT", "ARETS", "RAISE", "REALS", "EARLS", "ARISE",
            "ARLES", "LARES", "SERAL", "RALES", "LEARS", "LAERS", "EARNS", "SNARE", "NEARS",
            "SANER", "NARES", "REANS", "REAIS", "SERAI", "AESIR", "AROSE", "AEROS", "SOARE",
            "URSAE", "AURES", "URASE", "UREAS", "ARSED", "SARED", "TOEAS", "STOAE", "AISLE",
            "ANISE", "SAINE", "ISNAE", "ALOES", "AEONS",
        ];

        let mut words: Vec<Word> = input
            .into_iter()
            .filter(|x| x.len() == 5)
            .map(|x| x.as_bytes().try_into().unwrap())
            .collect();
        let expected_transformed = expected
            .into_iter()
            .filter(|x| x.len() == 5)
            .map(|x| x.as_bytes().try_into().unwrap())
            .collect::<Vec<Word>>();
        order_choices(&mut words);
        assert_eq!(words, expected_transformed);
    }
}
