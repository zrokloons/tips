// This is the main of Tips application.
//
// This file define the clap and based on the user argument load respective
// function associated with used subcommand.
//
extern crate prettytable;
extern crate lazy_static;
extern crate clap;
extern crate serde;
extern crate serde_yaml;
extern crate regex;
extern crate chrono;
extern crate atty;
mod helpers;
mod init;
mod query;
mod add;
mod show;
mod tips;
mod tip;
mod metadata;
mod remove;
mod update;
mod config;
mod list;
mod present;
mod statics;
mod open;


use clap::{App, Arg, SubCommand, crate_version};

fn main() {
    let matches = App::new("tips")
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("add")
                .display_order(1)
                .visible_alias("a")
                .about("Add t(ip)")
                .subcommand(
                    SubCommand::with_name("-")
                        .about("Read text from stdin")
                        .arg(
                            Arg::with_name("dash")
                         )
                )
                .arg(
                    Arg::with_name("file")
                        .help("Insert contents of file")
                        .takes_value(true),
                )
        )
        .subcommand(
            SubCommand::with_name("list")
                .display_order(2)
                .visible_alias("l")
                .about("List tips")
                .arg(
                    Arg::with_name("pattern")
                        .help("List tips matching pattern in tag sections")
                        .default_value("all")
                )
                .arg(
                    Arg::with_name("part")
                        .help("Part to apply pattern on")
                        .default_value("all")
                        .possible_values(&["subject", "tag", "data", "all"]),
                )
        )
        .subcommand(
            SubCommand::with_name("show")
                .display_order(3)
                .visible_alias("s")
                .about("show a t(ips)")
                .arg(
                    Arg::with_name("id")
                        .help("t(ips) id")
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("update")
                .display_order(4)
                .visible_alias("u")
                .about("Update todo")
                .arg(
                    Arg::with_name("id")
                        .help("Update t(ips) id")
                        .takes_value(true)
                        .required(true)
            ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .display_order(5)
                .visible_alias("r")
                .about("Remove a t(ip)")
                .arg(
                    Arg::with_name("id")
                        .help("t(ips) id")
                        .takes_value(true)
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("init")
                .display_order(6)
                .about("Initialize t(ips)"),
        )
        .get_matches();

    // Only use init now
    use crate::init::{init};
    match matches.subcommand_name() {
        Some("init") => {
            init();
            std::process::exit(0);
        },
        Some(_)      => (),
        None         => (),
    };

    // Delay the use of other libs since Config will load T(ips) configuration
    // using layzy_static upon use.
    use crate::remove::{remove};
    use crate::update::{update};
    use crate::show::{show};
    use crate::list::{list};
    use crate::add::{add};

    match matches.subcommand_name() {
        Some("add")    => add(matches.subcommand_matches("add").unwrap()),
        Some("remove") => remove(matches.subcommand_matches("remove").unwrap()),
        Some("update") => update(matches.subcommand_matches("update").unwrap()),
        Some("show")   => show(matches.subcommand_matches("show").unwrap()),
        Some("list")   => list(matches.subcommand_matches("list").unwrap()),
        None           => {
            println!("Woops! No subcommand given");
            std::process::exit(1);
        },
        not_supported => {
            println!("'{}' not yet implemented!", not_supported.unwrap());
            std::process::exit(1);
        },
    };
}
