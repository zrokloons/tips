// This file contains functionality to list a Tip or all Tips


// Entry point for list subcommand
pub fn list(matches: &clap::ArgMatches) {

    // By default the "pattern" value is set to "all". Meaning if only "list"
    // is given all tips will be presented. however if user provide a pattern
    // the "pattern" will contain this and we need to match agains it.
    match matches.value_of("pattern") {
        Some("all")   => crate::tips::Tips::summary(),
        Some(pattern) => match_pattern(pattern, matches.value_of("part")),
        None          => panic!("default pattern not set for 'pattern' ?"),
    };
}

// Search for pattern among all tips. Unless specified all components are
// searched, but if given only search that component.
fn match_pattern(pattern: &str, part: Option<&str>) {

    // Load tips & initialize empty vector that will hold potential Tip matches
    let tips = crate::tips::Tips::load();
    let mut rows = Vec::new();

    // Set component according to "part" arg
    let component = match part {
        Some("subject") => crate::query::Component::Subject,
        Some("tag")     => crate::query::Component::Tag,
        Some("data")    => crate::query::Component::Data,
        Some("all")     => crate::query::Component::All,
        _               => panic!("Part not implemented!"),
    };

    // Iterate over all tips and populate the rows vector with tip headlines
    // for any Tip that matches the pattern.
    for tip in tips.tips.iter() {
        match crate::query::search(pattern, &tip, &component) {
            Some(tip) => rows.push(tip.header_cells()),
            None      => (),
        }
    };

    // Present the result
    match rows.len() {
        0 => {
            println!("No t(ips) found using pattern: {}", pattern);
            std::process::exit(1);
        },
        _ => crate::present::present(&rows),
    }
}
