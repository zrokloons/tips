// This module contains definition of the Config struct along with functionality
// to load and also store this struct to file.

use serde::{Deserialize, Serialize};


// Struct that hold Cell style configuration. This allows a user to configure
// the style of different Cells according to his/her style
#[derive(Serialize, Deserialize)]
pub struct CellStyle{

    // Style for the ID column
    pub id: String,

    // Style for the subject column
    pub subject: String,

    // Style for the tags column
    pub tags: String,

    // Style for the data output
    pub data: String,
}

// This struct represents the configuration of tips
//
// TODO replace String type for the fields below to Path type
#[derive(Serialize, Deserialize)]
pub struct Config {

    // db_file field holds the path to the database file.
    pub db_file: String,

    // tmp_file field holds a path to the file that shall be used for temporary
    // file need by Tips.
    pub tmp_file: String,

    // data field specifies a path where all data files will be placed.
    pub data: String,

    // editor field holds the absolute path for the editor that shall be
    // opened for the user.
    pub editor: String,

    // Color definitions
    pub cell_styles: CellStyle,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "db_file:            {}", self.db_file)?;
        writeln!(f, "tmp_file:           {}", self.tmp_file)?;
        writeln!(f, "editor:             {}", self.editor)?;
        writeln!(f, "Cell style id:      {}", self.cell_styles.id)?;
        writeln!(f, "Cell style subject: {}", self.cell_styles.subject)?;
        writeln!(f, "Cell style tags:    {}", self.cell_styles.tags)?;
        writeln!(f, "Cell style data:    {}", self.cell_styles.data)?;
        Ok(())
    }
}

impl Config {

    // Functions

    // This function takes the Config struct and write it to specified file
    pub fn store(&self, file_path: &String) {

        // Serialize the Config struct
        let data = match serde_yaml::to_string(&*self) {
            Ok(string) => string,
            Err(err)   => {
                panic!("Error when serializing Config struc: {}\n{}",
                       self, err);
            },
        };

        crate::helpers::write_to_file(&file_path, &data);
    }

    // Associated functions

    // Load config from ~/.tipsrc file and deserialize it into a Config
    // struct.
    pub fn load() -> Config {

        // Set path to .tipsrc first
        let tipsrc = match std::env::var("HOME") {
            Ok(home) => {
                format!("{}/.tipsrc", home)
            },
            Err(err) => {
                panic!("Error loading env HOME: {}", err);
            },
        };

        // read the contents to a string.
        let data = crate::helpers::read_to_string(&tipsrc);

        // deserialize contents into Config struct, and return this
        // struct.
        match serde_yaml::from_str(&data) {
            Ok(file) => file,
            Err(error) => {
                panic!("Error deserialize tipsrc config: {}", error);
            },
        }
    }
}
