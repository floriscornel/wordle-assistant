use std::collections::HashMap;

use crate::{wordle::Wordle, Word, WORDLE_LETTER_COUNT, WORD_LIST};

const LETTER_CORRECT_MODIFIER: f64 = 5.0;
const LETTER_OTHER_MODIFIER: f64 = 0.5;
const FREQUENCY_MODIFIER: f64 = 7.0;

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
    let mut letter_counts = HashMap::<u8, [usize; 5]>::new();
    let (word_frequencies, freq_max) = get_frequency_map();

    for word in words.clone() {
        for (idx, letter) in word.iter().enumerate() {
            let counts = letter_counts.entry(*letter).or_insert([0; 5]);
            counts[idx] += 1;
        }
    }

    let max_letters = letter_counts
        .iter()
        .map(|(_, counts)| counts.iter().sum::<usize>())
        .max()
        .unwrap_or(0);

    let get_value = |word: &Word| -> u64 {
        let mut letter_score = 0.0f64;
        for (idx, letter) in word.iter().enumerate() {
            let counts = letter_counts.get(letter).unwrap_or(&[0; 5]);
            letter_score += (counts[idx] as f64) / (max_letters as f64) * LETTER_CORRECT_MODIFIER;
            letter_score += (counts.iter().sum::<usize>() as f64) / (max_letters as f64)
                * LETTER_OTHER_MODIFIER;
        }
        let frequency_score =
            1.0 + *word_frequencies.get(word).unwrap_or(&0) as f64 / freq_max as f64;
        (letter_score + frequency_score * FREQUENCY_MODIFIER) as u64
    };

    let mut mapped: Vec<([u8; WORDLE_LETTER_COUNT], u64)> = words
        .iter()
        .map(|word| (*word, get_value(word)))
        .collect::<_>();
    mapped.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    *words = mapped.iter().map(|(word, _)| *word).collect::<Vec<Word>>();
}

// #[cfg(test)]
// mod tests {
//     use crate::wordle::Wordle;

//     #[test]
//     fn check_preferred_order() {
//         let expected = vec![
//             "SALES", "OTHER", "GAMES", "SITES", "THERE", "YEARS", "SATES", "SAMES", "SANES",
//             "RATES", "PARES", "BOOKS", "SONES", "SERES",
//         ];

//         let words = Wordle::new()
//             .ordered_permutations()
//             .into_iter()
//             .take(50)
//             .map(|x| String::from_utf8(x.to_vec()).unwrap())
//             .collect::<Vec<String>>();

//         let expected_transformed = expected
//             .into_iter()
//             .map(|x| x.to_owned())
//             .collect::<Vec<String>>();

//         assert_eq!(words, expected_transformed);
//     }
// }
