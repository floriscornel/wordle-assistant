use inquire::{validator::Validation, Text};

use wordle_assistant::wordle::{Feedback, Guess, Wordle};

pub fn main() {
    let mut wordle = Wordle::new();
    print_choices(&wordle);

    while let Some(guess) = get_guess() {
        wordle.guess(guess);
        print_choices(&wordle);
    }
}

fn get_guess() -> Option<Guess> {
    let validator = |input: &str| {
        if input.chars().count() != 5 {
            Ok(Validation::Invalid("Guess must be 5 characters.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let guess = Text::new("What was your word guess?")
        .with_validator(validator)
        .prompt();

    match guess {
        Ok(guess) => {
            println!("Your guess was {guess}.");
            let mut guesses = vec![];
            for c in guess.chars() {
                let char = c.to_ascii_uppercase();
                guesses.push((char, get_feedback(char)));
            }
            guesses.try_into().ok()
        }
        Err(err) => {
            println!("Error, your guess was invalid: {err}");
            None
        }
    }
}

fn get_feedback(char: char) -> Feedback {
    use inquire::{error::InquireError, Select};

    let options: Vec<&str> = vec!["Incorrect", "Other Location", "Correct"];

    let ans: Result<&str, InquireError> =
        Select::new(&format!("What was the result for '{char}'?"), options).prompt();

    match ans {
        Ok(choice) => match choice {
            "Incorrect" => Feedback::NotCorrect,
            "Other Location" => Feedback::OtherLocation,
            "Correct" => Feedback::Correct,
            _ => Feedback::NotCorrect,
        },
        Err(_) => Feedback::NotCorrect,
    }
}

fn print_choices(wordle: &Wordle) {
    let permutations = wordle
        .ordered_permutations()
        .into_iter()
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect::<Vec<_>>();
    println!(
        " {:?} Possible words: {:?}",
        permutations.len(),
        permutations.iter().take(50).collect::<Vec<_>>()
    );
}
