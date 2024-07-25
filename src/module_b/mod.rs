use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn sort(list: Vec<i64>) -> PyResult<Vec<i64>> {
    if list.is_empty() {
        return Err(PyValueError::new_err("list must not be empty"))
    }

    let mut copy = list.clone();
    copy.sort();

    Ok(copy)
}

#[pymodule]
fn module_b(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sort, m)?)?;

    Ok(())
}

pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let mod_b = PyModule::new_bound(m.py(), "pyo3_sample.module_b")?;
    module_b(&mod_b)?;

    m.add("module_b", &mod_b)?;
    m.py().import_bound("sys")?
        .getattr("modules")?
        .set_item("pyo3_sample.module_b", mod_b)?;

    Ok(())
}

