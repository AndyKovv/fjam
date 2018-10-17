/// Module for work with files which use in test
/// When you need create or remove different files for unittesting or integratio tests
/// simple use structs wich implemented in this file

use std::fs::{File, remove_dir_all};
use std::io::prelude::*;
use std::fs::create_dir_all;
use std::env::current_dir;

#[derive(Debug)]
pub struct MockFile {
    name: String,
    dir_name: String,
    extension: Option<String>,
    data: Vec<u8>,
    path: String,
}

/// Implementation MockFile methods
/// create -> create file with choosen params that was craeted by builder
impl MockFile {

    pub fn get_file_path(&self) -> String {
        // Method should return concated file path
        match &self.extension {
            Some(extension) => format!("{}/{}/{}.{}", self.path, self.dir_name, self.name, extension),
            None => format!("{}/{}/{}", self.path, self.dir_name, self.name)
        }
    }

    pub fn get_abs_path(&self) -> String {
        // Method should get absolut path for directory
        format!("{}/{}", self.path, self.dir_name)
    }

    pub fn create_tmp_dir(&self) {
        // Method should create tmp dir
        // If error raise panic!
        let path_to_dir = format!("{}/{}", self.path, self.dir_name);
        let path = create_dir_all(&path_to_dir);
        match path {
            Ok(_) => trace!("Directory {} was created", path_to_dir),
            Err(_err) => panic!("Error create directory {}", path_to_dir),
        };
    }

    pub fn remove_with_dir(&self) -> &Self {
        // Method should remove files with its dir
        let path = self.get_abs_path();
        match remove_dir_all(&path) {
            Ok(_) => info!("Directory removed successully {}", path),
            Err(err) => {
                error!(
                    "Directory removed failure {}, err: {:?}", path, err
                )}, 
        };
        self
    }

    pub fn create(self) -> Self {
        // Method should create file with income params
        self.create_tmp_dir();
        let file_path = self.get_file_path();
        let file = File::create(file_path);
        match file {
            Ok(mut file) => {
                file.write_all(&self.data).unwrap();
                println!("Wrote data is {:?}", &self.data);
            },
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
            _data: vec![],
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

    pub fn create_tmp_dir<S: Into<String>>(mut self, name: S) -> Self {
        // Method create temp dir with chosen name
        self._dir_name = name.into();
        self
    }

    pub fn file_name<S: Into<String>>(mut self, name: S) -> Self {
        // Method should add name for file
        self._name = name.into();
        self
    }

    pub fn extension<S: Into<String>>(mut self, extension: S) -> Self {
        // Method should create extension for file
        self._extension = Some(extension.into());
        self
    }

    pub fn add_data(mut self, data:  &[u8]) -> Self {
        // Method should add data to bytes vector
        self._data.append(&mut data.to_vec());
        self
    }

    pub fn build(&self) -> MockFile {
        // Method should build data and return File instance
        MockFile {
            name: self._name.clone(),
            dir_name: self._dir_name.clone(),
            extension: self._extension.clone(),
            // TODO add data method
            data: self._data.clone(),
            path: self._path.clone(),
        }
    }
}


#[cfg(test)]
pub mod file_integration_tests {
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

    #[test]
    fn test_should_add_data_to_created_file() {
        let expected_byte_string = b"some bytes data";
        let created_file = 
            FileBuilder::new()
            .file_name("file_with_data")
            .add_data(expected_byte_string)
            .build()
            .create();
        let file_abs_path = created_file.get_file_path();
        let mut file = File::open(file_abs_path).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, String::from_utf8(expected_byte_string.to_vec()).unwrap()); 
    }

    #[test]
    fn test_should_remove_directory_with_files_after_remove_with_dir_call() {
        let created_file = 
            FileBuilder::new()
            .file_name("file_with_data")
            .build()
            .create();

        let exist_file = match File::open(created_file.get_file_path()) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert_eq!(exist_file, true);
        created_file.remove_with_dir();
        let non_exist_file = match File::open(created_file.get_file_path()) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert_eq!(non_exist_file, false);
    }
}