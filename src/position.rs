use std::char;
use std::fmt;

// A-Z + AA-ZZ
const MAX_COLUMNS: u32 = 26 + (26 * 26) - 1;
const ALPHA_OFFSET: u32 = 65;

pub struct Position {
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

        if col <= 25 {
            buf.push(char::from_u32(col + ALPHA_OFFSET).unwrap());
        } else {
            let low = col % 26;
            let high = (col - 25 - low) / 26;
            buf.push(char::from_u32(high + ALPHA_OFFSET).unwrap());
            buf.push(char::from_u32(low + ALPHA_OFFSET).unwrap());
        }

        buf
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.fmt_col(), self.row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_col() {
        let pos = Position { row: 0, col: 0 };
        assert_eq!(pos.fmt_col(), "A");

        let pos = Position { row: 0, col: 25 };
        assert_eq!(pos.fmt_col(), "Z");

        let pos = Position { row: 0, col: 26 };
        assert_eq!(pos.fmt_col(), "AA");

        let pos = Position { row: 0, col: 27 };
        assert_eq!(pos.fmt_col(), "AB");

        let pos = Position { row: 0, col: 51 };
        assert_eq!(pos.fmt_col(), "AZ");

        let pos = Position { row: 0, col: 52 };
        assert_eq!(pos.fmt_col(), "BA");

        let pos = Position {
            row: 0,
            col: MAX_COLUMNS,
        };
        assert_eq!(pos.fmt_col(), "ZZ");
    }
}
