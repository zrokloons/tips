use crate::statics::CONFIG;
use std::io::{Write};
use std::{fs};


// Function to remove the tmp file used by tips for update or adding
// a new tip.
//
// Will panic if removal of file was unsuccessful.
pub fn remove_file(file: &str) {
    let result = std::fs::remove_file(file);
    match result {
        Ok(_)      => (),
        Err(error) => {
            panic!("Unable to remove: {}\n{}",
                   &CONFIG.tmp_file, error);
        },
    };
}

// Function to create a file based on input path
//
// Will panic if error
pub fn create_file(path: &str) -> fs::File {
    match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error creating file {}\n{}",
                   path, err);
        },
    }
}

// Function to create a directory and all parents (if needed)
//
// Will panic if error
pub fn create_dir_all(path: &str) {
    match fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(error) => {
            panic!("Error when crating directory {}\n{}",
                   &path, error)
        },
    };
}

// Function that create a file and then write provided data into it
//
// Will panic if error
pub fn write_to_file(path: &str, data: &str) {
    let mut file = create_file(path);
    match file.write_all(data.as_bytes()) {
        Ok(_) => (),
        Err(error) => {
            panic!("Error writing to file: {:?}\n{}",
                   file, error)
        },
    };
}


pub fn read_to_string(path: &str) -> String {
    match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(error) => {
            panic!("Error reading file: {}\n{}",
                   path, error);
        },
    }
}
