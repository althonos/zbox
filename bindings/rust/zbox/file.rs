use pyo3::prelude::*;

#[py::class(subclass)]
pub struct File {
    token: PyToken,
}
