// This file contains functionality to list a Tip or all Tips


// Entry point for list subcommand
pub fn list(matches: &clap::ArgMatches) {

    // List all tips or if pattern vas given list those matching
    if let Some(pattern) = matches.value_of("pattern") {
         match_pattern(pattern, matches.value_of("source"));
    } else {
        crate::tips::Tips::summary();
    }
}

// Search for pattern among all tips. Unless specified all components are
// searched, but if given only search that component.
fn match_pattern(pattern: &str, part: Option<&str>) {

    // Load tips & initialize empty vector that will hold potential Tip matches
    let tips = crate::tips::Tips::load();
    let mut rows = Vec::new();

    // Set component according to "part" arg
    let component = match part {
        Some("*")       => crate::query::Component::All,
        Some("subject") => crate::query::Component::Subject,
        Some("tag")     => crate::query::Component::Tag,
        Some("data")    => crate::query::Component::Data,
        _               => panic!("Part not implemented!"),
    };

    // Iterate over all tips and populate the rows vector with tip headlines
    // for any Tip that matches the pattern.
    for tip in tips.tips.iter() {
        if let Some(tip) = crate::query::search(pattern, &tip, &component) {
            rows.push(tip.header_cells())
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
