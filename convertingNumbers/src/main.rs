use std::mem::size_of;
use std::convert::TryInto;

fn main() {
    let column_width = 10;

    println!(
        "{0: <4} | {1: <4$} | {2: <4$} | {3: <4$}",
        "size", "verb", "big-endian", "little-endian", column_width
    );
    for size in vec![size_of::<u16>(), size_of::<u32>()] {
        for item in vec![
            "GET", "HEAD", "POST", "PUT", "DELETE", "OPTIONS", "TRACE", "PATCH",
        ] {
            let mut trunc = String::from(item);
            // pad strings with spaces
            trunc = format!("{1: <0$}", 4, trunc);
            trunc.truncate(size);
            let be_string: String;
            let le_string: String;

            if size == 4 {
                let array:[u8;4] = trunc.as_bytes()[..4].try_into().unwrap();
                be_string = u32::from_be_bytes(array).to_string();
                le_string = u32::from_le_bytes(array).to_string();
            } else if size == 2 {
                let array:[u8;2] = trunc.as_bytes()[..2].try_into().unwrap();
                be_string = u16::from_be_bytes(array).to_string();
                le_string = u16::from_le_bytes(array).to_string();
            } else {
                unimplemented!();
            }

            println!(
                "{0: <4} | {1: <4$} | {2: <4$} | {3: <4$}",
                size,
                item,
                be_string,
                le_string,
                column_width
            );
        }
    }
}
