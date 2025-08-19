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
    let mut input_iter = input_line.chars();
    let mut ret_value = false;
    'out: loop {
        let mut pattern_iter = pattern.chars();
        ret_value = false;

        loop {
            let pattern_char = pattern_iter.next().unwrap();
            println!("pattern_char {}", pattern_char);
            ret_value = match pattern_char {
                '\\' => match pattern_iter.next().unwrap() {
                    'd' => {
                        if input_iter.find(|c| c.is_numeric()).is_some() {
                            true
                        } else {
                            false
                        }
                    }
                    'w' => {
                        if input_iter
                            .find(|c| c.is_ascii_alphanumeric() || c == &'_')
                            .is_some()
                        {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                '[' => {
                    let mut ret = false;
                    //[abc] 只要匹配一个字符就可以返回true,否则返回false
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
                ' ' | 'a'..='z' | 'A'..='Z' => {
                    //如果字符匹配过程有错误，立刻跳出本轮匹配
                    if input_iter.next().is_some_and(|c| c == pattern_char) {
                        true
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            };
            if pattern_iter.clone().peekable().peek().is_none() {
                if ret_value || input_iter.clone().peekable().peek().is_none() {
                    break 'out;
                } else {
                    break;
                }
            }
        }
        if input_iter.clone().peekable().peek().is_none() {
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
