extern crate clap;
extern crate csv;

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Field<'a> {
    Text(&'a str),
    Float(f64),
    Int(i64),
}

// A-Z + AA-ZZ
const MAX_COLUMNS: u32 = 26 + (26 * 26);
const DIGITS: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
const ALPHA_OFFSET: u32 = 65;

fn parse_field(raw: &str) -> Field {
    if raw.len() == 0 {
        return Field::Text(raw);
    }

    if DIGITS.contains(&raw.bytes().next().unwrap()) {
        if raw.contains(".") {
            match raw.parse::<f64>() {
                Ok(num) => Field::Float(num),
                Err(_) => Field::Text(raw),
            }
        } else {
            match raw.parse::<i64>() {
                Ok(num) => Field::Int(num),
                Err(_) => Field::Text(raw),
            }
        }
    } else {
        Field::Text(raw)
    }
}

struct Position {
    pub col: u32,
    pub row: u32,
}

impl Position {
    pub fn new() -> Position {
        Position { col: 0, row: 0 }
    }

    pub fn incr_col(&mut self) {
        assert!(self.col < MAX_COLUMNS);
        self.col += 1
    }

    pub fn incr_row(&mut self) {
        self.row += 1
    }

    pub fn reset_col(&mut self) {
        self.col = 0
    }

    pub fn fmt_col(&self) -> String {
        let col = self.col;
        assert!(col <= MAX_COLUMNS);

        let mut buf = String::with_capacity(2);

        if col <= 27 {
            buf.push(std::char::from_u32(col + ALPHA_OFFSET).unwrap());
        } else {
            let low = col % 26;
            let high = (col - low) / 26;
            buf.push(std::char::from_u32(high + ALPHA_OFFSET).unwrap());
            buf.push(std::char::from_u32(low + ALPHA_OFFSET).unwrap());
        }

        buf
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.fmt_col(), self.row)
    }
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

fn run() -> Result<(), Box<StdError>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut pos = Position::new();

    for result in reader.records() {
        let record = result?;
        for val in record.iter() {
            match parse_field(val) {
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
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
