// This file contains definition of Tip.

use crate::statics::{CONFIG};
use serde::{Deserialize, Serialize};
use std::{fmt, fs};


pub static TIP: TipTemplate = TipTemplate {

    // The template is used when a new is added. It's inserted into the
    // temporary file that is opened.
    template:
r##"subject:
tags:
  - notag
----- TIP BELOW THIS LINE -----
"##,

    // The separator is inserted after the template and act as a separator
    // to the tip data. The separator is not included in the stored Tip.
    separator: "----- TIP BELOW THIS LINE -----",
};

// Struct that holds the Tip template and separator fields
pub struct TipTemplate{
    pub template: &'static str,
    pub separator: &'static str,
}


// Structure that describes the Tip, including both metadata and
// tip data.
//
// The Tip is divided into two fields, where the contents of the tip
// is stored in a file referenced (by name) by the data field. The
// metadata contains all other information used by Tips.
#[derive(Serialize, Deserialize)]
pub struct Tip {

    // Metadata contains all other information needed except the
    // contents of the Tip itself. Such as id,subject, and tags.
    // See Metadata type for more info.
    pub metadata: crate::metadata::Metadata,

    // The tips data is the actual tip it self. It is stored in a
    // separate file with a name same as this field.
    pub data: uuid::Uuid,
}

impl fmt::Display for Tip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "metadata: {}\ndata: {:?}\n", self.metadata, self.data)
    }
}

impl PartialEq for Tip {
    fn eq(&self, other: &Tip) -> bool {
        self.metadata == other.metadata && self.data == other.data
    }
}

impl Tip {

    // Functions

    // Present (write to stdout) a summary of the Tip
    pub fn summary(&self) {
        let mut rows = Vec::new();
        rows.push(self.header_cells());
        crate::present::present(&rows);
    }

    // List of Cells for all metadata of Tip
    pub fn header_cells(&self) -> Vec<prettytable::Cell> {
        vec![self.id_cell(), self.subject_cell(), self.tags_cell()]
    }

    // Cell for metadata.id
    fn id_cell(&self) -> prettytable::Cell {
        let mut cell = prettytable::Cell::new(
            &format!("{}", self.metadata.id.unwrap()))
            .style_spec(&CONFIG.cell_styles.id);

        cell.align(prettytable::format::Alignment::LEFT);
        cell.set_hspan(4);

        cell
    }

    // Cell for metadata.subject
    fn subject_cell(&self) -> prettytable::Cell {
        let mut cell = prettytable::Cell::new(
            &format!("{}", self.metadata.subject))
            .style_spec(&CONFIG.cell_styles.subject);

        cell.align(prettytable::format::Alignment::LEFT);
        cell.set_hspan(4);

        cell
    }

    // Cell for metadata.tags
    fn tags_cell(&self) -> prettytable::Cell {

        // Create a new String containing all tags separated
        // by a space
        let mut tag_string = String::new();
        for tag in self.metadata.tags.iter() {
            tag_string.insert(0, ' ');
            tag_string.insert_str(0, &tag);
        }

        // Create the cell with the string created
        // and set the style, alignment
        let mut cell = prettytable::Cell::new(&tag_string)
            .style_spec(&CONFIG.cell_styles.tags);

        cell.align(prettytable::format::Alignment::RIGHT);
        cell.set_hspan(4);

        cell
    }

    // Return the contents of the data file
    pub fn get_data(&self) -> String {
        let data_file = format!("{}/{}", &CONFIG.data, self.data);
        crate::helpers::read_to_string(&data_file)
    }

    // Write the Tip structure to file, adding the contents of data
    // separated by the separator string.
    pub fn to_file(&self, outfile: &str) {

        // Serialize Tip to string
        let serialized_tip = match serde_yaml::to_string(&*self) {
            Ok(data) => data,
            Err(error) => {
                panic!("Error serialize tip {}\n{}",
                       &*self, error)
            },
        };

        // Separator string
        let separator = format!("\n{}\n", &TIP.separator);

        // Tip data
        let data_file = format!("{}/{}", &CONFIG.data, self.data);
        let tip_contents = match fs::read_to_string(&data_file) {
            Ok(data) => data,
            Err(error) => {
                panic!("Error reading file {}\n{}",
                       &data_file, error)
            },
        };

        // Insert all into a String and finally write to file
        let mut data = String::new();
        data.insert_str(0, tip_contents.as_str());
        data.insert_str(0, separator.as_str());
        data.insert_str(0, serialized_tip.as_str());
        crate::helpers::write_to_file(&outfile, &data);
    }

    // Present (print to stdout) this tip
    pub fn present(&self) {

        // Read the data contents
        let data_file = format!("{}/{}", &CONFIG.data, &self.data);
        let content = crate::helpers::read_to_string(&data_file);

        // Create a cell for the data
        let mut tip = prettytable::Cell::new(&content)
            .style_spec(&CONFIG.cell_styles.data);
        tip.align(prettytable::format::Alignment::LEFT);
        tip.set_hspan(0);

        // Present the tip
        crate::present::present_tip(self.header_cells(), tip);
    }
}
