// This file contains functionality to search pattern in a Tip


// Enum defining the components of a Tip
pub enum Component {

    // The subject component of a Tip
    Subject,

    // The Tip's data component
    Data,

    // The Tip's tags component
    Tag,

    // This represent all of the components
    All,
}

// The function takes a pattern constructs a Regex and search the
// provided tip's component for a match. If match is found return Tip
pub fn search<'a, 'b>(
    pattern: &'a str,
    tip: &'b crate::tip::Tip,
    component: &Component) -> Option<&'b crate::tip::Tip> {

    // Construct regex
    let regex = match regex::Regex::new(pattern) {
        Ok(regex) => regex,
        Err(error) => {
            panic!("Unable to construct Regex with pattern {}\n{}",
                   pattern, error)
        },
    };

    // Find matches if any, and return result
    match find(&regex, tip, &component) {
        Some(hit) => Some(hit),
        None      => None,
    }
}

// Find, or at least try to find regex in Tip's component
fn find<'a, 'b, 'c>(
    regex: &'a regex::Regex,
    tip: &'b crate::tip::Tip,
    component: &'c Component) -> Option<&'b crate::tip::Tip> {

    match component {

        // Search the Subject component
        Component::Subject => {
            match regex.find(&tip.metadata.subject) {
                Some(_) => Some(tip),
                None    => None,
            }
        },

        // Search the Tip's data component
        Component::Data => {
            match regex.find(&tip.get_data()) {
                Some(_) => Some(tip),
                None    => None,
            }
        },

        // Search the Tip's tags component
        Component::Tag => {
            for tag in tip.metadata.tags.iter() {
                if let Some(_match) = regex.find(&format!("{:?}", tag)) {
                    return Some(tip);
                }
            }
            None
        },

        // Search all components (or as long we need until a hit)
        Component::All => {
            if let Some(m) = find(regex, tip, &Component::Subject) {
                Some(m)
            }
            else if let Some(m) = find(regex, tip, &Component::Data) {
                Some(m)
            }
            else if let Some(m) = find(regex, tip, &Component::Tag) {
                Some(m)
            }
            else {
                None
            }
        },
    }
}
