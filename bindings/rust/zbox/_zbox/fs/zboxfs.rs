use std::collections::HashSet;
use std::path::Path;

use pyo3::prelude::*;

use ::file::File;
use ::file::Mode;
use ::fs::enums::ResourceType;
use ::fs::errors::FSError;
use ::fs::errors::fsexc;

#[py::class(subclass)]
pub struct ZboxFS {
    repo: ::zbox::Repo,
    token: PyToken,
}

#[py::methods]
impl ZboxFS {

    #[new]
    #[args(pwd = "\"\"", create = "true")]
    fn __new__(obj: &PyRawObject, uri: &str, pwd: &str, create: bool) -> PyResult<()> {
        match ::zbox::RepoOpener::new().create(create).open(uri, pwd) {
            Ok(repo) => obj.init(|token| ZboxFS { repo, token }),
            Err(err) => FSError::from(err).into(),
        }
    }

    #[args(overwrite = "false")]
    fn copy(&mut self, src: &str, dst: &str, overwrite: bool) -> PyResult<()> {
        if !overwrite && self.repo.is_file(dst) {
            fsexc::DestinationExists::new(dst.to_owned()).into()
        } else {
            self.repo.copy(src, dst).map_err(|err| FSError::with_path(err, src).into())
        }
    }

    fn exists(&self, path: &str) -> PyResult<bool> {
        Ok(self.repo.path_exists(path))
    }

    fn isdir(&self, path: &str) -> PyResult<bool> {
        Ok(self.repo.is_dir(path))
    }

    fn isfile(&self, path: &str) -> PyResult<bool> {
        Ok(self.repo.is_file(path))
    }

    // FIXME: wait for PyO3/pyo3#141 and replace `PyString` with `PyUnicode`
    fn getinfo(&self, path: &str, namespaces: Option<Vec<&str>>) -> PyResult<&PyDict> {

        let meta = match self.repo.metadata(path) {
            Err(err) => return FSError::with_path(err, path).into(),
            Ok(meta) => meta,
        };

        let ns = namespaces.unwrap_or(vec!["basic"]);
        let info = PyDict::new(self.token.py());
        let is_dir = self.repo.is_dir(path);

        // Basic namespace - always present
        let basic = PyDict::new(self.token.py());
        let name = path.rsplit_terminator("/").next().unwrap_or("");
        basic.set_item("name", PyString::new(self.token.py(), name));
        basic.set_item("is_dir", is_dir);
        info.set_item("basic", basic);

        // Details namespace
        if ns.contains(&"details") {
            let details = PyDict::new(self.token.py());
            let resource_type = if is_dir {ResourceType::Directory} else {ResourceType::File};
            details.set_item("size", meta.len());
            details.set_item("type", resource_type as i32);
            info.set_item("details", details);
        }

        Ok(info)
    }

    // FIXME: wait for PyO3/pyo3#141 and replace `PyString` with `PyUnicode`
    fn listdir(&self, path: &str) -> PyResult<Vec<Py<PyString>>> {
        match self.repo.read_dir(path) {
            Err(err) => FSError::with_path(err, path).into(),
            Ok(entries) => {
                let names = entries.iter().map(|ref e| e.file_name());
                let strings = names.map(|ref n| PyString::new(self.token.py(), n));
                Ok(strings.collect())
            }
        }
    }

    #[args(recreate = "false")]
    fn makedir(
        &mut self,
        path: &str,
        permissions: Option<PyObject>,
        recreate: bool,
    ) -> PyResult<()> {
        use ::zbox::Error::AlreadyExists;
        match self.repo.create_dir(path) {
            Ok(()) => Ok(()),
            Err(AlreadyExists) if recreate => Ok(()),
            Err(AlreadyExists) if !recreate => fsexc::DirectoryExists::new(path.to_owned()).into(),
            Err(err) => FSError::with_path(err, path).into(),
        }
    }

    #[args(overwrite = "false")]
    fn move_(&mut self, src: &str, dst: &str, overwrite: bool) -> PyResult<()> {
        if self.repo.is_dir(src) {
            return fsexc::FileExpected::new(src.to_owned()).into();
        }
        if self.repo.is_file(dst) && !overwrite {
            return fsexc::DestinationExists::new(dst.to_owned()).into();
        }
        self.repo.rename(src, dst).map_err(|err| FSError::with_path(err, src).into())
    }

    #[args(mode = "\"rb\"", buffering = "-1", options = "**")]
    fn openbin(
        &mut self,
        path: &str,
        mode: &str,
        buffering: isize,
        options: Option<&PyDict>,
    ) -> PyResult<Py<File>> {
        use ::zbox::Error::NotDir;

        let _mode = Mode::from(mode);
        if !_mode.create && !self.repo.path_exists(path) {
            return fsexc::ResourceNotFound::new(path.to_owned()).into()
        }

        match ::zbox::OpenOptions::new()
            .read(_mode.reading)
            .write(_mode.writing)
            .append(_mode.appending)
            .create(_mode.create)
            .create_new(_mode.exclusive)
            .truncate(_mode.truncate)
            .open(&mut self.repo, path) {
                Ok(f) => { self.token.py().init(|token| File::new(token, f, _mode)) }
                Err(NotDir) => { fsexc::ResourceNotFound::new(path.to_owned()).into() }
                Err(err) => { FSError::with_path(err, path).into() }
            }
    }

    fn remove(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_file(path)
            .map_err(|err| FSError::with_path(err, path).into())
    }

    fn removedir(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_dir(path)
            .map_err(|err| FSError::with_path(err, path).into())
    }

    fn setinfo(&self, path: &str, info: &PyDict) -> PyResult<()> {
        self.getinfo(path, None).map(|_| ())
    }
}
