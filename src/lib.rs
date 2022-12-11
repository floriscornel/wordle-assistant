pub mod choice_order;
pub mod word_list;
pub mod wordle;

#[cfg(feature = "api_code")]
pub mod recommendation_api;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
