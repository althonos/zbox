#![feature(proc_macro, specialization, const_fn)]
#![allow(unused_imports)]

#[macro_use]
extern crate pyo3;
#[macro_use]
extern crate lazy_static;
extern crate zbox;

mod constants;
mod file;
mod fs;
mod repo;
mod utils;

use ::pyo3::prelude::*;
use ::pyo3::py::*;

#[modinit(_zbox)]
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
    m.add_class::<fs::ZboxFS>()?;

    Ok(())
}
