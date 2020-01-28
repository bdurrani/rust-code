use std::collections::HashMap;
pub const LETTER_WIDTH: usize = 9;
pub const LETTER_HEIGHT: usize = 5;
pub type Letter = [&'static [u8; LETTER_WIDTH]; LETTER_HEIGHT];

pub fn build_map() -> HashMap<char, [&'static [u8; LETTER_WIDTH]; LETTER_HEIGHT]> {
    let mut encoding = HashMap::new();
    encoding.insert('a', ENCODED_A);
    encoding.insert('b', ENCODED_B);
    encoding.insert('c', ENCODED_C);
    encoding.insert('d', ENCODED_C);

    return encoding;
}

pub const ENCODED_A: [&'static [u8; LETTER_WIDTH]; 5] = [
    b"AABBBBBAA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABAAAAABA",
];

pub const ENCODED_B: [&'static [u8; LETTER_WIDTH]; 5] = [
    b"ABBBBBBAA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBAA",
];

pub const ENCODED_C: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABAAAAAAA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
];

pub const ENCODED_D: [&'static [u8; 9]; 5] = [
    b"ABBBBBBAA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABBBBBBAA",
];

pub const ENCODED_E: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
];

pub const ENCODED_F: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABAAAAAAA",
];

pub const ENCODED_G: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABAAAABBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
];

pub const ENCODED_H: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABAAAAABA",
];

pub const ENCODED_I: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"ABBBBBBBA",
];

pub const ENCODED_J: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAAAABAA",
    b"AAAAAABAA",
    b"ABAAAABAA",
    b"ABBBBBBAA",
];

pub const ENCODED_K: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"ABAAABBAA",
    b"ABBBBBAAA",
    b"ABAAABBAA",
    b"ABAAAAABA",
];

pub const ENCODED_L: [&'static [u8; 9]; 5] = [
    b"ABAAAAAAA",
    b"ABAAAAAAA",
    b"ABAAAAAAA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
];

pub const ENCODED_M: [&'static [u8; 9]; 5] = [
    b"ABBAAABBA",
    b"ABABABABA",
    b"ABAABAABA",
    b"ABAABAABA",
    b"ABAABAABA",
];

pub const ENCODED_N: [&'static [u8; 9]; 5] = [
    b"ABBAAAABA",
    b"ABABAAABA",
    b"ABAABBABA",
    b"ABAAAABBA",
    b"ABAAAAABA",
];

pub const ENCODED_O: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
];

pub const ENCODED_P: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABAAAAAAA",
];

pub const ENCODED_Q: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABAAABABA",
    b"ABBBBABAA",
];

pub const ENCODED_R: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAABAA",
    b"ABAAAAABA",
];

pub const ENCODED_S: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"AABBBBBAA",
    b"AAAAAAABA",
    b"ABBBBBBBA",
];

pub const ENCODED_T: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"AAAABAAAA",
];

pub const ENCODED_U: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
];

pub const ENCODED_V: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"ABAAAAABA",
    b"AABAAABAA",
    b"AAABBBAAA",
];

pub const ENCODED_W: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"ABAABAABA",
    b"ABAABAABA",
    b"ABABBBABA",
    b"ABBAAABBA",
];

pub const ENCODED_X: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"AABAAABAA",
    b"AAABBBAAA",
    b"AABAAABAA",
    b"ABAAAAABA",
];

pub const ENCODED_Y: [&'static [u8; 9]; 5] = [
    b"ABAAAAABA",
    b"AABAAABAA",
    b"AAABBBAAA",
    b"AAAABAAAA",
    b"AAAABAAAA",
];

pub const ENCODED_Z: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAAAABAA",
    b"AAAABBAAA",
    b"AABAAAAAA",
    b"ABBBBBBBA",
];

pub const ENCODED_SPACE: [&'static [u8; 9]; 5] = [
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"AAAAAAAAA",
];

pub const encodedExclamation: [&'static [u8; 9]; 5] = [
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"AAAAAAAAA",
    b"AAAABAAAA",
];

pub const encodedQuestion: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAAAAABA",
    b"AAAABBBAA",
    b"AAAAAAAAA",
    b"AAAABAAAA",
];

pub const encodedPeriod: [&'static [u8; 9]; 5] = [
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"AAAABAAAA",
];

pub const encodedDash: [&'static [u8; 9]; 5] = [
    b"AAAAAAAAA",
    b"AAAAAAAAA",
    b"ABBBBBBBA",
    b"AAAAAAAAA",
    b"AAAAAAAAA",
];

pub const encodedEquals: [&'static [u8; 9]; 5] = [
    b"AAAAAAAAA",
    b"ABBBBBBBA",
    b"AAAAAAAAA",
    b"ABBBBBBBA",
    b"AAAAAAAAA",
];

pub const encodedPlus: [&'static [u8; 9]; 5] = [
    b"AAAABAAAA",
    b"AAAABAAAA",
    b"ABBBBBBBA",
    b"AAAABAAAA",
    b"AAAABAAAA",
];

pub const encoded1: [&'static [u8; 9]; 5] = [
    b"AAAABBAAA",
    b"AAABABAAA",
    b"AAAAABAAA",
    b"AAAAABAAA",
    b"AABBBBBBA",
];

pub const encoded2: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
];

pub const encoded3: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAAAAABA",
    b"AABBBBBAA",
    b"AAAAAAABA",
    b"ABBBBBBAA",
];

pub const encoded4: [&'static [u8; 9]; 5] = [
    b"ABAAAABAA",
    b"ABAAAABAA",
    b"ABBBBBBBA",
    b"AAAAAABAA",
    b"AAAAAABAA",
];

pub const encoded5: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
    b"AAAAAAABA",
    b"ABBBBBBBA",
];

pub const encoded6: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAAAA",
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
];

pub const encoded7: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"AAAAAABAA",
    b"AAAAABAAA",
    b"AAAABAAAA",
    b"AAABAAAAA",
];

pub const encoded8: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
];

pub const ENCODED9: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAAABA",
    b"ABBBBBBBA",
    b"AAAAAAABA",
    b"ABBBBBBBA",
];

pub const ENCODED0: [&'static [u8; 9]; 5] = [
    b"ABBBBBBBA",
    b"ABAAAABBA",
    b"ABAABAABA",
    b"ABBAAAABA",
    b"ABBBBBBBA",
];
