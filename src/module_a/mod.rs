use pyo3::prelude::*;

#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[pymodule]
fn module_a(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;

    Ok(())
}

pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let mod_a = PyModule::new_bound(m.py(), "pyo3_sample.module_a")?;
    module_a(&mod_a)?;

    m.add("module_a", &mod_a)?;
    m.py().import_bound("sys")?
        .getattr("modules")?
        .set_item("pyo3_sample.module_a", mod_a)?;

    Ok(())
}
