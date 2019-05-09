// This file contains functionality to show a Tip

use std::str::{FromStr};


// Entry point for add subcommand
pub fn show(matches: &clap::ArgMatches) {

    // Since arg "id" is required it is safe to call unwrap here
    let id_str = matches.value_of("id").unwrap();

    // Convert id to usize
    let id: usize = match usize::from_str(id_str) {
        Ok(id) => id,
        Err(error) => {
            panic!("Error converting str id {} to usize\n{}",
                   id_str, error)
        },
    };

    // Load tips
    let tips = crate::tips::Tips::load();

    // If we get an index for the tip present it. Otherwise panic.
    if let Some(index) = tips.get_tip_index(&id) {
        let tip = tips.tips.get(index).unwrap();
        tip.present();
    } else {
        panic!("No tip in DB for ID: {}", id)
    }
}
