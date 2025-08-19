use std::collections::hash_map;
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

#[warn(unused_assignments)]
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut input_iter = input_line.chars().peekable();
    let mut ret_value = false;
    let mut bracket_flag = false;

    'out: loop {
        let mut pattern_iter = pattern.chars().peekable();
        ret_value = false;
        let mut first_match = false;
        // match start with "^"
        first_match = match pattern_iter.clone().peekable().peek().unwrap() {
            '^' => true,
            _ => false,
        };

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
                '^' => {
                    if first_match {
                        let mut ret = true;
                        while pattern_iter.clone().peekable().peek().is_some() {
                            let pattern_char = pattern_iter.next().unwrap();
                            match pattern_char {
                                ' ' | 'a'..='z' | 'A'..='Z' => {
                                    //如果字符匹配过程有错误
                                    if input_iter.next().is_some_and(|c| c != pattern_char) {
                                        ret = false;
                                    }
                                }
                                '$' => {
                                    if pattern_iter.clone().peekable().peek().is_some() {
                                        ret = false;
                                    }
                                }
                                _ => ret = false,
                            }
                        }
                        ret
                    } else {
                        false
                    }
                }
                '[' => {
                    let mut reverse = false;
                    //[abc] 只要匹配一个字符就可以返回true,否则返回false
                    //[^abc] 只要匹配到一个字符，则返回false
                    if pattern_iter.peek().is_some_and(|c| c == &'^') {
                        pattern_iter.next().unwrap();
                        reverse = true;
                    }
                    let a_pattern: Vec<char> =
                        pattern_iter.clone().take_while(|c| c != &']').collect();

                    let mut has_been_true = false;
                    loop {
                        //匹配[]模式，直到消耗掉所有的input_iter内容
                        if input_iter.peek().is_none() {
                            break;
                        }
                        let input_next = input_iter.next().unwrap();
                        let mut a_pattern_iter = a_pattern.iter();
                        let ret = if reverse {
                            a_pattern_iter
                                .by_ref()
                                .inspect(|&c| println!("{c}"))
                                .all(|c| c != &input_next)
                        } else {
                            a_pattern_iter.any(|c| c == &input_next)
                        };
                        if ret {
                            println!("has been true");
                            has_been_true = true;
                        }
                    }
                    bracket_flag = true;
                    if !has_been_true {
                        println!("return false");
                        false
                    } else {
                        println!("return true");
                        true
                    }
                }
                ' ' | 'a'..='z' | 'A'..='Z' => {
                    if pattern_iter.peek().is_some_and(|c| c == &'+') {
                        pattern_iter.next().unwrap(); // consume the '+'
                        while input_iter.peek().unwrap() == &pattern_char {
                            input_iter.next().unwrap();
                        }
                        true
                    } else if input_iter.next().is_some_and(|c| c == pattern_char) {
                        true
                    } else {
                        false
                    }
                }
                '$' => {
                    if input_iter.clone().peekable().peek().is_none() {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            //结束本轮匹配
            //
            if !ret_value {
                break;
            } else {
                //如果匹配成功，且模式字符消耗完成，则退出
                if pattern_iter.clone().peekable().peek().is_none() {
                    break 'out;
                }
            }
            // 判断input 是none,则跳出循环
            if input_iter.clone().peekable().peek().is_none() {
                println!("input get end!");
                //if input end but pattern not end,return false
                if pattern_iter.clone().peekable().peek().is_some() {
                    if pattern_iter.clone().peekable().peek().unwrap() == &'$' {
                        continue;
                    }
                    //如果没有匹配[],则匹配模式耗尽后返回false
                    if !bracket_flag {
                        ret_value = false;
                    }
                }
                break 'out;
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
