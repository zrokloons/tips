// This file holds definition of a Tip's metadata

use serde::{Deserialize, Serialize};
use std::{fmt};


// Metadata struct
#[derive(Serialize, Deserialize)]
pub struct Metadata {

    // The subject is supposed to be a ... subject ... for the string
    pub subject: String,

    // The id field is the highest free id in database, and will uniquely
    // identify the Tip in the database.
    pub id: Option<usize>,

    // The tags list holds tags for the Tip.
    pub tags: Vec<String>,
}

// Implement PartialEq trait for Metadata, so we can compare two Metadata
// structs against each other.
impl PartialEq for Metadata {
    fn eq(&self, other: &Metadata) -> bool {
        self.subject == other.subject &&
            self.id == other.id &&
            self.tags == other.tags
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _id = match self.id {
            Some(id) => id,
            None => 0,
        };
        write!(
            f,
            "subject: {}\nid: {}\ntags: {:?}]\n",
            self.subject, _id, self.tags,
        )
    }
}
