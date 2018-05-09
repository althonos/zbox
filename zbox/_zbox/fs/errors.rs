use pyo3::prelude::*;
use pyo3::exc;


pub mod fsexc {
    import_exception!(fs.errors, DestinationExists);
    import_exception!(fs.errors, DirectoryExists);
    import_exception!(fs.errors, DirectoryExpected);
    import_exception!(fs.errors, DirectoryNotEmpty);
    import_exception!(fs.errors, FileExpected);
    import_exception!(fs.errors, FileExists);
    import_exception!(fs.errors, ResourceNotFound);
    import_exception!(fs.errors, ResourceReadOnly);
    import_exception!(fs.errors, RemoveRootError);
}


#[derive(Debug)]
pub struct FSError(::zbox::Error, Option<String>);

impl FSError {
    pub fn with_path<S: Into<String>>(err: ::zbox::Error, path: S) -> Self {
        FSError(err, Some(path.into()))
    }
}

impl ::std::fmt::Display for FSError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl ::std::error::Error for FSError {
    fn description(&self) -> &str {
        self.0.description()
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        self.0.cause()
    }
}

impl ::std::convert::From<::zbox::Error> for FSError {
    fn from(err: ::zbox::Error) -> Self {
        FSError(err, None)
    }
}

impl<T> ::std::convert::Into<PyResult<T>> for FSError {
    fn into(self) -> PyResult<T> {
        let pyerr: PyErr = self.into();
        Err(pyerr)
    }
}


impl ::std::convert::Into<PyErr> for FSError {
    fn into(self) -> PyErr {

        use std::error::Error;
        use zbox::Error::*;

        let _path = if let Some(path) = self.1 {
            path
        } else {
            self.0.description().to_string()
        };

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
            err @ InvalidUri => exc::ValueError::new(err.description().to_string()),
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
            err @ InvalidPath => exc::ValueError::new(_path),

            NotFound => fsexc::ResourceNotFound::new(_path),
            err @ AlreadyExists => fsexc::FileExists::new(_path),

            // `IsRoot` should be used only when trying to remove root
            // or creating a file or directory as root
            IsRoot => fsexc::RemoveRootError::new(_path),

            IsDir => fsexc::FileExpected::new(_path),
            IsFile => fsexc::DirectoryExpected::new(_path),
            NotDir => fsexc::DirectoryExpected::new(_path),
            NotFile => fsexc::FileExpected::new(_path),
            NotEmpty => fsexc::DirectoryNotEmpty::new(_path),
            // NoVersion,
            ReadOnly => fsexc::ResourceReadOnly::new(_path),
            // CannotRead,
            // CannotWrite,
            // NotWrite,
            // NotFinish,
            Encode(err) => exc::UnicodeDecodeError::new(err.description().to_string()),
            // Decode(DecodeError),
            // Var(VarError),
            // Io(IoError),

            err => exc::RuntimeError::new(err.description().to_string()),
        }
    }
}
