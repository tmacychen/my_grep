use std::io;
use std::process;

use clap::Parser;
use tklog::debug;
use tklog::{error, info, Format, LEVEL, LOG};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arg {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short = 'E')]
    pattern: Option<String>,
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let match_escape_char = |c: &str| match c {
        "d" => input_line.chars().any(|c| c.is_numeric()),
        "w" => input_line
            .chars()
            .any(|c| c.is_ascii_alphanumeric() || c == '_'),
        _ => false,
    };

    let match_brackets = |slice: &str| -> bool {
        let mut reverse: bool = false;
        if slice.chars().peekable().peek().unwrap() == &'^' {
            reverse = true;
        }
        for c in slice.chars() {
            if c.is_ascii_alphabetic() {
                if reverse == false {
                    if input_line.chars().any(|ch| ch == c) {
                        return true;
                    }
                } else {
                    if input_line.chars().all(|ch| ch != c) {
                        return true;
                    }
                }
            }
        }
        false
    };

    if pattern.chars().count() == 1 {
        log::debug!("count is 1");
        return input_line.contains(pattern);
    } else {
        let (first, last) = pattern.split_at(1);
        match first {
            "\\" => match_escape_char(last),

            "[" => match_brackets(last),
            _ => false,
        }
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // eprintln!("Logs from your program will appear here!");
    LOG.set_level(LEVEL::Debug)
        .set_format(Format::LevelFlag | Format::ShortFileName)
        .set_formatter("{level} | {file}:{message} \n");

    let args = Arg::parse();

    if let Some(pattern) = args.pattern {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        // Uncomment this block to pass the first stage
        log::debug!("{pattern},{input_line}");
        if match_pattern(&input_line, &pattern) {
            println!("success exit 0!");
            process::exit(0)
        } else {
            println!("fail exit 1!");
            process::exit(1)
        }
    }
}
