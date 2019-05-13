// This file contains help function to present tips for the user
// via prettytabke and syntect crates.
//
// This file needs refacor and comments!

use crate::statics::{CONFIG};
use prettytable::format::{LinePosition, LineSeparator, TableFormat};
use prettytable::{Cell, Row, Table};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};


pub fn present(rows: &[Vec<Cell>]) {
    let tf = tableformat();
    let mut table = Table::new();
    table.set_format(tf);

    for row in rows {
        table.add_row(Row::new(row.to_vec()));
    }

    table.printstd();
}

pub fn present_tip(head_rows: Vec<Cell>, data: &str, data_extension: Option<String>) {
    let tf = tableformat_inter();
    let mut table = Table::new();

    // Print the Tip header unless env variable is set.
    match std::env::var("TIPS_SHOW_NOHEADER") {
        Ok(_) => (),
        Err(_) => {
            table.set_format(tf);
            table.add_row(Row::new(head_rows));
            table.printstd();
        },
    };

    // Test if stdout is a tty. If it's not an tty we shall print the raw
    // string to stdout, since a formatted string via syntect adds extra
    // characters that is most probably not wanted when stdout is not a tty.
  	if atty::is(atty::Stream::Stdout) {

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        // Get the data extention if set, owtherwise default to txt
        let de = match data_extension {
            Some(de) => de,
            None     => String::from("txt"),
        };

        // Get the syntax given the data extension, if not found default to syntax
        // for 'txt' extention
        let syntax = match ps.find_syntax_by_extension(&de) {
            Some(syntax) => syntax,
            None         => ps.find_syntax_by_extension("txt").unwrap(),
        };

        let mut h = HighlightLines::new(syntax, &ts.themes[&CONFIG.style.data.theme]);
        for line in LinesWithEndings::from(&data) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            print!("{}", escaped);
        }

    } else {

        // Just print the raw string, stdout is not a tty!
        println!("{}", data);
    }
}

fn tableformat_inter() -> TableFormat {
    let mut this = _tableformat();
    this.tableformat.separator(LinePosition::Intern, this.lineseparator);
    this.tableformat
}

fn tableformat() -> TableFormat {
    let this = _tableformat();
    this.tableformat
}

struct TableAndLine {
    tableformat: TableFormat,
    lineseparator: LineSeparator,
}

fn _tableformat() -> TableAndLine {
    let ls = LineSeparator::new('-', '-', '-', '-');
    let mut tf = TableFormat::new();
    tf.indent(0);
    tf.column_separator('|');
    tf.separator(LinePosition::Bottom, ls);
    tf.separator(LinePosition::Top, ls);
    TableAndLine { tableformat: tf, lineseparator: ls }
}
