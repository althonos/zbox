// use pyo3::prelude::*;

pub enum ResourceType {
    Unknown,
    Directory,
    File,
    Character,
    BlockSpecialFile,
    Fifo,
    Socket,
    Symlink,
}

// impl ToPyObject for ResourceType {
//     fn to_object(&self, py: Python) -> PyObject {
//
//         let fs = py.import("fs").unwrap();
//         let resource_type = fs.get("ResourceType").unwrap().to_object(py);
//
//         let name = match self {
//             ResourceType::Unknown => "unknown",
//             ResourceType::Directory => "directory",
//             ResourceType::File => "file",
//             ResourceType::Character => "character",
//             ResourceType::BlockSpecialFile => "block_special_file",
//             ResourceType::Fifo => "fifo",
//             ResourceType::Socket => "socket",
//             ResourceType::Symlink => "symlink",
//         };
//
//         resource_type.getattr(py, name).unwrap()
//     }
// }
