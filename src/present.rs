// This file contains help function to present tips for the user
// via prettytabke crate.
//
// This file needs refacor and comments!

use prettytable::format::{LinePosition, LineSeparator, TableFormat};
use prettytable::{Cell, Row, Table};


pub fn present(rows: &Vec<Vec<Cell>>) {
    let tf = tableformat();
    let mut table = Table::new();
    table.set_format(tf);

    for row in rows {
        table.add_row(Row::new(row.to_vec()));
    }

    table.printstd();
}

pub fn present_tip(head_rows: Vec<Cell>, mut desc_cell: Cell) {
    let tf = tableformat_inter();
    let mut table = Table::new();
    table.set_format(tf);

    table.add_row(Row::new(head_rows));
    desc_cell.set_hspan(12);
    table.add_row(Row::new(vec![desc_cell]));

    table.printstd();
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
