use std::error::Error;
use std::io::{Read, Seek, SeekFrom, Write};

use pyo3::prelude::*;
use pyo3::py::*;
use pyo3::buffer::PyBuffer;
use pyo3::class::context::*;
use pyo3::exc;

use ::file::errors::ioexc;
use ::file::mode::Mode;
use ::utils::QuickFind;
use ::utils::Tell;


macro_rules! check_open {
    ($file: expr) => {
        match $file {
            Some(ref mut f) => f,
            None => return Err(exc::ValueError::new("I/O operation on closed file.")),
        }
    };
}


macro_rules! check_readable {
    ($file: expr, $mode: expr) => {
        if !$mode.reading {
            return Err(ioexc::UnsupportedOperation::new("not readable"));
        } else {
            check_open!($file)
        }
    }
}


macro_rules! check_writable {
    ($file: expr, $mode: expr) => {
        if !$mode.writing {
            return Err(ioexc::UnsupportedOperation::new("not writable"));
        } else {
            check_open!($file)
        }
    }
}


#[class(subclass)]
pub struct File {
    file: Option<::zbox::File>,
    mode: Mode,
    token: PyToken,
}


impl File {

    pub fn new(token: PyToken, file: ::zbox::File, mode: Mode) -> Self {
        Self {
            token,
            file: Some(file),
            mode: mode,
        }
    }

    fn _readline(file: &mut ::zbox::File, buf: &mut Vec<u8>) -> PyResult<Vec<u8>> {

        let mut line = Vec::with_capacity(buf.len());
        let mut read: usize = 1;
        let mut end: usize = 1;
        let pos = file.tell()?;

        while {
            read = file.read(buf)?;
            line.last() != Some(&b'\n') && read != 0
        } {
            end = buf[..read].quickfind(b'\n').unwrap_or(read - 1);
            line.extend_from_slice(&buf[..end + 1]);
        }

        file.seek(SeekFrom::Start((pos + line.len() as u64) as u64))
            .map_err(PyErr::from)?;

        Ok(line)
    }

}

#[methods]
impl File {
    #[getter]
    fn mode(&self) -> PyResult<&str> {
        Ok(&self.mode.mode)
    }

    #[getter]
    fn closed(&self) -> PyResult<bool> {
        Ok(self.file.is_none())
    }

    fn close(&mut self) -> PyResult<()> {
        self.file = None;
        Ok(())
    }

    fn fileno(&self) -> PyResult<()> {
        ioexc::UnsupportedOperation::new("fileno").into()
    }

    fn flush(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn isatty(&mut self) -> PyResult<bool> {
        Ok(false)
    }

    #[args(size = "-1")]
    fn read(&mut self, mut size: i64) -> PyResult<Py<PyBytes>> {

        let mut data: Vec<u8>;
        let mut file = check_readable!(self.file, self.mode);

        let bytes_read = if size >= 0 {
            data = Vec::with_capacity(size as usize);
            file.take(size as u64).read_to_end(&mut data)?
        } else {
            data = Vec::with_capacity(file.metadata().map(|m| m.len()).unwrap_or(0));
            file.read_to_end(&mut data)?
        };

        Ok(PyBytes::new(self.token.py(), &data))
    }

    fn readable(&self) -> PyResult<bool> {
        Ok(self.mode.reading)
    }

    fn readinto(&mut self, dest: &PyObjectRef) -> PyResult<usize> {

        let mut raw_data: &mut [u8];
        let mut file = check_readable!(self.file, self.mode);
        let buffer = PyBuffer::get(self.token.py(), dest)?;

        let ptr = buffer
            .as_mut_slice::<u8>(self.token.py())
            .ok_or(exc::TypeError::new("object supporting the buffer API required"))?;

        // The unsafe code is safe since we checked the buffer contains writable well-aligned bytes
        unsafe { raw_data = ::std::slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, ptr.len()) }

        let bytes_read = file.read(raw_data)?;
        Ok(bytes_read)
    }

    fn readline(&mut self) -> PyResult<Py<PyBytes>> {
        let file = check_readable!(self.file, self.mode);
        let mut buf = vec![0; *::constants::io::DEFAULT_BUFFER_SIZE];
        let line = Self::_readline(file, &mut buf)?;
        Ok(PyBytes::new(self.token.py(), &line))
    }

