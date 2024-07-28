# PyO3 Sample
In my journey to create a Python module using Rust, I found that many of the resources I found were either outdated or lacked some of the information I desired.

This is not intended to be a thourough and comprehensive sample or tutorial, rather just a demonstration of some features.
# Some goals
* creating a structured python module because none of the documentation showed how to make python reflect the submodule structure
* incorporate what i learn here in my linalg module
# Creating a Python module using maturin and PyO3
```
py -m venv env
env\Scripts\activate
pip install maturin
maturin init -b pyo3
```
Write some rust code. Your modules should take the form of either `src/<module_name>/mod.rs` or `src/<module_name>.rs` if you want the compiler to find them.
```
maturin dev -r
```
This does not cover publishing your module so that others can install them with pip, but you now have a usable Python extension module written in Rust. enjoy :)
