#[cfg(feature = "cli")]
use wordle_assistant::cli::interactive_cli;

fn main() {
    #[cfg(feature = "cli")]
    interactive_cli()
}
