use memmap::{MmapMut, MmapOptions};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::str;

// reference
// https://github.com/joyent/statemap/blob/81b7c44c2edb9b9036324e152b1a7fde945b4e82/src/statemap.rs

#[derive(Deserialize, Debug)]
struct StatemapInputDatum {
    time: String,   // time of this datum
    entity: String, // name of entity
    state: u32,     // state entity is in at time
}

fn main() {
    let file = File::open("input.json").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let mut contents = str::from_utf8(&mmap[..]).unwrap();
    let len = contents.len();
    println!("lenght {}", len);
    match try_parse::<StatemapInputDatum>(&mut contents) {
        Ok(None) => println!("end of the line"),
        Ok(Some(datum)) => println!("{:?}", datum),
        Err(err) => {
            println!("{}", err);
        }
    };

    match try_parse::<StatemapInputDatum>(&mut contents) {
        //        Ok(None) => Ok("no"),
        Ok(Some(datum)) => println!("{:?}", datum),
        Ok(None) => println!("end of the line"),
        Err(err) => {
            println!("{}", err);
        }
    };
    println!("done");
}

fn try_parse<'de, T>(content: &mut &'de str) -> Result<Option<T>, serde_json::Error>
where
    T: serde::Deserialize<'de>,
{
    let mut de = serde_json::Deserializer::from_str(*content).into_iter();
    match de.next() {
        Some(Ok(value)) => {
            *content = &content[de.byte_offset()..];
            Ok(Some(value))
        }
        Some(Err(err)) => Err(err),
        None => Ok(None),
    }
}
