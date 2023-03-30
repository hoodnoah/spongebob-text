//! # SpongeBob_Text
//!
//! A simple CLI utility to convert text to SpongeBob text.

use sb::{arg_parsing, text_conversion};

fn main() {
    let input_string = arg_parsing::read_args(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let output_str = text_conversion::convert_to_sb_text(&input_string);

    println!("{}", output_str);
}
