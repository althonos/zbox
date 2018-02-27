#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
extern crate zbox;

mod repo;
mod file;

use pyo3::prelude::*;

#[py::class(subclass)]
struct File {
    token: PyToken,
}

#[py::modinit(zbox)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    // let authors_re = regex::Regex::new(r"(.*) <(.*)>").unwrap();
    // if let Some(captures) = authors_re.captures(env!("CARGO_PKG_AUTHORS")) {
    //     m.add("__author__", captures.get(1).unwrap().as_str())?;
    //     m.add("__author_email__", captures.get(2).unwrap().as_str())?;
    // } else {
    //     m.add("__author__", py.None())?;
    //     m.add("__author_email__", py.None())?;
    // }
    // m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    zbox::init_env();

    m.add_class::<repo::Repo>()?;
    m.add_class::<file::File>()?;

    Ok(())
}
