use std::collections::HashSet;
use std::path::Path;

use pyo3::prelude::*;

use ::file::File;
use ::fs::enums::ResourceType;
use ::fs::errors::Error;

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
            Err(err) => Error::from(err).into(),
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

    fn getinfo(&self, path: &str, namespaces: Option<Vec<&str>>) -> PyResult<&PyDict> {

        let meta = match self.repo.metadata(path) {
            Err(err) => return Error::from(err).into(),
            Ok(meta) => meta,
        };

        let ns = namespaces.unwrap_or(vec!["basic"]);
        let info = PyDict::new(self.token.py());
        let is_dir = self.repo.is_dir(path);

        // Basic namespace - always present
        let basic = PyDict::new(self.token.py());
        basic.set_item("name", path.rsplit_terminator("/").next().unwrap_or(""));
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

    fn listdir(&self, path: &str) -> PyResult<Vec<String>> {
        match self.repo.read_dir(path) {
            Err(err) => Error::from(err).into(),
            Ok(entries) => Ok(entries.iter().map(|ref e| e.file_name().into()).collect()),
        }
    }

    fn makedir(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .create_dir(path)
            .map_err(|err| Error::from(err).into())
    }

    #[args(mode = "\"rb\"", buffering = "-1", options = "**")]
    fn openbin(
        &mut self,
        path: &str,
        mode: &str,
        buffering: isize,
        options: Option<&PyDict>,
    ) -> PyResult<Py<File>> {
        match ::zbox::OpenOptions::new()
            .read(mode.contains(|c| c == '+' || c == 'r'))
            .write(mode.contains(|c| c == '+' || c == 'a' || c == 'w' || c == 'x'))
            .append(mode.contains(|c| c == 'a'))
            .create(mode.contains(|c| c == 'a' || c == 'w' || c == 'x'))
            .create_new(mode.contains(|c| c == 'x'))
            .open(&mut self.repo, path) {
                Ok(file) => self.token.py().init(|token| File::new(token, file, mode.to_string())),
                Err(err) => Error::from(err).into(),
            }
    }

    fn remove(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_file(path)
            .map_err(|err| Error::from(err).into())
    }

    fn removedir(&mut self, path: &str) -> PyResult<()> {
        self.repo
            .remove_dir(path)
            .map_err(|err| Error::from(err).into())
    }

    fn setinfo(&self, path: &str, info: &PyDict) -> PyResult<()> {
        self.getinfo(path, None).map(|_| ())
    }
}
