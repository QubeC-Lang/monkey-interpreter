use crate::Tokenizer;
use std::io::{self};

const PROMPT: &str = ">> ";

pub fn start_repl(mut reader: impl io::BufRead, mut writer: impl io::Write) {
    let mut input = String::new();
    loop {
        print!("{}", PROMPT);
        writer.flush().expect("Failed to flush stdout.");

        input.clear();
        match reader.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed_input = input.trim();
                let tokenizer = Tokenizer::new(trimmed_input);
                for token in tokenizer {
                    writeln!(writer, "{:?}", token).expect("Failed to write to output.");
                }
            }
            Err(error) => {
                writeln!(writer, "Error reading input: {}", error).expect("Failed to write to output.");
                break;
            }
        }
    }
}
