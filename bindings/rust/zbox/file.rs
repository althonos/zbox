use std::io::{Read, Seek, SeekFrom, Write};

use pyo3::prelude::*;
use pyo3::buffer::PyBuffer;
use pyo3::exc;

use error::Error;
use utils::QuickFind;

macro_rules! check_open {
    ($file: expr) => {
        match $file {
            Some(ref mut f) => f,
            None => return Err(exc::ValueError::new("I/O operation on closed file.")),
        }
    };
}

#[py::class(subclass)]
pub struct File {
    file: Option<::zbox::File>,
    mode: String,
    token: PyToken,
}

impl File {
    pub fn new<S>(token: PyToken, file: ::zbox::File, mode: S) -> Self
    where
        S: Into<Option<String>>,
    {
        Self {
            token,
            file: Some(file),
            mode: mode.into().unwrap_or(String::from("r")),
        }
    }
}

#[py::methods]
impl File {
    #[getter]
    fn mode(&self) -> PyResult<&str> {
        Ok(&self.mode)
    }

    #[getter]
    fn closed(&self) -> PyResult<bool> {
        Ok(self.file.is_none())
    }

    fn close(&mut self) -> PyResult<()> {
        if let Some(ref mut f) = self.file {
            f.finish();
        }

        self.file = None;
        Ok(())
    }

    fn flush(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn isatty(&mut self) -> PyResult<bool> {
        Ok(false)
    }

    #[args(size = "-1")]
    fn read(&mut self, mut size: isize) -> PyResult<Py<PyBytes>> {
        let file = check_open!(self.file);
        let mut data: Vec<u8>;

        if size < 0 {
            data = Vec::with_capacity(file.metadata().len());
            file.read_to_end(&mut data);
        } else {
            data = Vec::with_capacity(size as usize);
            file.read_exact(&mut data);
        }

        Ok(PyBytes::new(self.token.py(), &data))
    }

    fn readable(&self) -> PyResult<bool> {
        Ok(self.mode.contains(|c| c == 'r' || c == '+'))
    }

    fn readinto(&mut self, dest: &PyObjectRef) -> PyResult<usize> {
        let buffer = PyBuffer::get(self.token.py(), dest)?;
        let file = check_open!(self.file);
        let mut raw_data: &mut [u8];

        if let Some(b) = buffer.as_mut_slice::<u8>(self.token.py()) {
            // The unsafe code is actually safe since we checked beforehand the buffer
            // contains writable well-aligned bytes
            unsafe { raw_data = ::std::slice::from_raw_parts_mut(b.as_ptr() as *mut u8, b.len()) }
            file.read(raw_data).map_err(PyErr::from)
        } else {
            Err(exc::TypeError::new(
                "object supporting the buffer API required",
            ))
        }
    }

    // FIXME!
    fn readline(&mut self) -> PyResult<Py<PyBytes>> {
        let py = self.token.py();
        let file = check_open!(self.file);

        let size: usize = py.import("io")?
            .get("DEFAULT_BUFFER_SIZE")?
            .to_object(py)
            .extract(py)?;

        let mut buf: Vec<u8> = vec![0; size];
        let mut line = Vec::with_capacity(size);
        let mut read: usize = 1;

        while line.last() != Some(&b'\n') && read != 0 {
            read = file.read(&mut buf).map_err(PyErr::from)?;
            if let Some(pos) = buf[..read].quickfind(b'\n') {
                line.extend_from_slice(&buf[..pos + 1]);
                file.seek(SeekFrom::Current(-(read as i64) + (pos as i64) + 1))
                    .map_err(PyErr::from)?;
            } else {
                line.extend_from_slice(&buf[..read]);
            }
        }

        return Ok(PyBytes::new(py, &line));
    }

    // #[args(hint = "-1")]
    // fn readlines(&mut self, hint: isize) -> PyResult<PyObject> {
    //     let py = self.token.py();
    //     let io = py.import("io")?;
    //     let RawIOBase = io.get("RawIOBase")?.to_object(py);
    //
    //     RawIOBase.call_method1(
    //         py,
    //         "readline",
    //         (self.to_object(py), hint)
    //     )
    // }

    fn truncate(&mut self, size: Option<usize>) -> PyResult<usize> {
        let file = check_open!(self.file);
        file.finish();
        let newsize = size.unwrap_or_else(|| file.metadata().len());

        match file.set_len(newsize) {
            Ok(_) => Ok(newsize),
            Err(err) => Error::from(err).into(),
        }
    }

    fn write(&mut self, data: &PyObjectRef) -> PyResult<usize> {
        let buffer = PyBuffer::get(self.token.py(), data)?;
        let file = check_open!(self.file);
        let raw_data: &[u8];

        if let Some(s) = buffer.as_slice::<u8>(self.token.py()) {
            // The unsafe code is actually safe since we checked beforehand the buffer
            // contains read-only well-aligned bytes
            unsafe { raw_data = ::std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len()) }
            file.write(raw_data).map_err(PyErr::from)
        } else {
            Err(exc::TypeError::new(
                "object supporting the buffer API required",
            ))
        }
    }

    fn writelines(&mut self, lines: Vec<&PyBytes>) -> PyResult<()> {
        for line in lines {
            self.write(line.as_ref())?;
        }
        Ok(())
    }

    fn writable(&self) -> PyResult<bool> {
        Ok(self.mode
            .contains(|c| c == 'w' || c == 'a' || c == '+' || c == 'x'))
    }

    fn seek(&mut self, offset: i64, whence: Option<usize>) -> PyResult<u64> {
        let file = check_open!(self.file);
        let py = self.token.py();

        // Import constants from the io module
        let io = py.import("io")?;
        let seek_set: usize = io.get("SEEK_SET")?.to_object(py).extract(py)?;
        let seek_cur: usize = io.get("SEEK_CUR")?.to_object(py).extract(py)?;
        let seek_end: usize = io.get("SEEK_END")?.to_object(py).extract(py)?;

        // Turn the (offset, whence) pair into a SeekFrom instance
        let seekfrom = match whence.unwrap_or(seek_set) {
            seek_cur => SeekFrom::Current(offset),
            seek_set => SeekFrom::Start(offset as u64),
            seek_end => SeekFrom::End(offset),

            // Unknown whence
            unknown => {
                return Err(exc::ValueError::new(format!(
                    "invalid whence ({}, should be {}, {} or {})",
                    unknown, seek_cur, seek_set, seek_end
                )));
            }
        };

        // Seek the file
        file.seek(seekfrom).map_err(PyErr::from)
    }

    fn seekable(&self) -> PyResult<bool> {
        Ok(false)
    }

    fn tell(&mut self) -> PyResult<u64> {
        let file = check_open!(self.file);
        file.seek(SeekFrom::Current(0)).map_err(PyErr::from)
    }
}
