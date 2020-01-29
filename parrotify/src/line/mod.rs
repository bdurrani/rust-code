// https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
mod constants;
use std::{fmt, str};
use std::collections::HashMap;
type Row = String;

#[derive(Debug)]
pub struct Line {
    items: [Row; constants::LETTER_HEIGHT],
    hashmap: HashMap<char, constants::Letter>
}

impl Line {
    /// Create a new Line
    pub fn new() -> Line {
        let arr: [Row; constants::LETTER_HEIGHT] = Default::default();
        let has = Line::build_map();
        Line { items: arr, hashmap:has }
    }

    pub fn add_letter1(&mut self, letter: &char) {
        for i in 0..constants::LETTER_HEIGHT {
            // consider using String::from_utf8_lossy()
            self.items[i].push_str(str::from_utf8(letter[i]).unwrap());
        }
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
    fn build_map() -> HashMap<char, constants::Letter> {
        let mut encoding = HashMap::new();
        encoding.insert('a', constants::ENCODED_A);
        encoding.insert('f', constants::ENCODED_F);
        encoding.insert('b', constants::ENCODED_B);
        encoding.insert('c', constants::ENCODED_C);
        encoding.insert('h', constants::ENCODED_H);
        encoding.insert('d', constants::ENCODED_D);
        encoding.insert('e', constants::ENCODED_E);
        encoding.insert('f', constants::ENCODED_F);
        encoding.insert('g', constants::ENCODED_G);
        encoding.insert('h', constants::ENCODED_H);
        encoding.insert('i', constants::ENCODED_I);
        encoding.insert('j', constants::ENCODED_J);
        encoding.insert('k', constants::ENCODED_K);
        encoding.insert('l', constants::ENCODED_L);
        encoding.insert('m', constants::ENCODED_M);
        encoding.insert('n', constants::ENCODED_N);
        encoding.insert('o', constants::ENCODED_O);
        encoding.insert('p', constants::ENCODED_P);
        encoding.insert('q', constants::ENCODED_Q);
        encoding.insert('r', constants::ENCODED_R);
        encoding.insert('s', constants::ENCODED_S);
        encoding.insert('t', constants::ENCODED_T);
        encoding.insert('u', constants::ENCODED_U);
        encoding.insert('v', constants::ENCODED_V);
        encoding.insert('w', constants::ENCODED_W);
        encoding.insert('x', constants::ENCODED_X);
        encoding.insert('y', constants::ENCODED_Y);
        encoding.insert('z', constants::ENCODED_Z);
        encoding.insert(' ', constants::ENCODED_SPACE);
        encoding.insert('!', constants::ENCODED_EXCLAMATION);
        encoding.insert('?', constants::ENCODED_QUESTION);
        encoding.insert('.', constants::ENCODED_PERIOD);
        encoding.insert('-', constants::ENCODED_DASH);
        encoding.insert('=', constants::ENCODED_EQUALS);
        encoding.insert('+', constants::ENCODED_PLUS);
        encoding.insert('1', constants::ENCODED1);
        encoding.insert('2', constants::ENCODED2);
        encoding.insert('3', constants::ENCODED3);
        encoding.insert('4', constants::ENCODED4);
        encoding.insert('5', constants::ENCODED5);
        encoding.insert('6', constants::ENCODED6);
        encoding.insert('7', constants::ENCODED7);
        encoding.insert('8', constants::ENCODED8);
        encoding.insert('9', constants::ENCODED9);
        encoding.insert('0', constants::ENCODED0);

        return encoding;
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
