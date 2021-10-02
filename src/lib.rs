extern crate crossterm;

mod inp;

use pyo3::prelude::*;
use inp::*;

#[pymodule]
fn better_input(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(input, m)?)?;
    Ok(())
}