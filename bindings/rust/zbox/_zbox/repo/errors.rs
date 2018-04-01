use pyo3::prelude::*;
use pyo3::exc;

// #[cfg(Py_3)] use ::pyo3::exc::FileNotFoundError;
// #[cfg(Py_3)] use ::pyo3::exc::FileExistsError;
// #[cfg(Py_3)] use ::pyo3::exc::NotADirectoryError;
//
// #[cfg(not(Py_3))] use ::pyo3::exc::{OSError as FileNotFoundError};
// #[cfg(not(Py_3))] use ::pyo3::exc::{OSError as FileExistsError};
// #[cfg(not(Py_3))] use ::pyo3::exc::{OSError as NotADirectoryError};


#[derive(Debug)]
pub struct Error(::zbox::Error);


impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.0.fmt(f)
    }
}


impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        self.0.cause()
    }
}


impl ::std::convert::From<::zbox::Error> for Error {
    fn from(err: ::zbox::Error) -> Self {
        Error(err)
    }
}


impl ::std::convert::Into<PyErr> for Error {

    fn into(self) -> PyErr {

        use std::error::Error;
        use zbox::Error::*;

        match self.0 {

            // RefOverflow,
            // RefUnderflow,
            // InitCrypto,
            // NoAesHardware,
            // Hashing,
            // InvalidCost,
            // InvalidCipher,
            // Encrypt,
            // Decrypt,
            // err @ InvalidUri => exc::ValueError::new(err.description().to_string()),
            // InvalidSuperBlk,
            // Corrupted,
            // Opened,
            // WrongVersion,
            // NoEntity,
            // InTrans,
            // NotInTrans,
            // NoTrans,
            // Uncompleted,
            // InUse,
            // NoContent,
            // InvalidArgument,
            // err @ InvalidPath => exc::ValueError::new(err.description().to_string()),
            // err @ NotFound => FileNotFoundError::new(err.description().to_string()),
            // err @ AlreadyExists => FileExistsError::new(err.description().to_string()),
            // IsRoot,
            // IsDir,
            // IsFile,
            // err @ NotDir => NotADirectoryError::new(err.description().to_string()),
            // NotFile,
            // NotEmpty,
            // NoVersion,
            // ReadOnly,
            // CannotRead,
            // CannotWrite,
            // NotWrite,
            // NotFinish,
            // Encode(err) => exc::UnicodeDecodeError::new(err.description().to_string()),
            // Decode(DecodeError),
            // Var(VarError),
            // Io(IoError),

            err => exc::RuntimeError::new(err.description().to_string()),
        }
    }
}

impl<T> ::std::convert::Into<PyResult<T>> for Error {
    fn into(self) -> PyResult<T> {
        let pyerr: PyErr = self.into();
        Err(pyerr)
    }
}
