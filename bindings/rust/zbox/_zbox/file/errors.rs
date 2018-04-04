use pyo3::prelude::*;

pub mod ioexc {
    import_exception!(io, UnsupportedOperation);
}
