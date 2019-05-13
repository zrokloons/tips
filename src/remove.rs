// This file contains functionality to remove a Tip

use crate::statics::{CONFIG};
use std::{io};
use std::io::{Write};


// Struct that represent a Tip to be removed
struct RemoveTip {

    // Index of the tip in tips
    index: usize,

    // uuid of the tip's data
    data: String,
}

// Entry point for add subcommand
pub fn remove(matches: &clap::ArgMatches) {

    // Since arg "id" is required it is safe to call unwrap here
    let id = matches.value_of("id").unwrap();
    let mut tips = crate::tips::Tips::load();

    // Remove the Tip if matched found and confirmed
    if let Some(remove_tip) = confirm_removal(&tips, id) {
        crate::helpers::remove_file(&remove_tip.data);
        tips.tips.remove(remove_tip.index);
        tips.store();
    } else { // else exit with a message
        println!("No tip with id: {} found", id);
        std::process::exit(1);
    }

}

// Iterate over all tips and if one is found matching id confirm removal of
// this Tip.
fn confirm_removal(tips: &crate::tips::Tips, id: &str) -> Option<RemoveTip> {

    // Iterate over all Tips
    for (idx, tip) in tips.tips.iter().enumerate() {
        if tip.metadata.id.unwrap().to_string() == id {

            // print out the found Tip's summary, so user can verify the id
            // is correct before permanently remove the Tip
            tip.summary();

            // Ask user to confirm the removal of Tip, (flush the question)
            print!("Sure you want to delete tip ? [y/n]: ");
            match io::stdout().flush() {
                Ok(_) => (),
                Err(error) => {
                    panic!("Error when trying to flush to stdout\n{}",
                           error)
                },
            };

            let mut answer = String::new();
            match io::stdin().read_line(&mut answer) {
                Ok(_) => (),
                Err(error) => {
                    panic!("Error when trying to read line from stdin\n{}",
                           error)
                },
            };

            // Match answer, accept 'y' and 'Y' as a confirmation to remove Tip
            // This is done by returning a RemoveTip struct with fields set to
            // Tip.
            match answer.trim() {
                "y" | "Y" => {
                    return Some(RemoveTip {
                        index: idx,
                        data: format!("{}/{}", &CONFIG.data, tip.data),
                    })
                },
                _ => {
                    println!("Aborted by user");
                    std::process::exit(1);
                }
            }
        }
    }

    None
}
