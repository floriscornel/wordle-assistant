use serde::{Deserialize, Serialize};

use crate::{
    wordle::{Feedback, Wordle},
    WORDLE_LETTER_COUNT,
};

#[derive(Serialize, Deserialize)]
pub struct GuessElem {
    letter: char,
    feedback: Feedback,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    guesses: Vec<[GuessElem; WORDLE_LETTER_COUNT]>,
}

#[derive(Serialize, Deserialize)]
pub struct WordleResponse {
    recommendations: Vec<String>,
}

pub fn get_recommendations(payload: State) -> WordleResponse {
    let mut wordle = Wordle::new();

    for guess in payload.guesses {
        let guess = guess
            .into_iter()
            .map(|x| (x.letter.to_ascii_uppercase(), x.feedback))
            .collect::<Vec<(char, Feedback)>>();
        wordle.guess(guess.try_into().unwrap());
    }

    let recommendations = wordle
        .ordered_permutations()
        .into_iter()
        .take(50)
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect();

    WordleResponse { recommendations }
}
