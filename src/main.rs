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
    if pattern.chars().count() == 1 {
        log::debug!("count is 1");
        return input_line.contains(pattern);
    } else {
        input_line.chars().any(|c| c.is_alphabetic())
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
