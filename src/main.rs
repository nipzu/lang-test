#![feature(once_cell)]

mod ast;
mod token;
mod tokenizer;

use ast::Program;
use tokenizer::{TokenizingError, TokenizingErrorKind};

fn main() {
    let contents = include_str!("../example.txt");
    match tokenizer::tokenize_text(contents) {
        Ok((tokens, literal_data)) => {
            println!("{:#?}", tokens);
            let _program = Program::from_tokens(tokens.into_iter(), &literal_data);
        }
        Err(e) => print_tokenizing_error(contents, &e),
    };
}

fn print_tokenizing_error(contents: &str, error: &TokenizingError) {
    let line = contents
        .lines()
        .nth(error.location.line - 1)
        .expect("ICE: error on non-existing line");

    let message = match error.kind {
        TokenizingErrorKind::InvalidEscape => format!(
            "invalid escape character {} at column {} on line {}",
            line.chars()
                .nth(error.location.column - 1)
                .expect("ICE: error at non-existing column"),
            error.location.line,
            error.location.column
        ),
        TokenizingErrorKind::InvalidSuffix => format!(
            "invalid suffix starting from column {} on line {}",
            error.location.column, error.location.line
        ),
        TokenizingErrorKind::UnknownToken => format!(
            "invalid token starting from column {} on line {}",
            error.location.column, error.location.line
        ),
    };

    println!("ERROR: {}", message);
    println!("{}: {}", error.location.line, line.trim());
}
