#[macro_use] extern crate prettytable;
use prettytable::format;

fn main() {
    let mut table = table!(["Value 1", "Value 2"], ["Value three", "Value four"]);
    table.set_titles(row!["Title 1", "Title 2"]);
    println!("{}─{}", '┌', '┐');
    println!("{}─{}", '├', '┤');
    println!("{}─{}", '└', '┘');
    table.set_format(
        format::FormatBuilder::new()
        .column_separator('│')
        //.column_separator('|')
        .borders('|')
        .borders('│')
        .separators( &[format::LinePosition::Top],    format::LineSeparator::new('─', '┬', '┌', '┐'))
        .separators( &[format::LinePosition::Intern], format::LineSeparator::new('─', '┼', '├', '┤'))
        .separators( &[format::LinePosition::Bottom], format::LineSeparator::new('─', '┴', '└', '┘'))
        .padding(1, 1)
        .build()
        );
    table.printstd();
}
