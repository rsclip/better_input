use pyo3::prelude::*;
use pyo3::types::*;
use std::io::{stdout, Write, Stdout};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{SavePosition, RestorePosition, MoveLeft},
    event::{read, Event, KeyCode}
};

#[pyfunction]
pub fn input(prefix: &PyUnicode, _suffix: &PyUnicode) -> String {
    let mut stdout = stdout();
    let mut chars = String::new();

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
                        chars.pop();
                        rem_last_char(&mut stdout);
                    },
                    _ => {}
                }
            },
            _ => {}
        };

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