use pyo3::prelude::*;
use pyo3::exc;

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

        exc::RuntimeError::new(self.description().to_string())

        // match self.0 {
        //
        //     e => exc::RuntimeError::new(e.description().to_string())
        //     // RefOverflow,
        //     // RefUnderflow,
        //     // InitCrypto,
        //     // NoAesHardware,
        //     // Hashing,
        //     // InvalidCost,
        //     // InvalidCipher,
        //     // Encrypt,
        //     // Decrypt,
        //     // InvalidUri,
        //     // InvalidSuperBlk,
        //     // Corrupted,
        //     // Opened,
        //     // WrongVersion,
        //     // NoEntity,
        //     // InTrans,
        //     // NotInTrans,
        //     // NoTrans,
        //     // Uncompleted,
        //     // InUse,
        //     // NoContent,
        //     // InvalidArgument,
        //     // InvalidPath,
        //     // NotFound,
        //     // AlreadyExists,
        //     // IsRoot,
        //     // IsDir,
        //     // IsFile,
        //     // NotDir,
        //     // NotFile,
        //     // NotEmpty,
        //     // NoVersion,
        //     // ReadOnly,
        //     // CannotRead,
        //     // CannotWrite,
        //     // NotWrite,
        //     // NotFinish,
        //     // Encode(EncodeError),
        //     // Decode(DecodeError),
        //     // Var(VarError),
        //     // Io(IoError),
        // }
    }
}

impl<T> ::std::convert::Into<PyResult<T>> for Error {
    fn into(self) -> PyResult<T> {
        let pyerr: PyErr = self.into();
        Err(pyerr)
    }
}
