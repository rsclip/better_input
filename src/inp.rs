use pyo3::prelude::*;
use pyo3::types::*;
use std::io::{stdout, Write, Stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{SavePosition, MoveLeft},
    event::{read, Event, KeyCode}
};

#[pyfunction]
pub fn input(prefix: &PyUnicode, suffix: &PyUnicode) -> String {
    let suffix: String = suffix.to_string();
    let suffix_len: u16 = suffix.len() as u16;
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
                        chars.push(char);
                        print!("{}", char);
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
    chars
}

fn rem_last_char(stdout: &mut Stdout) {
    stdout.queue(MoveLeft(1u16)).unwrap();
    print!(" ");
    stdout.queue(MoveLeft(1u16)).unwrap();
}