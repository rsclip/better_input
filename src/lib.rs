extern crate crossterm;

mod inp;
mod selection;

use pyo3::prelude::*;
use inp::*;
use selection::*;

#[pymodule]
fn better_input(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(input, m)?)?;
    m.add_function(wrap_pyfunction!(selection, m)?)?;
    Ok(())
}