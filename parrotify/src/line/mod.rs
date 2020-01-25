// https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
use crate::constants;
use std::{fmt, str};
type Row = String;

#[derive(Debug)]
pub struct Line {
    items: [Row; constants::LETTER_HEIGHT],
}

impl Line {
    /// Create a new Line
    pub fn new() -> Line {
        let arr: [Row; constants::LETTER_HEIGHT] = Default::default();
        Line { items: arr }
    }

    pub fn add_letter(&mut self, letter: constants::Letter) {
        for i in 0..constants::LETTER_HEIGHT {
            // consider using String::from_utf8_lossy()
            self.items[i].push_str(str::from_utf8(letter[i]).unwrap());
        }
    }

    pub fn replace_a(&mut self, replacement: &char) {
        for i in 0..constants::LETTER_HEIGHT {
            self.items[i] = self.items[i].replace("A", &replacement.to_string()[..]);
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..constants::LETTER_HEIGHT {
            writeln!(f, "{}", self.items[i]).unwrap();
        }
        Ok(())
    }
}
