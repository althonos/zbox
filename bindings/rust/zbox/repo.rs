use std::error::Error;
use std::time::UNIX_EPOCH;

use pyo3::prelude::*;
use pyo3::exc;

use zbox;

#[py::class(subclass)]
pub struct Repo {
    repo: zbox::Repo,
    token: PyToken,
}

impl Repo {
    // FIXME: no unwrap if possible !
    fn dict_from_metadata(&self, metadata: &zbox::Metadata) -> &PyDict {
        let metadict = PyDict::new(self.token.py());
        metadict.set_item("is_dir", metadata.is_dir()).unwrap_or(());
        metadict
            .set_item("is_file", metadata.is_file())
            .unwrap_or(());
        metadict.set_item("len", metadata.len()).unwrap_or(());
        metadict
            .set_item("curr_version", metadata.curr_version())
            .unwrap_or(());

        if let Ok(ctime) = metadata.created().duration_since(UNIX_EPOCH) {
            metadict.set_item("created", ctime.as_secs()).unwrap_or(());
        }

        if let Ok(mtime) = metadata.modified().duration_since(UNIX_EPOCH) {
            metadict.set_item("modified", mtime.as_secs()).unwrap_or(());
        }

        metadict
    }

    // FIXME: no unwrap if possible !
    fn dict_from_direntry(&self, entry: &zbox::DirEntry) -> &PyDict {
        let entrydict = PyDict::new(self.token.py());
        entrydict
            .set_item("metadata", self.dict_from_metadata(&entry.metadata()))
            .unwrap_or(());
        // FIXME: entrydict.set_item("file_type", entry.file_type());
        entrydict
            .set_item("file_name", entry.file_name())
            .unwrap_or(());

        if let Some(path) = entry.path().to_str() {
            entrydict.set_item("path", path).unwrap_or(());
        }

        entrydict
    }
}

#[py::methods]
impl Repo {
    // FIXME: allow any object instead of only &str as Path

    #[new]
    #[args(create = "true")]
    fn __new__(obj: &PyRawObject, uri: &str, pwd: &str, create: bool) -> PyResult<()> {
        match zbox::RepoOpener::new().create(create).open(uri, pwd) {
            Ok(repo) => obj.init(|token| Repo { repo, token }),
            Err(err) => exc::RuntimeError::new(err.description().to_string()).into(),
        }
    }

    #[classmethod]
    fn exists(_cls: &PyType, uri: &str) -> PyResult<bool> {
        zbox::Repo::exists(uri).map_err(|e| exc::RuntimeError::new(e.description().to_string()))
    }

    fn path_exists(&self, path: &str) -> PyResult<bool> {
        Ok(self.repo.path_exists(path))
    }

    fn is_file(&self, path: &str) -> PyResult<bool> {
        Ok(self.repo.is_file(path))
    }

    fn is_dir(&self, path: &str) -> PyResult<bool> {
        Ok(self.repo.is_dir(path))
    }

    // fn create_file(&mut self, path: &str) -> PyResult<File> {
    // }

    // fn open_file(&mut self, path: &str) -> PyResult<File> {
    // }

    fn create_dir(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .create_dir(path)
            .map_err(|err| exc::RuntimeError::new(err.description().to_string()))
    }

    fn create_dir_all(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .create_dir_all(path)
            .map_err(|err| exc::RuntimeError::new(err.description().to_string()))
    }

    fn read_dir(&self, path: &str) -> PyResult<Vec<&PyDict>> {
        match self.repo.read_dir(path) {
            Err(err) => Err(exc::RuntimeError::new(err.description().to_string())),
            Ok(entries) => Ok(entries
                .iter()
                .map(|ref e| self.dict_from_direntry(e))
                .collect()),
        }
    }

    fn metadata(&self, path: &str) -> PyResult<&PyDict> {
        match self.repo.metadata(path) {
            Err(err) => Err(exc::RuntimeError::new(err.description().to_string())),
            Ok(ref metadata) => Ok(self.dict_from_metadata(metadata)),
        }
    }

    // fn history(&self, path: &str)

    fn copy(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.repo
            .copy(from, to)
            .map_err(|e| exc::RuntimeError::new(e.description().to_string()))
    }

    fn remove_file(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_file(path)
            .map_err(|e| exc::RuntimeError::new(e.description().to_string()))
    }

    fn remove_dir(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_dir(path)
            .map_err(|e| exc::RuntimeError::new(e.description().to_string()))
    }

    fn remove_dir_all(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_dir_all(path)
            .map_err(|e| exc::RuntimeError::new(e.description().to_string()))
    }

    fn rename(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.repo
            .rename(from, to)
            .map_err(|e| exc::RuntimeError::new(e.description().to_string()))
    }
}
