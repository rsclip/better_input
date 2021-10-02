use pyo3::prelude::*;
use pyo3::types::*;
use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{SavePosition, RestorePosition, Hide, Show},
    event::{read, Event, KeyCode}
};

/// Allow a user to choose between multiple options
/// 
/// options     list    a list of options to choose from
/// prefix      string  text preceding the selection
/// suffix      string  text following the selection
#[pyfunction(
    prefix = "None",
    suffix = "None",
)]
#[allow(dead_code)]
pub fn selection<'a>(
    options: &'a PyList,
    prefix: Option<&'a str>,
    suffix: Option<&'a str>,
) -> PyResult<usize> {
    let mut stdout = stdout();
    let mut index: u8 = 0u8;

    let options_vec: Vec<&'a str> = options.extract()?;
    let options_len: u8 = (options_vec.len() - 1) as u8;

    match prefix {
        Some(x) => println!("{}", x),
        None => {}
    };
    
    stdout.execute(SavePosition)?;
    stdout.execute(Hide)?;

    print!("{}", build_selection_string(&options_vec, index));

    match suffix {
        Some(x) => print!("\n{}\n", x),
        None => {}
    };

    loop {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => {
                        index = decrement(index, options_len);
                    },
                    KeyCode::Down => {
                        index = increment(index, options_len);
                    },
                    KeyCode::Enter => {
                        break;
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        // build selection string and print
        stdout.queue(RestorePosition)?;
        print!("{}", build_selection_string(&options_vec, index));
        
        match suffix {
            Some(x) => print!("\n{}\n", x),
            None => {}
        };

        stdout.flush()?;
    }

    stdout.execute(Show)?;

    Ok(index as usize)
}

fn build_selection_string(options: &Vec<&str>, index: u8) -> String {
    options
        .iter()
        .enumerate()
        .map(|(i, x)| if i == index.into() {
                format!(" > {}\n", x)
            } else {
                format!("   {}\n", x)
            }
        )
        .collect()
}

fn increment(index: u8, max: u8) -> u8 {
    if index >= max {
        0u8
    } else {
        index + 1u8
    }
}

fn decrement(index: u8, max: u8) -> u8 {
    if index <= 0 {
        max
    } else {
        index - 1u8
    }
}