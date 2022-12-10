# Wordle Assistant
![](./docs/wordle-assistant-preview.gif)

This is a simple CLI tool that assist in solving wordle puzzles.

## Usage
The CLI will show give a list of best choices to choose from at any point in the game.
```
 12971 Possible words: ["OTHER", "THEIR", "WHICH", "THERE", "FIRST", "THESE", "WOULD", "PRICE", "STATE", "EMAIL", "CLICK", "YEARS", "AFTER", "MUSIC", "WORLD", "ITEMS", "STORE", "GAMES", "VIDEO", "GREAT", "LINKS", "TERMS", "THOSE", "ORDER", "UNDER", "BASED", "HOTEL", "WHERE", "PAGES", "BOOKS", "HOUSE", "GROUP", "USING", "SINCE", "COULD", "TIMES", "WATER", "MEDIA", "PHONE", "RATES", "SOUTH", "LEARN", "PLACE", "TODAY", "LOCAL", "RIGHT", "SITES", "SALES", "POWER", "WOMEN"]
``` 
As a user you can register which guess you've made in Wordle.
You should then input the feedback from Wordle for each of the letters from your guess. You can choose from
```
> What was your word guess? OTHER
Your guess was OTHER.
? What was the result for 'O'?
> Incorrect
  Other Location
  Correct
[↑↓ to move, enter to select, type to filter]
```
The application will exit automatically once the number of possible words has narrowed down to one.

## Build instructions
1. Check out the git repository
2. Run build command with bin `cli` and features `cli`:
```bash
cargo build --release --bin cli --features cli
```
3. Run file `./target/release/cli`
