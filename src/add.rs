// This file contains functionality to add a Tip to Tips

use crate::tip::TIP;
use crate::statics::CONFIG;
use std::io::Read;
use std::{io, process};


// Enum that describe the source input of the new Tip
enum Input {

    // File means that the contents of the Tip comes from a file. For example
    // 'tips add /tmp/note.txt
    File(String),

    // Stdin means that the Tip contents comes via a pipe. The Tip template
    // will in this case contain the output of the pipe.
    Stdin(String),

    // Means that the Tip contents will need to be manually entered by the user.
    Interactive,
}

// Struct holding input used.
struct Source {

    // This field hold the Input type (see above)
    origin: Input,
}


// Entry point for add subcommand
pub fn add(matches: &clap::ArgMatches) {

    // Test if input comes via stdin
    if matches.is_present("-") {

        // Read data from stdin to String
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut lines = String::new();

        while let Ok(n_bytes) = stdin.read_to_string(&mut lines) {
            if n_bytes == 0 { break }
        }

        // Call function add_tip with input Stdin and String containing
        // the data just read.
        add_tip(
            Source {
                origin: Input::Stdin(lines)
            }
        );

    } else {

        // Test if new tip was added as file, and if so call the add_tip with
        // input File and the path to the file.
        //
        // Otherwise the add must be interactive.
        match matches.value_of("file") {
            Some(path) => add_tip(
                Source {
                    origin: Input::File(path.to_string())
                }
            ),
            None => add_tip(
                Source {
                    origin: Input::Interactive
                }
            ),
        };

    }
}

// Help function to add a new Tip
fn add_tip(source: Source) {

    // Write the template to file, and then open it
    write_template(source);
    crate::open::editor(&CONFIG.tmp_file);

    // Read the contents of tmp file and remove the tmp file
    let contents = crate::helpers::read_to_string(&CONFIG.tmp_file);
    crate::helpers::remove_file(&CONFIG.tmp_file);

    // if contents do not differ from template then abort
    if &contents[0..TIP.template.len()] == TIP.template {
        println!("Aborting. Contents no different from template.");
        process::exit(1)
    }

    // Create a new Tip from the read data and add it to tips
    let mut tip: crate::tip::Tip = create(contents);
    tip.metadata.id = crate::tips::Tips::next_id();
    crate::tips::Tips::add(tip);
}

// Function to write down the template to the temporary file specified
// in config.
fn write_template(source: Source) {

    // Get the contents to add in data section of template.
    let data = get_tip_data(source);

    // Write the template and the data to temporary file.
    let mut template = TIP.template.to_string();
    template.insert_str(TIP.template.len(), &data);
    crate::helpers::write_to_file(&CONFIG.tmp_file, &template);
}

// This function returns that data for the different sources. That means that
// if the new tip comes from a:
//      file, then return the contents of the file.
//      stdin, then return the provided data
//      interactive, then return a "replace me"
fn get_tip_data(s: Source) -> String {
    match s.origin {
        Input::File(file)   => crate::helpers::read_to_string(&file),
        Input::Stdin(stdin) => stdin,
        Input::Interactive  => "<replace me>".to_string(),
    }
}

// Create the Tip and the data file associated with this tip
fn create(contents: String) -> crate::tip::Tip {

    // Split the contents based on separator string, create a
    // create metadata struct and write down the data section to a file
    // before creating the Tip.
    let byte_index = match contents.find(TIP.separator) {
        Some(byte_index) => byte_index,
        None    => {
            panic!("Unable to find split pattern!")
        },
    };

    let (_metadata, _data) = contents.split_at(byte_index);
    let mut __data = _data.to_string();
    __data.replace_range(..TIP.separator.len(), "");
    __data.remove(0); // remove newline left from above


    // First deserialize the Metadata from String, then add the datetime to the
    // created field in Metadata.
    let mut md: crate::metadata::Metadata = match serde_yaml::from_str(&_metadata) {
        Ok(md) => md,
        Err(err) => {
            panic!("Error deserialize metadata {}\n{}",
                   &_metadata, err)
        },
    };

    md.created = Some(chrono::offset::Local::now());

    // Generate a new uudi for this Tip's data.
    let uuid = uuid::Uuid::new_v4();

    crate::helpers::write_to_file(
        &format!("{}/{}", &CONFIG.data, uuid),
        &__data);

    crate::tip::Tip { metadata: md, data: uuid }
}
