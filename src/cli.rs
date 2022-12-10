#[cfg(feature = "cli")]
use crate::wordle::{Feedback, Guess, Wordle};
#[cfg(feature = "cli")]
use inquire::{validator::Validation, Text};

#[cfg(feature = "cli")]
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
            println!("Your guess was {}.", guess);
            let mut guesses = vec![];
            for char in guess.chars() {
                guesses.push((char, get_feedback(char)));
            }
            guesses.try_into().ok()
        }
        Err(err) => {
            println!("Error, your guess was invalid: {}", err);
            None
        }
    }
}

#[cfg(feature = "cli")]
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

#[cfg(feature = "cli")]
pub fn interactive_cli() {
    let mut wordle = Wordle::new();
    let permutations = wordle.permutations();
    println!(
        " {:?} Possible words: {:?}",
        permutations.len(),
        permutations.iter().take(50).collect::<Vec<_>>()
    );

    while let Some(guess) = get_guess() {
        wordle.guess(guess);
        let permutations = wordle.permutations();
        println!(
            " {:?} Possible words: {:?}",
            permutations.len(),
            permutations.iter().take(50).collect::<Vec<_>>()
        );
        if permutations.len() <= 2 {
            break;
        }
    }
}
