# PyO3 Sample
In my journey to create a Python module using Rust, I found that many of the resources I found were either outdated or lacked some of the information I desired. This is not intended to be a thourough and comprehensive sample or tutorial, rather just a demonstration of some features.
# Some goals
* creating a structured python module because none of the documentation showed how to make python reflect the submodule structure
# Building
```
py -m venv env
env\Scripts\activate
pip install maturin
maturin init -b pyo3
maturin dev -r
```
