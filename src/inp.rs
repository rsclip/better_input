use pyo3::prelude::*;
use std::io::{stdout, Write, Stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{SavePosition, MoveLeft},
    event::{read, Event, KeyCode}
};

/// Reads and returns a string input from a user
/// 
/// prefix      Text preceding user input
/// suffix      Text following user input
/// mask        Hide user input with a character
/// allowed     Only allow these characters
#[pyfunction(
    prefix = "\"\"", 
    suffix = "\"\"",
    mask = "None",
    allowed = "\"\"",
)]
pub fn input<'a>(
    prefix: &'a str, 
    suffix: &'a str,
    mask: Option<&'a str>,
    allowed: &'a str,
) -> PyResult<String> {
    let suffix: String = suffix.to_string();
    let suffix_len: u16 = suffix.len() as u16;
    let allowed: Option<Vec<char>> = parse_allowed(allowed);
    let mut stdout = stdout();
    let mut chars = String::new();
    let mut rem_suffix_char = false;

    print!("{}{}", prefix, suffix);
    stdout.queue(MoveLeft(suffix_len)).unwrap();

    // save pos
    stdout.execute(SavePosition).unwrap();

    loop {
        // read() is blocking until keyboard event
        match read().unwrap() {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Enter => {break;},
                    KeyCode::Char(char) => {
                        match allowed {
                            Some(ref allowed_chars) => {
                                if !(allowed_chars.iter().any(|x| x == &char)) {continue;}
                            },
                            None => {}
                        };
                        chars.push(char);
                        match mask {
                            Some(m) => print!("{}", m),
                            None => print!("{}", char),
                        };
                    },
                    KeyCode::Backspace => {
                        if chars.len() > 0 {
                            chars.pop();
                            rem_last_char(&mut stdout);
                            rem_suffix_char = true;
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        };

        print!("{}", suffix);
        if rem_suffix_char {
            rem_suffix_char = false;
            print!(" ");
            stdout.queue(MoveLeft(suffix_len + 1)).unwrap();
        } else {
            stdout.queue(MoveLeft(suffix_len)).unwrap();    
        }    

        stdout.flush().unwrap();
    };

    print!("\n");
    Ok(chars)
}

fn rem_last_char(stdout: &mut Stdout) {
    stdout.queue(MoveLeft(1u16)).unwrap();
    print!(" ");
    stdout.queue(MoveLeft(1u16)).unwrap();
}

fn parse_allowed(allowed: &str) -> Option<Vec<char>> {
    match allowed.len() {
        0 => None,
        _ => {
            Some(allowed.chars().collect())
        }
    }
}