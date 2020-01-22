use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() {
    let mut output = File::create("file.json").unwrap();
//    let file = OpenOptions::new()
//    .write(true)
//    .open("new_file.txt").unwrap();

    for i in 0..30000000 {
        let time = 20000000000u64 + i;
        let entity = 50000 + i % 1000;
        let state = i % 3;
        write!(output, r#"{{ "time": "{}", "entity": "{}", "state": {} }}"#, time, entity, state);
    }
}
