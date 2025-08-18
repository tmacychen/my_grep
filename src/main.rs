use std::io;
use std::process;

use clap::Parser;
use log::Log;
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
    let mut pattern_iter = pattern.chars();
    let mut input_iter = input_line.chars();

    let mut ret_value = false;

    loop {
        let pattern_char = pattern_iter.next().unwrap();
        println!("pattern_char {}", pattern_char);
        ret_value = match pattern_char {
            '\\' => match pattern_iter.next().unwrap() {
                'd' => input_iter.next().is_some_and(|c| c.is_numeric()),
                'w' => input_iter
                    .next()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_'),
                _ => false,
            },
            '[' => {
                let mut ret = false;
                while !ret {
                    let next_char = pattern_iter.next().unwrap();
                    println!("next_char {}", next_char);
                    ret = match next_char {
                        'a'..='z' | 'A'..='Z' => input_line.chars().any(|c| c == next_char),
                        ']' => false,
                        _ => false,
                    };
                }
                ret
            }
            ' ' | 'a'..='z' | 'A'..='Z' => input_iter.next().is_some_and(|c| c == pattern_char),
            _ => {
                break;
            }
        };
        if ret_value || pattern_iter.clone().peekable().peek().is_none() {
            break;
        }
    }

    ret_value
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
            LOG.flush();
            println!("success exit 0!");
            process::exit(0)
        } else {
            LOG.flush();
            println!("fail exit 1!");
            process::exit(1)
        }
    }
}
