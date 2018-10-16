/// Module for work with files which use in test
/// When you need create or remove different files for unittesting or integratio tests
/// simple use structs wich implemented in this file

use std::panic::catch_unwind;
use std::fs::File as SourceFile;
use std::fs::create_dir_all;
use std::path::Path;
use std::env::current_dir;

#[derive(Debug)]
pub struct File {
    name: String,
    dir_name: String,
    extension: Option<String>,
    data: Vec<u8>,
    path: String,
}

/// Implementation File methods
/// create -> create file with choosen params that was craeted by builder
impl File {

    fn get_file_path(&self) -> String {
        // Method should return concated file path
        match &self.extension {
            Some(extension) => format!("{}/{}/{}.{}", self.path, self.dir_name, self.name, extension),
            None => format!("{}/{}/{}", self.path, self.dir_name, self.name)
        }
    }

    fn create_tmp_dir(&self) {
        // Method should create tmp dir
        // If error raise panic!
        let path_to_dir = format!("{}/{}", self.path, self.dir_name);
        let path = create_dir_all(&path_to_dir);
        match path {
            Ok(_) => trace!("Directory {} was created", path_to_dir),
            Err(_err) => panic!("Error create directory {}", path_to_dir),
        };
    }

    fn create(self) -> Self {
        // Method should create file with income params
        self.create_tmp_dir();
        let file_path = self.get_file_path();
        let file = SourceFile::create(file_path);
        match file {
            Ok(data) => println!("{:?}", data),
            Err(err) => eprintln!("Error create file {:?}", err),
        };
        self
    }

}

/// Builder for File struct
/// Use builder for create new files with default
/// or none default values
/// pub struct FileBuilder {
///    name: default,
///    dir_name: "tmp",
///    extension: None,
///    data: [b''],
///    path: "path/to/root",
///}
pub struct FileBuilder {
    _name: String,
    _dir_name: String,
    _extension: Option<String>,
    _data: Vec<u8>,
    _path: String,
}

impl Default for FileBuilder {
    fn default() -> FileBuilder {
        let path = match current_dir() {
            Ok(res) => res.display().to_string(),
            Err(_) => "/".to_string(),
        };
        FileBuilder {
            _name: "default".to_string(),
            _dir_name: "tmp".to_string(),
            _extension: None,
            _data: vec![1],
            _path: path,
        }
    }
}

/// Implementation of builder
/// `new` -> create new instrance of FileBulder
/// `create_tmp_dir` -> create temp dirrectory with chosen name
/// `file_name` -> add file name for create
/// `extension` -> add file extension
/// `build` -> create File and return instance
impl FileBuilder {

    pub fn new() -> Self {
        Self{..Default::default()}
    }

    fn create_tmp_dir<S: Into<String>>(mut self, name: S) -> Self {
        // Method create temp dir with chosen name
        self._dir_name = name.into();
        self
    }

    fn file_name<S: Into<String>>(mut self, name: S) -> Self {
        // Method should add name for file
        self._name = name.into();
        self
    }

    fn extension<S: Into<String>>(mut self, extension: S) -> Self {
        // Method should create extension for file
        self._extension = Some(extension.into());
        self
    }

    fn build(&self) -> File {
        // Method should build data and return File instance
        File {
            name: self._name.clone(),
            dir_name: self._dir_name.clone(),
            extension: self._extension.clone(),
            // TODO add data method
            data: vec![1],
            path: self._path.clone(),
        }
    }
}


#[cfg(test)]
pub mod file_integration {
    use super::*;

    #[test]
    fn test_should_create_empty_file_in_temp_directory() {
        let current_dir = current_dir().unwrap();
        let created_file = 
            FileBuilder::new()
            .create_tmp_dir("temp")
            .file_name("test")
            .extension("txt")
            .build()
            .create();
        println!("{:?}", created_file);
        assert_eq!(created_file.name, "test");
        assert_eq!(created_file.extension.unwrap(), "txt");

        assert_eq!(created_file.path, current_dir.display().to_string());
    }
}