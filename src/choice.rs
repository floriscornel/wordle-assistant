use crate::wordle::WORDLE_LETTER_COUNT;
use std::collections::{HashMap, HashSet};

type Word = [u8; WORDLE_LETTER_COUNT];

// Determine the value of a word by the number of times each letter appears in the word
fn get_value(word: &Word, map: &HashMap<u8, i32>) -> i32 {
    let mut score = 0;
    for letter in HashSet::<&u8>::from_iter(word.iter()) {
        score += map.get(letter).unwrap();
    }
    score
}

// Order the choices by the number of times each letter appears in the word
pub fn order_choices(words: &mut Vec<Word>) {
    let mut map = HashMap::<u8, i32>::new();
    for word in words.clone() {
        for letter in HashSet::<&u8>::from_iter(word.iter()) {
            let count = map.entry(*letter).or_insert(0);
            *count += 1;
        }
    }
    let mut mapped: Vec<([u8; WORDLE_LETTER_COUNT], i32)> = words
        .iter()
        .map(|word| (*word, get_value(word, &map)))
        .collect::<_>();
    mapped.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let x = mapped.iter().map(|(word, _)| *word).collect::<Vec<Word>>();
    *words = x;
}

#[cfg(test)]
mod tests {
    use super::{order_choices, Word};

    #[test]
    fn example_1() {
        let input = vec![
            "KEECH", "KEEFS", "KEEKS", "KEELS", "KEEMA", "KEENO", "KEENS", "KEEPS", "KEETS",
            "KEEVE", "KEFIR", "KEHUA", "KEIRS", "KELEP", "KELIM", "KELLS", "KELLY", "KELPS",
            "KELPY", "KELTS", "KELTY", "KEMBO", "KEMBS", "KEMPS", "KEMPT", "KEMPY", "KENAF",
            "KENCH", "KENDO", "KENOS", "KENTE", "KENTS", "KEPIS", "KERBS", "KEREL", "KERFS",
            "KERKY", "KERMA", "KERNE", "KERNS", "KEROS", "KERRY", "KERVE", "KESAR", "KESTS",
            "KETAS", "KETCH", "KETES", "KETOL", "KEVEL", "KEVIL", "KEXES", "KEYED", "KEYER",
            "KHADI", "KHAFS", "KHAKI", "KHANS", "KHAPH", "KHATS", "KHAYA", "KHAZI", "KHEDA",
            "KHETH", "KHETS", "KHOJA", "KHORS", "KHOUM", "KHUDS", "KIAAT",
        ];
        let expected = vec![
            "KHETS", "KETAS", "KESAR", "KELTS", "KENTS", "KERNS", "KEROS", "KEIRS", "KELPS",
            "KERFS", "KENOS", "KEPIS", "KEMPS", "KERBS", "KESTS", "KETES", "KEETS", "KELLS",
            "KEELS", "KEMBS", "KERMA", "KEENS", "KHEDA", "KEHUA", "KEEPS", "KETCH", "KELTY",
            "KETOL", "KEEFS", "KHETH", "KEMPT", "KENAF", "KENCH", "KELPY", "KELIM", "KEXES",
            "KEEKS", "KEFIR", "KEREL", "KEMPY", "KERNE", "KEVIL", "KENTE", "KEEMA", "KENDO",
            "KERRY", "KERKY", "KHATS", "KEYER", "KEECH", "KELLY", "KELEP", "KEMBO", "KEENO",
            "KHANS", "KERVE", "KEVEL", "KHORS", "KEYED", "KHAFS", "KEEVE", "KHUDS", "KHADI",
            "KHAZI", "KHOJA", "KHAPH", "KHAYA", "KHAKI", "KIAAT", "KHOUM",
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
