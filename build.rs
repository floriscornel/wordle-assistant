use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const WORDLE_LETTER_COUNT: usize = 5;
const WORDS_TXT_PATH: &str = "./data/5-letter-words.txt";
const FREQUENCIES_CSV_PATH: &str = "./data/5-letter_frequencies.csv";

type Word = [u8; WORDLE_LETTER_COUNT];

fn get_word_list() -> Vec<Word> {
    let mut rdr = csv::Reader::from_path(WORDS_TXT_PATH).unwrap();
    let mut list = vec![];

    for result in rdr.records() {
        let record = result.unwrap();
        let word = record.get(0).unwrap();
        if word.len() == WORDLE_LETTER_COUNT {
            list.push(word.to_uppercase().as_bytes().try_into().unwrap());
        }
    }
    list
}

// function that reads a csv file from a path and returns a vector of Frequency
fn get_frequency_map() -> HashMap<Word, u64> {
    let mut rdr = csv::Reader::from_path(FREQUENCIES_CSV_PATH).unwrap();
    let mut frequencies = HashMap::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let word = record.get(0).unwrap();
        let count = record.get(1).unwrap().parse::<u64>().unwrap();
        if word.len() == WORDLE_LETTER_COUNT {
            frequencies.insert(word.to_uppercase().as_bytes().try_into().unwrap(), count);
        }
    }
    frequencies
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = phf_codegen::Map::new();

    let words = get_word_list();
    let frequencies = get_frequency_map();

    for word in words {
        let count = frequencies.get(&word).unwrap_or(&0);
        map.entry(word, &format!("{count}"));
    }

    writeln!(
        &mut file,
        "pub const WORDLE_LETTER_COUNT: usize = {};",
        WORDLE_LETTER_COUNT
    )
    .unwrap();
    writeln!(&mut file, "pub type Word = [u8; WORDLE_LETTER_COUNT];",).unwrap();
    write!(
        &mut file,
        "static WORD_LIST: phf::Map<Word, u64> = {}",
        map.build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();
}
