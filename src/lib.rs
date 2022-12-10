pub mod choice;
pub mod cli;
pub mod word_list;
pub mod wordle;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
