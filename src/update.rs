// This file contains functionality to update a Tip

use crate::statics::CONFIG;
use crate::tip::TIP;
use std::str::{FromStr};


// Entry point for update subcommand
pub fn update(matches: &clap::ArgMatches) {

    // arg id is mandatory, so unwrap is safe. Then convert id to usize
    let id = match usize::from_str(matches.value_of("id").unwrap()) {
        Ok(id) => id,
        Err(error) => {
            panic!("Error convert str to usize\n{}", error)
        },
    };

    // Possible metadata updates require mutable tips and tip initializing
    let mut tips = crate::tips::Tips::load();
    let mut tip = get_tip_with_id(&id, &mut tips);

    // Write tip to file then open it with configured editor
    tip.to_file(&CONFIG.tmp_file);
    crate::open::editor(&CONFIG.tmp_file);

    // Read updated file, then remove tmp file
    let contents = crate::helpers::read_to_string(&CONFIG.tmp_file);
    crate::helpers::remove_file(&CONFIG.tmp_file);

    // Separate metadata and data
    let container = extrace_metadata_data(&contents);

    // Update data and metadata & then store tips
    update_data(&tip, &container.data);
    update_metadata(&mut tip, &container.metadata);
    tips.store();
}

// Get Tip given ID
fn get_tip_with_id<'a>(id: &usize,
               tips: &'a mut crate::tips::Tips) -> &'a mut crate::tip::Tip {

    // Locate index of Tip given ID or panic since no Tip with ID was found
    if let Some(index) = tips.get_tip_index(&id) {

        // Return mut Tip reference or panic.
        return match tips.tips.get_mut(index) {
            Some(tip) => tip,
            None => {
                panic!("Error unable to get mut reference of tip with index {}",
                       index)
            },
        };
    } else {
        panic!("Error id {} not found in DB", id)
    }
}

// Write Tip data to file if it differ
fn update_data(tip: &crate::tip::Tip, data: &String) {

    // Compare data in updated Tip with data stored and only if it differ
    // write the new data to file, replacing old data.
    if data != &tip.get_data() {
        crate::helpers::write_to_file(
            &format!("{}/{}", &CONFIG.data, &tip.data),
            &data);
    }
}

// Update the Tip's metadata if it differ. This is done by creating a temporary
// Tip using the metadata received and comparing it against original Tip
fn update_metadata(tip: &mut crate::tip::Tip, metadata: &String) {

    // Create a temporary Tip struct from updated metadata
    let tmp_tip: crate::tip::Tip = match serde_yaml::from_str(metadata) {
        Ok(tmp_tip) => tmp_tip,
        Err(error) => {
            panic!("Error deserialize metadata {}\n{}",
                   metadata, error)
        },
    };

    // Compare Tip's metadata against the temporary, and if differ
    // set Tip's metadata subject/tags
    if tip != &tmp_tip {
        tip.metadata.subject = tmp_tip.metadata.subject;
        tip.metadata.tags = tmp_tip.metadata.tags;
    }
}

// Structure to to hold metadata and data from updated file
struct MetadataAndData {
    metadata: String,
    data: String,
}

// Extract the metadata and data by splitting the contents by the separator,
// then initialize a MetadataAndData struct that is returned.
fn extrace_metadata_data(contents: &String) -> MetadataAndData {
    let byte_index = match contents.find(TIP.separator) {
        Some(bi) => bi,
        None     => panic!("Unable to find split pattern!"),
    };

    let (metadata, _data) = contents.split_at(byte_index);
    let mut data = _data.to_string();

    // The contents sent in to this function contains both metadata, data but
    // also the Tip separator. The separator need to be removed from the data
    // part, so remove it from data part.
    data.replace_range(..TIP.separator.len(), "");
    data.remove(0); // remove newline left from above

    MetadataAndData {
        metadata: metadata.to_string(),
        data: data,
    }
}
