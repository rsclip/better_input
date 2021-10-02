extern crate crossterm;

mod inp;
mod selection;
mod spinbox;

use pyo3::prelude::*;
use inp::*;
use selection::*;
use spinbox::*;

#[pymodule]
fn better_input(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(input, m)?)?;
    m.add_function(wrap_pyfunction!(selection, m)?)?;
    m.add_function(wrap_pyfunction!(spinbox, m)?)?;
    Ok(())
}