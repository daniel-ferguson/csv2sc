#[macro_use]
extern crate clap;
extern crate csv;

mod field;
mod position;

use std::error::Error as StdError;
use std::io;
use std::io::prelude::*;

use field::Field;
use position::Position;

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    use clap::{App, Arg};
    App::new("csv2sc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Converts CSV or TSV input into sc spreadsheet format")
        .arg(
            Arg::with_name("tsv")
                .short("t")
                .long("tsv")
                .takes_value(false)
                .help("Use tabs rather than commas as field delimiter"),
        )
}

macro_rules! fmt_string {
    () => {
        r#"leftstring {} = {:?}"#
    };
}

macro_rules! fmt_num {
    () => {
        "let {} = {}"
    };
}

fn run(tsv: bool) -> Result<(), Box<StdError>> {
    let mut reader = csv::ReaderBuilder::new();
    if tsv {
        reader.delimiter(b'\t');
    }

    let mut reader = reader.from_reader(io::stdin());

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut pos = Position::new();

    for result in reader.records() {
        let record = result?;
        for val in record.iter() {
            match Field::from(val) {
                Field::Float(n) => writeln!(stdout, fmt_num!(), pos, n)?,
                Field::Int(n) => writeln!(stdout, fmt_num!(), pos, n)?,
                Field::Text(t) => writeln!(stdout, fmt_string!(), pos, t)?,
            }
            pos.incr_col();
        }
        pos.incr_row();
        pos.reset_col();
    }

    Ok(())
}

fn main() {
    let matches = app().get_matches();

    if let Err(e) = run(matches.is_present("tsv")) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
