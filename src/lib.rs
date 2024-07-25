use pyo3::prelude::*;

mod module_a;
mod module_b;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_sample(m: &Bound<'_, PyModule>) -> PyResult<()> {
    
    // root level functions
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    
    // sub modules
    module_a::init(m)?;
    module_b::init(m)?;

    Ok(())
}
