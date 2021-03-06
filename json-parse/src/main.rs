use memmap::MmapOptions;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::str;
use std::str::FromStr;

// reference
// https://github.com/joyent/statemap/blob/81b7c44c2edb9b9036324e152b1a7fde945b4e82/src/statemap.rs

#[derive(Deserialize, Debug)]
struct StatemapInputDatum {
    #[serde(deserialize_with = "datum_time_from_string")]
    time: u64, // time of this datum
    entity: String, // name of entity
    state: u32,     // state entity is in at time
}

fn main() {
    let file = File::open("file.json").unwrap();
    //    https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#access-a-file-randomly-using-a-memory-map
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let mut contents = str::from_utf8(&mmap[..]).unwrap();
    loop {
        match try_parse::<StatemapInputDatum>(&mut contents) {
            Ok(None) => {
                println!("end of the line");
                break;
            }
            Ok(Some(datum)) => {
                let time = datum.time;
                ()
            }
            Err(err) => {
                println!("{}", err);
                break;
            }
        };
    }
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

/*
 * The time value is written in the input as a JSON string containing a number.
 * Deserialize just the number here without allocating memory for a String.
 */
fn datum_time_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    match u64::from_str(s) {
        Ok(time) => Ok(time),
        Err(_) => Err(serde::de::Error::custom("illegal time value")),
    }
}
