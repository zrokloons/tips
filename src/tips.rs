// Tips describe the complete database of tip(s).

use crate::statics::{CONFIG};
use serde::{Deserialize, Serialize};
use std::fmt;


// Tips struct only contains a list of Tip. All tip(s) information goes into
// a Tip. However operation on all tips are handled as methods of tips.
#[derive(Serialize, Deserialize)]
pub struct Tips {

    // List of tip structs that holds all tip information
    pub tips: Vec<crate::tip::Tip>,
}

impl fmt::Display for Tips {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tip in &self.tips {
            write!(f, "{}", tip)?;
        }
        Ok(())
    }
}


impl Tips {

    // Functions

    // This function replaces the DB file on disk with contents of it self
    pub fn store(&self) {
        let serialized = match serde_yaml::to_string(&*self) {
            Ok(string) => string,
            Err(error) => {
                panic!("Error serialize tips {}\n{}",
                       &*self, error)
            },
        };

        crate::helpers::write_to_file(&CONFIG.db_file, &serialized);
    }

    // Return the index of tip matching given ID
    pub fn get_tip_index(&self, id: &usize) -> Option<usize> {
        for (index, tip) in self.tips.iter().enumerate() {
            if &tip.metadata.id.unwrap() == id {
                return Some(index)
            }
        }
        None
    }

    // Associated functions

    // This function presents a summary of the tips struct to the user
    pub fn summary() {
        let tips: Tips = Tips::load();
        let mut rows = Vec::new();

        for tip in tips.tips.iter() {
            rows.push(tip.header_cells())
        };

        crate::present::present(&rows);
    }

    // Load the current DB into a Tips struct, and then returns it to caller
    pub fn load() -> Tips {
        let contents = crate::helpers::read_to_string(&CONFIG.db_file);
        let tips: Tips = match serde_yaml::from_str(&contents) {
            Ok(tips) => tips,
            Err(error) => {
                panic!("Error deserialize tips {}\n{}",
                       &contents, error)
            },
        };

        tips
    }

    // Function to get the next available Tip ID
    pub fn next_id() -> Option<usize> {
        let tips: Tips = Tips::load();

        let num_tips = tips.tips.len();
        let last_tip = match tips.tips.get(num_tips - 1) {
            Some(tip) => tip,
            None => {
                panic!("Error unable to get next id from tips vector")
            },
        };

        Some(last_tip.metadata.id.unwrap() + 1)
    }

    // Add a Tip to the database
    pub fn add(tip: crate::tip::Tip) {
        let mut tips = Tips::load();
        tips.tips.push(tip);
        tips.store();
    }
}
