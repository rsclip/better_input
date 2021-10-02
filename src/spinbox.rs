use pyo3::prelude::*;
use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{SavePosition, RestorePosition, Hide, Show},
    event::{read, Event, KeyCode}
};

/// Create a spinbox returning an integer value
/// 
/// min     int     minimum integer value
/// max     int     maximum integer value
/// step    int     step between values
/// prefix  string  text preceding spinbox
/// suffix  string  text following spinbox
/// default int     default starting integer value
#[pyfunction(
    step = "1",
    prefix = "None",
    suffix = "\" ⇅\"",
    default = "0",
)]
pub fn spinbox<'a>(
    min: i64,
    max: i64,
    step: i64,
    prefix: Option<&'a str>,
    suffix: Option<&'a str>,
    default: i64,
) -> PyResult<i64> {
    if min > max {
        return Err(pyo3::exceptions::PyException::new_err(format!("Argument min ({}) is greater than max ({})", min, max)));
    }

    let mut stdout = stdout();
    let mut val = default;

    // max_val_space = maximum space taken up by the value
    let max_val_space = get_max_space(min, max, &suffix);

    match prefix {
        Some(x) => print!("{}", x),
        None => {},
    };

    stdout.execute(SavePosition)?;
    stdout.execute(Hide)?;

    print!("{}", build_spinbox_string(val, max_val_space, &suffix));
    stdout.flush()?;

    loop {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Enter => {break;},
                    KeyCode::Up => {
                        val = if (val + step) > max {max} else {val + step};
                    },
                    KeyCode::Down => {
                        val = if (val - step) < min {min} else {val - step};
                    },
                    _ => {},
                }
            },
            _ => {}
        };

        stdout.queue(RestorePosition)?;
        print!("{}", build_spinbox_string(val, max_val_space, &suffix));

        stdout.flush()?;
    };

    print!("\n");

    stdout.execute(Show)?;
    Ok(val)
}

fn get_max_space(min: i64, max: i64, suffix: &Option<&str>) -> usize {
    let min = if min < 0 {min * -1} else {min};
    let max = if max < 0 {max * -1} else {max};
    let largest: f64 = if min > max {min as f64} else {max as f64};

    let val_space = int_space(largest);

    match suffix {
        Some(x) => {val_space + x.len()},
        None => {val_space}
    }
}

/// Get the num of spaces for a positive f64 value
fn int_space(val: f64) -> usize {
    if val == 0f64 {
        // if its 0, it may return 0 digits which is
        // technically true but it displays as 1 digit.
        1
    } else {
        (val.log10() + 1f64).floor() as usize
    }
}

fn build_spinbox_string(val: i64, max_val_space: usize, suffix: &Option<&str>) -> String {
    let space: usize = int_space(val as f64);
    let spacechar_space = " ".repeat(max_val_space - space + 2);
    let suffix = match suffix {
        Some(x) => x,
        None => ""
    };

    format!(
        "{}{}{} ",
        spacechar_space,
        val,
        suffix,
    )
}