// This file contains functionality for initialize Tips

use std::io::{Write};
use std::{collections, io, path, env};


// Create a static HashMap containing the paths needed to setup Tips
lazy_static::lazy_static! {
	static ref TIPS: collections::HashMap<&'static str, String> = {
        let mut tips = collections::HashMap::new();
        tips.insert("tipsrc", format!("{}/.tipsrc", env::var("HOME").unwrap()));
        tips.insert("tips", format!("{}/.tips", env::var("HOME").unwrap()));
        tips.insert("data", format!("{}/data", &tips["tips"]));
        tips.insert("db_file", format!("{}/db.yaml", &tips["tips"]));
        tips.insert("tmp_file", format!("{}/tmp_file.yaml", &tips["tips"]));
        tips.insert("editor", "/usr/bin/vim".to_string());
        tips
    };
}

// This function will look for the ~/.tipsrc file. If not found the needed
// files and paths will be created. If it is found a validation of config
// and file structure will be performed.
pub fn init() {
    if path::Path::new(&TIPS.get("tipsrc").unwrap()).exists() {
        verify_existing()
    } else {
        create()
    };
}

// Verify that existing ~/.tiprc is valid and that the file structure
// exists.
//
// TODO this need to be implemented
fn verify_existing() {
    println!("TODO: init verify existing");
}

// Create needed file structure, tipsrc and database
fn create() {

    // Let user know what files and directories will be created
    // and let user answer before creating anything
    println!("Following directories and files will be created:");
    println!("\tfile      {}", TIPS.get("tipsrc").unwrap());
    println!("\tdirectory {}", TIPS.get("tips").unwrap());
    println!("\tdirectory {}", TIPS.get("data").unwrap());
    print!("Go ahead and create them ? [y/n]: ");

    match io::stdout().flush() {
        Ok(_) => (),
        Err(err) => {
            panic!("Error when trying to flush to stdout\n{}",
                   err)
        },
    };

    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => (),
        Err(err) => {
            panic!("Error when trying to read line from stdin\n{}",
                   err)
        },
    };

    // Only proceed if user accepted, otherwise abort
    match answer.trim() {
        "y" | "Y" => (),
        _ => {
            println!("Aborted by user");
            std::process::exit(1);
        }
    };

    create_directory();
    create_tipsrc();
    create_database();
}

// Create all directories needed for Tips
fn create_directory() {
    crate::helpers::create_dir_all(TIPS.get("tips").unwrap());
    crate::helpers::create_dir_all(TIPS.get("data").unwrap());
}

// Create a Config struct and store it. This becomes ~/.tipsrc
fn create_tipsrc() {
    let tipsrc = crate::config::Config {
        db_file:     TIPS.get("db_file").unwrap().to_string(),
        tmp_file:    TIPS.get("tmp_file").unwrap().to_string(),
        data:        TIPS.get("data").unwrap().to_string(),
        editor:      TIPS.get("editor").unwrap().to_string(),

        style: crate::config::Style {

            // The cell styles are intentionally set to an empty string,
            // it's up to the user to change this after his/her style
            table: crate::config::TableStyle {
                id: "".to_string(),
                subject: "".to_string(),
                tags: "".to_string(),
            },

            // Let's expect that everyone runs Solarized dark theme... I do
            data: crate::config::DataStyle {
                theme: "Solarized (dark)".to_string(),
            },
        },
    };

    tipsrc.store(TIPS.get("tipsrc").unwrap());
}

// Create a welcome tip, and by doing that we initialize the database
fn create_database() {

    // generate an uuid and write the data contents to file
    let uuid = uuid::Uuid::new_v4();
    introduction_tip_data(&uuid);

    // Instantiate a Tips struct with a 'welcome tip'
    let tips = crate::tips::Tips {
        tips: vec![
            crate::tip::Tip {
                metadata: crate::metadata::Metadata {
                    subject: "My first tip".to_string(),
                    id: Some(1),
                    tags: Some(vec!["tip".to_string()]),
                    created: None,
                    last_updated: None,
                    data_extension: Some(String::from("txt")),
                },
                data: uuid,
            }
        ]
    };

    // store will create the database file
    tips.store();
}

// write the welcome tip data
fn introduction_tip_data(uuid: &uuid::Uuid) {
    let introduction = r##"
Welcome to T(ips)

    Colored output
    --------------

    Below applies for Tip summary output, that means when listing Tips and also
    the header when showing the data contents of a tip.

	Prettytable supports setting a style on a Cell. Tips configuration ~/.tipsrc
    contain an empty style. By changing that string you can get the style you
    prefer.

    Search for "Style spec syntax" for a description on how to set it.
    https://docs.rs/prettytable-rs/0.8.0/prettytable/struct.Cell.html#method.with_style
"##.to_string();

    crate::helpers::write_to_file(
        &format!("{}/{}", TIPS.get("data").unwrap(), uuid),
        &introduction);
}