    #[args(hint = "-1")]
    fn readlines(&mut self, hint: i64) -> PyResult<Vec<Py<PyBytes>>> {

        let file = check_readable!(self.file, self.mode);
        let mut buf = vec![0; *::constants::io::DEFAULT_BUFFER_SIZE];

        let mut total = 0;
        let mut lines = Vec::new();
        let mut line: Vec<u8> = Vec::new();

        while {
            line = Self::_readline(file, &mut buf)?;
            !line.is_empty() && total < hint as usize
        } {
            total += line.len();
            lines.push(PyBytes::new(self.token.py(), &line));
        }

        Ok(lines)
    }

    fn truncate(&mut self, size: Option<u64>) -> PyResult<u64> {
        let file = check_writable!(self.file, self.mode);

        let newsize = match size {
            Some(s) => s,
            None => file.tell().map_err(PyErr::from)?,
        };

        match file.set_len(newsize as usize) {
            Ok(_) => Ok(newsize),
            Err(err) => Err(exc::IOError::new(err.description().to_string())),
        }
    }

    fn write(&mut self, data: &PyObjectRef) -> PyResult<usize> {

        let buffer = PyBuffer::get(self.token.py(), data)?;
        let mut file = check_writable!(self.file, self.mode);
        let pos = file.tell()?;

        let ptr = buffer
            .as_slice::<u8>(self.token.py())
            .ok_or(exc::TypeError::new("object supporting the buffer API required"))?;

        // The unsafe code is actually safe since we checked beforehand the buffer
        // contains read-only well-aligned bytes
        let raw_data = unsafe {
            ::std::slice::from_raw_parts(ptr.as_ptr() as *const u8, ptr.len())
        };

        let bytes_written = file
            .write(raw_data)
            .map_err(PyErr::from)?;

        file.finish();
        file.seek(SeekFrom::Start(pos + bytes_written as u64))?;

        Ok(bytes_written)
    }

    // TODO: proper implementation
    fn writelines(&mut self, lines: Vec<&PyBytes>) -> PyResult<()> {
        for line in lines {
            self.write(line.as_ref())?;
        }
        Ok(())
    }

    fn writable(&self) -> PyResult<bool> {
        Ok(self.mode.writing)
    }

    #[args(whence = "*::constants::io::SEEK_SET")]
    fn seek(&mut self, offset: i64, whence: usize) -> PyResult<u64> {
        let file = check_open!(self.file);
        let py = self.token.py();

        // Import constants from the io module
        use ::constants::io::{SEEK_CUR, SEEK_SET, SEEK_END};

        // Turn the (offset, whence) pair into a SeekFrom instance
        let seekfrom = if SEEK_CUR == whence {
            SeekFrom::Current(offset)
        } else if whence == SEEK_SET {
            SeekFrom::Start(offset as u64)
        } else if whence == SEEK_END {
            SeekFrom::End(offset)
        } else {
            return Err(exc::ValueError::new(format!(
                "invalid whence ({}, should be {}, {} or {})",
                whence, SEEK_SET, SEEK_CUR, SEEK_END
            )));
        };

        // Seek the file
        file.seek(seekfrom).map_err(PyErr::from)
    }

    fn seekable(&self) -> PyResult<bool> {
        Ok(true)
    }

    fn tell(&mut self) -> PyResult<u64> {
        let file = check_open!(self.file);
        file.seek(SeekFrom::Current(0)).map_err(PyErr::from)
    }
}

#[proto]
impl PyIterProtocol for File {

    fn __iter__(&mut self) -> PyResult<PyObject> {
        Ok(self.into())
    }

    fn __next__(&mut self) -> PyResult<Option<Py<PyBytes>>> {
        let bytes = self.readline()?;
        if bytes.as_ref(self.token.py()).data().is_empty() {
            Ok(None)
        } else {
            Ok(Some(bytes))
        }
    }
}

#[proto]
impl<'p> PyContextProtocol<'p> for File {

    fn __enter__(&mut self) -> PyResult<PyObject> {
        Ok(self.to_object(self.token.py()))
    }

    fn __exit__(
        &mut self,
        ty: Option<&'p PyType>,
        value: Option<&'p PyObjectRef>,
        traceback: Option<&'p PyObjectRef>
    ) -> PyResult<bool> {
        self.close();
        Ok(false)
    }
}
